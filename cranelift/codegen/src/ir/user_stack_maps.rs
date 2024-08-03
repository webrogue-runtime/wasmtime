//! User-defined stack maps.
//!
//! This module provides types allowing users to define stack maps and associate
//! them with safepoints.
//!
//! A **safepoint** is a program point (i.e. CLIF instruction) where it must be
//! safe to run GC. Currently all non-tail call instructions are considered
//! safepoints. (This does *not* allow, for example, skipping safepoints for
//! calls that are statically known not to trigger collections, or to have a
//! safepoint on a volatile load to a page that gets protected when it is time
//! to GC, triggering a fault that pauses the mutator and lets the collector do
//! its work before resuming the mutator. We can lift this restriction in the
//! future, if necessary.)
//!
//! A **stack map** is a description of where to find all the GC-managed values
//! that are live at a particular safepoint. Stack maps let the collector find
//! on-stack roots. Each stack map is logically a set of offsets into the stack
//! frame and the type of value at that associated offset. However, because the
//! stack layout isn't defined until much later in the compiler's pipeline, each
//! stack map entry instead includes both an `ir::StackSlot` and an offset
//! within that slot.
//!
//! These stack maps are **user-defined** in that it is the CLIF producer's
//! responsibility to identify and spill the live GC-managed values and attach
//! the associated stack map entries to each safepoint themselves (see
//! `cranelift_frontend::Function::declare_needs_stack_map` and
//! `cranelift_codegen::ir::DataFlowGraph::append_user_stack_map_entry`). Cranelift
//! will not insert spills and record these stack map entries automatically (in
//! contrast to the old system and its `r64` values).

use crate::ir;
use cranelift_bitset::CompoundBitSet;
use cranelift_entity::PrimaryMap;
use smallvec::SmallVec;

pub(crate) type UserStackMapEntryVec = SmallVec<[UserStackMapEntry; 4]>;

/// A stack map entry describes a single GC-managed value and its location on
/// the stack.
///
/// A stack map entry is associated with a particular instruction, and that
/// instruction must be a safepoint. The GC-managed value must be stored in the
/// described location across this entry's instruction.
#[derive(Clone, Debug, PartialEq, Hash)]
#[cfg_attr(
    feature = "enable-serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct UserStackMapEntry {
    /// The type of the value stored in this stack map entry.
    pub ty: ir::Type,

    /// The stack slot that this stack map entry is within.
    pub slot: ir::StackSlot,

    /// The offset within the stack slot where this entry's value can be found.
    pub offset: u32,
}

/// A compiled stack map, describing the location of many GC-managed values.
///
/// A stack map is associated with a particular instruction, and that
/// instruction is a safepoint.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "enable-serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize)
)]
pub struct UserStackMap {
    by_type: SmallVec<[(ir::Type, CompoundBitSet); 1]>,
}

impl UserStackMap {
    /// Coalesce the given entries into a new `UserStackMap`.
    pub fn new(
        entries: &[UserStackMapEntry],
        stack_slot_offsets: &PrimaryMap<ir::StackSlot, u32>,
    ) -> Self {
        let mut by_type = SmallVec::<[(ir::Type, CompoundBitSet); 1]>::default();

        for entry in entries {
            let offset = stack_slot_offsets[entry.slot] + entry.offset;
            let offset = usize::try_from(offset).unwrap();

            // Don't bother trying to avoid an `O(n)` search here: `n` is
            // basically always one in practice; even if it isn't, there aren't
            // that many different CLIF types.
            let index = by_type
                .iter()
                .position(|(ty, _)| *ty == entry.ty)
                .unwrap_or_else(|| {
                    by_type.push((entry.ty, CompoundBitSet::with_capacity(offset + 1)));
                    by_type.len() - 1
                });

            by_type[index].1.insert(offset);
        }

        UserStackMap { by_type }
    }
}
