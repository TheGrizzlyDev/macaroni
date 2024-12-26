use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, ItemFn, ReturnType, Type, TypePath,
};


/// Interposes a given function and captures native bindings
///
/// Usage:
/// ```ignore
/// #[interpose]
/// fn my_func(arg1: i32, arg2: i32) -> LibcResult<i32> {
///     // user code here
/// }
/// ```
///
/// This will expand into two modules:
/// - `original`: an extern "C" declaration for `my_func`
/// - `replacement`: a wrapper function called `my_func` that interprets LibcResult
#[proc_macro_attribute]
pub fn interpose(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_vis  = &input_fn.vis;
    let fn_block = &input_fn.block;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_generics = &input_fn.sig.generics;
    let fn_output = &input_fn.sig.output;

    let ret_ty = match fn_output {
        ReturnType::Default => {
            return syn::Error::new_spanned(fn_output, "Function must return LibcResult<T>")
                .to_compile_error().into();
        }
        ReturnType::Type(_rarrow, ty) => ty,
    };

    let link_name = format!("{fn_name}");

    let inner_ret_ty: Type = match &**ret_ty {
        Type::Path(TypePath { path, .. }) => {
            let segments = &path.segments;
            if segments.len() == 1 && segments[0].ident == "LibcResult" {
                if let syn::PathArguments::AngleBracketed(ref generic_args) = segments[0].arguments {
                    if generic_args.args.len() == 1 {
                        let first_arg = &generic_args.args[0];
                        if let syn::GenericArgument::Type(ty) = first_arg {
                            ty.clone()
                        } else {
                            return syn::Error::new_spanned(
                                first_arg,
                                "Expected a type parameter in LibcResult",
                            ).to_compile_error().into();
                        }
                    } else {
                        return syn::Error::new_spanned(
                            generic_args,
                            "Expected exactly one type parameter in LibcResult",
                        )
                        .to_compile_error()
                        .into();
                    }
                } else {
                    return syn::Error::new_spanned(
                        path,
                        "Expected angle bracketed args, e.g. LibcResult<T>",
                    )
                    .to_compile_error()
                    .into();
                }
            } else {
                return syn::Error::new_spanned(
                    path,
                    "Expected return type LibcResult<T>",
                )
                .to_compile_error()
                .into();
            }
        }
        _ => {
            return syn::Error::new_spanned(ret_ty, "Expected LibcResult<T>")
                .to_compile_error()
                .into();
        }
    };

    let generated = quote! {
        pub mod #fn_name {
            use libc;
            use core::mem;
            use super::*;
            use libc_interposition_lib::{InterposeEntry as _InternalInterposeEntry, LibcResult as _InternalLibcResult};

            extern "C" {
                #[link_name = #link_name]
                pub fn original #fn_generics (#fn_inputs) -> #inner_ret_ty;
            }

            #[unsafe(export_name = #link_name)]
            #[link_section = "__TEXT,__macaroni"]
            #[allow(non_snake_case)]
            #fn_vis fn replacement #fn_generics (#fn_inputs) -> #inner_ret_ty {
                let inner = || -> _InternalLibcResult<#inner_ret_ty> {
                    #fn_block
                };

                match inner() {
                    _InternalLibcResult::Ok(value) => value,
                    _InternalLibcResult::Err(errno_val) => {
                        unsafe {
                            *libc::__error() = errno_val;
                        }
                        unsafe { mem::zeroed() }
                    },
                    _InternalLibcResult::ErrAndReturn(value, errno_val) => {
                        unsafe {
                            *libc::__error() = errno_val;
                        }
                        value
                    },
                    _InternalLibcResult::ReturnErr(errno_val) => {
                        unsafe {
                            *libc::__error() = errno_val;
                        }
                        errno_val as _
                    }
                }
            }

            pub static INTERPOSE_ENTRY: _InternalInterposeEntry = _InternalInterposeEntry {
                replacement: replacement as *const (),
                original: original as *const (),
            };
        }
    };

    generated.into()
}

