use crate::CodegenSettings;
use crate::config::Asyncness;
use crate::funcs::func_bounds;
use crate::names;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashSet;

pub fn link_module(
    module: &witx::Module,
    target_path: Option<&syn::Path>,
    settings: &CodegenSettings,
) -> TokenStream {
    let module_ident = names::module(&module.name);

    let send_bound = if settings.async_.contains_async(module) {
        quote! { + Send }
    } else {
        quote! {}
    };

    let mut bodies = Vec::new();
    let mut bounds = HashSet::new();
    for f in module.funcs() {
        let asyncness = settings.async_.get(module.name.as_str(), f.name.as_str());
        bodies.push(generate_func(&module, &f, target_path, asyncness));
        let bound = func_bounds(module, &f, settings);
        for b in bound {
            bounds.insert(b);
        }
    }

    let ctx_bound = if let Some(target_path) = target_path {
        let bounds = bounds
            .into_iter()
            .map(|b| quote!(#target_path::#module_ident::#b));
        quote!( #(#bounds)+* #send_bound )
    } else {
        let bounds = bounds.into_iter();
        quote!( #(#bounds)+* #send_bound )
    };

    let func_name = if target_path.is_none() {
        format_ident!("add_to_linker")
    } else {
        format_ident!("add_{}_to_linker", module_ident)
    };

    quote! {
        /// Adds all instance items to the specified `Linker`.
        pub fn #func_name<U>(
            linker: &mut Linker,
            cx_fn: std::rc::Rc<dyn Fn() -> std::rc::Rc<std::cell::RefCell<U>>>,
            mem_fn: std::rc::Rc<dyn Fn() -> wiggle::GuestMemory<'static>>,
        ) -> wiggle::error::Result<()>
            where
                U: #ctx_bound #send_bound + 'static
        {
            use std::ops::DerefMut;

            #(#bodies)*
            Ok(())
        }
    }
}

fn generate_func(
    module: &witx::Module,
    func: &witx::InterfaceFunc,
    target_path: Option<&syn::Path>,
    asyncness: Asyncness,
) -> TokenStream {
    let module_str = module.name.as_str();
    let module_ident = names::module(&module.name);

    let field_str = func.name.as_str();
    let field_ident = names::func(&func.name);

    let (params, results) = func.wasm_signature();

    let arg_names = (0..params.len())
        .map(|i| Ident::new(&format!("arg{i}"), Span::call_site()))
        .collect::<Vec<_>>();
    let arg_tys = params
        .iter()
        .map(|ty| names::wasm_type(*ty))
        .collect::<Vec<_>>();
    let arg_decls = arg_names
        .iter()
        .zip(arg_tys.iter())
        .enumerate()
        .map(|(i, (name, ty))| {
            let i = i as u32;
            quote! {
                let #name = #ty::try_from_js_value(args.get(#i)).unwrap();
            }
        })
        .collect::<Vec<_>>();

    let ret_ty = match results.len() {
        0 => quote!(()),
        1 => names::wasm_type(results[0]),
        _ => unimplemented!(),
    };

    let await_ = if asyncness.is_sync() {
        quote!()
    } else {
        quote!(.await)
    };

    let abi_func = if let Some(target_path) = target_path {
        quote!( #target_path::#module_ident::#field_ident )
    } else {
        quote!( #field_ident )
    };

    let body = quote! {
        let mut mem = mem_fn_arc();
        let ctx = cx_fn2();
        let mut ctx = ctx.borrow_mut();
        <#ret_ty>::from(#abi_func(ctx.deref_mut(), &mut mem #(, #arg_names)*) #await_ .unwrap())
    };

    match asyncness {
        Asyncness::Async => {
            todo!();
            // let arg_decls = quote! { ( #(#arg_names,)* ) : ( #(#arg_tys,)* ) };
            // quote! {
            //     linker.func_wrap_async(
            //         #module_str,
            //         #field_str,
            //         move |mut caller: wiggle::wasmtime_crate::Caller<'_, T>, #arg_decls| {
            //             Box::new(async move { #body })
            //         },
            //     )?;
            // }
        }

        Asyncness::Blocking { block_with } => {
            quote! {
                let cx_fn2 = cx_fn.clone();
                let mem_fn_arc = mem_fn.clone();
                let closure = wasm_bindgen::closure::Closure::<dyn FnMut(js_sys::Array<wasm_bindgen::JsValue>) -> #ret_ty>::new(move |args: js_sys::Array<wasm_bindgen::JsValue>| -> #ret_ty {
                    #(#arg_decls)*
                    let result = async { #body };
                    #block_with(result).unwrap()
                });
                linker.add_import(
                    #module_str,
                    #field_str,
                    &to_vararg_closure(&closure.into_js_value()), // Leaks!
                );
            }
        }

        Asyncness::Sync => {
            quote! {
                let cx_fn2 = cx_fn.clone();
                let mem_fn_arc = mem_fn.clone();
                let closure = wasm_bindgen::closure::Closure::<dyn FnMut(js_sys::Array<wasm_bindgen::JsValue>) -> #ret_ty>::new(move |args: js_sys::Array<wasm_bindgen::JsValue>| -> #ret_ty {
                    #(#arg_decls)*
                    #body
                });
                linker.add_import(
                    #module_str,
                    #field_str,
                    &to_vararg_closure(&closure.into_js_value()), // Leaks!
                );
            }
        }
    }
}
