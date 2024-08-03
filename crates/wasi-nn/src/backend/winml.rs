//! Implements a `wasi-nn` [`BackendInner`] using WinML.
//!
//! Note that the [docs.rs] documentation for the `windows` crate does have the
//! right features turned on to read about the functions used; see Microsoft's
//! private documentation instead: [microsoft.github.io/windows-docs-rs].
//!
//! [docs.rs]: https://docs.rs/windows
//! [microsoft.github.io/windows-docs-rs]: https://microsoft.github.io/windows-docs-rs/doc/windows/AI/MachineLearning

use crate::backend::{
    BackendError, BackendExecutionContext, BackendFromDir, BackendGraph, BackendInner, Id,
};
use crate::wit::{ExecutionTarget, GraphEncoding, Tensor, TensorType};
use crate::{ExecutionContext, Graph};
use std::{fs::File, io::Read, mem::size_of, path::Path};
use windows::core::{ComInterface, HSTRING};
use windows::Foundation::Collections::IVectorView;
use windows::Storage::Streams::{
    DataWriter, InMemoryRandomAccessStream, RandomAccessStreamReference,
};
use windows::AI::MachineLearning::{
    ILearningModelFeatureDescriptor, LearningModel, LearningModelBinding, LearningModelDevice,
    LearningModelDeviceKind, LearningModelEvaluationResult, LearningModelSession,
    TensorFeatureDescriptor, TensorFloat,
};

#[derive(Default)]
pub struct WinMLBackend();

impl BackendInner for WinMLBackend {
    fn encoding(&self) -> GraphEncoding {
        GraphEncoding::Onnx
    }

    fn load(&mut self, builders: &[&[u8]], target: ExecutionTarget) -> Result<Graph, BackendError> {
        if builders.len() != 1 {
            return Err(BackendError::InvalidNumberOfBuilders(1, builders.len()).into());
        }

        let model_stream = InMemoryRandomAccessStream::new()?;
        let model_writer = DataWriter::CreateDataWriter(&model_stream)?;
        model_writer.WriteBytes(&builders[0])?;
        model_writer.StoreAsync()?;
        model_writer.FlushAsync()?;
        let model = LearningModel::LoadFromStream(&RandomAccessStreamReference::CreateFromStream(
            &model_stream,
        )?)?;
        let device_kind = match target {
            ExecutionTarget::Cpu => LearningModelDeviceKind::Cpu,
            ExecutionTarget::Gpu => LearningModelDeviceKind::DirectX,
            ExecutionTarget::Tpu => unimplemented!(),
        };
        let graph = WinMLGraph { model, device_kind };

        let box_: Box<dyn BackendGraph> = Box::new(graph);
        Ok(box_.into())
    }

    fn as_dir_loadable(&mut self) -> Option<&mut dyn BackendFromDir> {
        Some(self)
    }
}

impl BackendFromDir for WinMLBackend {
    fn load_from_dir(
        &mut self,
        path: &Path,
        target: ExecutionTarget,
    ) -> Result<Graph, BackendError> {
        let model = read(&path.join("model.onnx"))?;
        self.load(&[&model], target)
    }
}

struct WinMLGraph {
    model: LearningModel,
    device_kind: LearningModelDeviceKind,
}

unsafe impl Send for WinMLGraph {}
unsafe impl Sync for WinMLGraph {}

impl BackendGraph for WinMLGraph {
    fn init_execution_context(&self) -> Result<ExecutionContext, BackendError> {
        let device = LearningModelDevice::Create(self.device_kind.clone())?;
        let session = LearningModelSession::CreateFromModelOnDevice(&self.model, &device)?;
        let box_: Box<dyn BackendExecutionContext> = Box::new(WinMLExecutionContext::new(session));
        Ok(box_.into())
    }
}

struct WinMLExecutionContext {
    session: LearningModelSession,
    binding: LearningModelBinding,
    result: Option<LearningModelEvaluationResult>,
}

impl WinMLExecutionContext {
    fn new(session: LearningModelSession) -> Self {
        Self {
            binding: LearningModelBinding::CreateFromSession(&session).unwrap(),
            session,
            result: None,
        }
    }
}

impl WinMLExecutionContext {
    /// Helper function for finding the internal index of a tensor by [`Id`].
    fn find(
        &self,
        id: Id,
        list: &IVectorView<ILearningModelFeatureDescriptor>,
    ) -> Result<u32, BackendError> {
        let index = match id {
            Id::Index(i) => {
                if i < list.Size()? {
                    i
                } else {
                    return Err(BackendError::BackendAccess(anyhow::anyhow!(
                        "incorrect tensor index: {i} >= {}",
                        list.Size()?
                    )));
                }
            }
            Id::Name(name) => list
                .into_iter()
                .position(|d| d.Name().unwrap() == name)
                .ok_or_else(|| {
                    BackendError::BackendAccess(anyhow::anyhow!("unknown tensor name: {name}"))
                })? as u32,
        };
        Ok(index)
    }
}

impl BackendExecutionContext for WinMLExecutionContext {
    fn set_input(&mut self, id: Id, tensor: &Tensor) -> Result<(), BackendError> {
        let input_features = self.session.Model()?.InputFeatures()?;
        let index = self.find(id, &input_features)?;
        let input = input_features.GetAt(index)?;

        // TODO: Support other tensor types. Only FP32 is supported right now.
        match tensor.ty {
            crate::wit::types::TensorType::Fp32 => {}
            _ => unimplemented!(),
        }

        // TODO: this is quite unsafe and probably incorrect--will the slice
        // still be around by the time the binding is used?!
        let data = unsafe {
            std::slice::from_raw_parts(
                tensor.data.as_ptr() as *const f32,
                tensor.data.len() / size_of::<f32>(),
            )
        };

        self.binding.Bind(
            &input.Name()?,
            &TensorFloat::CreateFromArray(
                &input.cast::<TensorFeatureDescriptor>()?.Shape()?,
                data,
            )?,
        )?;

        Ok(())
    }

    fn compute(&mut self) -> Result<(), BackendError> {
        self.result = Some(self.session.Evaluate(&self.binding, &HSTRING::new())?);
        Ok(())
    }

    fn get_output(&mut self, id: Id) -> Result<Tensor, BackendError> {
        if let Some(result) = &self.result {
            let output_features = self.session.Model()?.OutputFeatures()?;
            let index = self.find(id, &output_features)?;
            let output = output_features.GetAt(index)?;
            // TODO: this only handles FP32!
            let tensor = result
                .Outputs()?
                .Lookup(&output.Name()?)?
                .cast::<TensorFloat>()?;
            let dimensions = dimensions_as_u32(&tensor.Shape()?)?;
            let view = tensor.GetAsVectorView()?;
            let mut data = Vec::with_capacity(view.Size()? as usize * size_of::<f32>());
            for f in view.into_iter() {
                data.extend(f.to_le_bytes());
            }
            Ok(Tensor {
                ty: TensorType::Fp32,
                dimensions,
                data,
            })
        } else {
            return Err(BackendError::BackendAccess(anyhow::Error::msg(
                "Output is not ready.",
            )));
        }
    }
}

/// Read a file into a byte vector.
fn read(path: &Path) -> anyhow::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = vec![];
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

impl From<windows::core::Error> for BackendError {
    fn from(e: windows::core::Error) -> Self {
        BackendError::BackendAccess(anyhow::Error::new(e))
    }
}

fn dimensions_as_u32(dimensions: &IVectorView<i64>) -> Result<Vec<u32>, BackendError> {
    dimensions
        .into_iter()
        .map(|d| if d == -1 { Ok(1) } else { convert_i64(d) })
        .collect()
}

fn convert_i64(i: i64) -> Result<u32, BackendError> {
    u32::try_from(i).map_err(|d| -> BackendError {
        anyhow::anyhow!("unable to convert dimension to u32: {d}").into()
    })
}
