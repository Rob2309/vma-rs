use clang::{Entity, EntityKind, TypeKind};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Ident, LitStr};

use crate::parsing::{translate_ffi_type, translate_var, ArrayLen, VmaVarKind};

/// Generates a rust function for every vma function
pub fn generate_functions(tu: &Entity) -> TokenStream {
    let mut vma_functions = Vec::new();

    for item in tu.get_children() {
        if item.get_kind() == EntityKind::FunctionDecl {
            if let Some(name) = item.get_name() {
                if name.starts_with("vma") {
                    vma_functions.push(parse_function(&item));
                }
            }
        }
    }

    let mut res = quote! {};

    for func in vma_functions {
        res.extend(generate_func(func));
    }

    res
}

/// Generates rust bindings for a single parsed vma function
fn generate_func(func: VmaFunction) -> TokenStream {
    let docs = func.docs.map(|doc| quote! {#[doc = #doc]});
    let rs_name = func.rs_name;
    let c_name = func.c_name;

    // generate high-level argument declarations for the outer rust function.
    let rs_args = func.args.iter().filter_map(|arg| {
        let name = &arg.name;

        match &arg.rs_arg_kind {
            VmaVarKind::Normal => {
                let ty = &arg.ffi_type;
                Some(quote! {#name: #ty})
            }
            VmaVarKind::PNext(trait_name, is_const) => {
                let mutability = (!is_const).then(|| quote! {mut});
                Some(quote! { #name: Option<&#mutability impl #trait_name> })
            }
            VmaVarKind::Ref(ref_ty) => Some(quote! {#name: &#ref_ty}),
            VmaVarKind::Array(array_ty, _) => Some(quote! {#name: &[#array_ty]}),
            VmaVarKind::Str => Some(quote! {#name: Option<&::std::ffi::CStr>}),
            _ => None,
        }
    });
    // generate low-level argument declarations for the inner ffi function.
    let c_args = func.args.iter().map(|arg| {
        let name = &arg.name;
        let ty = &arg.ffi_type;
        quote! {#name: #ty}
    });

    // generate return type declaration for the outer function.
    let return_type = {
        // collect arguments that are used for returning results
        let return_args = func
            .args
            .iter()
            .filter_map(|arg| match &arg.rs_arg_kind {
                VmaVarKind::RefMut(ty) => Some(quote! { #ty }),
                VmaVarKind::ArrayMut(ty, _) => Some(quote! { Vec<#ty> }),
                VmaVarKind::StrMut => Some(quote! { Option<::std::ffi::CString> }),
                _ => None,
            })
            .collect::<Vec<_>>();
        let return_args_tuple = if return_args.is_empty() {
            quote! {()}
        } else if return_args.len() == 1 {
            quote! { #(#return_args)* }
        } else {
            quote! { (#(#return_args),*) }
        };

        // create final return type, either Result<..., vk::Result> or just the return_args
        if func.returns_result {
            quote! { -> Result<#return_args_tuple, vk::Result> }
        } else if !return_args.is_empty() {
            quote! {
                -> #return_args_tuple
            }
        } else {
            quote! {}
        }
    };
    // generate the low-level return type of the inner ffi function.
    // either nothing or "-> vk::Result"
    let c_return_type = if func.returns_result {
        quote! { -> vk::Result }
    } else {
        quote! {}
    };

    // generate the temporary variables that will be passed for the return arguments
    let buffers = func.args.iter().filter_map(|arg| {
        let name = &arg.name;
        match &arg.rs_arg_kind {
            VmaVarKind::RefMut(_) => Some(quote! {
                let mut #name = ::std::mem::zeroed();
            }),
            VmaVarKind::ArrayMut(_, len) => {
                let len_field_name = format_ident!("{name}_len");

                let len_init = match len {
                    ArrayLen::Adjacent(len_field) => {
                        // find the first array argument that uses this length field
                        let first_array = &func
                            .args
                            .iter()
                            .find(|arg| {
                                if let VmaVarKind::Array(_, ArrayLen::Adjacent(len)) =
                                    &arg.rs_arg_kind
                                {
                                    len == len_field
                                } else {
                                    false
                                }
                            })
                            .unwrap()
                            .name;
                        
                        // use this array to deduce length field
                        quote! {let #len_field_name = #first_array.len();}
                    }
                    ArrayLen::MemoryHeapCount => quote! {
                        let #len_field_name = (*get_memory_properties(allocator)).memory_heap_count;
                    },
                    ArrayLen::MemoryTypeCount => quote! {
                        let #len_field_name = (*get_memory_properties(allocator)).memory_type_count;
                    },
                };

                Some(quote! {
                    #len_init
                    let mut #name = vec![::std::mem::zeroed(); #len_field_name as _];
                })
            }
            VmaVarKind::StrMut => Some(quote! { let mut #name = ::std::mem::zeroed(); }),
            _ => None,
        }
    });

    // generate code for passing the high-level arguments to the low-level ffi function
    let c_call = {
        // if the ffi function returns a result, store it
        let catch_result = if func.returns_result {
            quote! {let result = }
        } else {
            quote! {}
        };

        // generate the code that passes the arguments
        let pass_args = func.args.iter().map(|arg| {
            let name = &arg.name;
            match &arg.rs_arg_kind {
                VmaVarKind::PNext(_, false) => quote! { #name.map_or(::std::ptr::null_mut(), |p| p as *mut _ as *mut _) },
                VmaVarKind::PNext(_, true) => quote! { #name.map_or(::std::ptr::null(), |p| p as *const _ as *const _) },
                VmaVarKind::Normal => quote! {#name},
                VmaVarKind::Len => {
                    let first_array = &func
                        .args
                        .iter()
                        .find(|arg| {
                            if let VmaVarKind::Array(_, ArrayLen::Adjacent(len)) = &arg.rs_arg_kind
                            {
                                len == name
                            } else {
                                false
                            }
                        })
                        .unwrap()
                        .name;

                    quote! {
                        #first_array.len() as _
                    }
                }
                VmaVarKind::Ref(_) => {
                    quote! {#name}
                }
                VmaVarKind::RefMut(_) => quote! {&mut #name},
                VmaVarKind::ArrayMut(_, _) => quote! {#name.as_mut_ptr()},
                VmaVarKind::Array(_, _) => quote! {#name.as_ptr()},
                VmaVarKind::Str => quote! { #name.map_or(::std::ptr::null(), |s| s.as_ptr())  },
                VmaVarKind::StrMut => quote! { &mut #name },
                VmaVarKind::ConstantArray(_, _) => panic!("Arrays not supported as arguments"),
            }
        });

        // final ffi function call
        quote! {
            #catch_result #c_name(#(#pass_args),*);
        }
    };

    // generate the final high-level return statement if needed
    let return_statement = {
        // collect all arguments used for returning results
        let arg_returns = func
            .args
            .iter()
            .filter_map(|arg| {
                let name = &arg.name;
                match &arg.rs_arg_kind {
                    VmaVarKind::RefMut(_) | VmaVarKind::ArrayMut(_, _) => {
                        Some(name.to_token_stream())
                    }
                    VmaVarKind::StrMut => {
                        Some(quote! { if !#name.is_null() { Some(::std::ffi::CStr::from_ptr(#name).to_owned()) } else { None } })
                    }
                    _ => None,
                }
            })
            .collect::<Vec<_>>();
        
        let arg_returns_tuple = if arg_returns.len() == 1 {
            quote! { #(#arg_returns)* }
        } else if arg_returns.is_empty() {
            if func.returns_result {
                quote! {()}
            } else {
                quote! {}
            }
        } else {
            quote! { (#(#arg_returns),*) }
        };

        // return either a Result<..., vk::Result> or the return arguments directly
        if func.returns_result {
            quote! {
                if result == vk::Result::SUCCESS {
                    Ok(#arg_returns_tuple)
                } else {
                    Err(result)
                }
            }
        } else {
            arg_returns_tuple
        }
    };

    // final function definition
    quote! {
        #docs
        pub unsafe fn #rs_name(#(#rs_args),*) #return_type {
            extern "C" {
                fn #c_name(#(#c_args),*) #c_return_type;
            }

            #(#buffers)*
            #c_call
            #return_statement
        }
    }
}

/// Parses the libclang definition of a vma function
fn parse_function(entity: &Entity) -> VmaFunction {
    let name = entity.get_name().unwrap();
    let c_name = syn::parse_str(&name).unwrap();
    // remove "vma" prefix and convert to snake_case
    let rs_name = syn::parse_str(&name.trim_start_matches("vma").to_case(Case::Snake)).unwrap();
    let docs = entity
        .get_comment()
        .map(|comment| {
            comment
                .trim_start_matches('/')
                .trim_start_matches('*')
                .trim_end_matches('/')
                .trim_end_matches('*')
                .trim()
                .replace("    ", " ")
        })
        .map(|comment| syn::parse2::<LitStr>(quote! {#comment}).unwrap());

    // parse all parameters
    let mut args = entity
        .get_children()
        .iter()
        .filter(|child| child.get_kind() == EntityKind::ParmDecl)
        .map(parse_arg)
        .collect::<Vec<_>>();

    // check which fields are used to describe array lengths and mark them accordingly
    let len_fields = args
        .iter()
        .filter_map(|arg| match &arg.rs_arg_kind {
            VmaVarKind::ArrayMut(_, ArrayLen::Adjacent(len_field))
            | VmaVarKind::Array(_, ArrayLen::Adjacent(len_field)) => Some(len_field.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    for arg in &mut args {
        if let VmaVarKind::Normal = &arg.rs_arg_kind {
            if len_fields.contains(&arg.name) {
                arg.rs_arg_kind = VmaVarKind::Len;
            }
        }
    }

    let returns_result = entity.get_result_type().unwrap().get_kind() != TypeKind::Void;

    VmaFunction {
        c_name,
        rs_name,
        docs,
        args,
        returns_result,
    }
}

/// Parses the libclang definition of a single vma function parameter
fn parse_arg(entity: &Entity) -> VmaFunctionArg {
    // convert param name to snake_case
    let name = syn::parse_str(&entity.get_name().unwrap().to_case(Case::Snake)).unwrap();

    // Try to find a VMA_LEN_IF_NOT_NULL(...) attribute, which describes
    // the required length for a given array pointer.
    // The generator uses #define VMA_LEN_IF_NOT_NULL(len) __attribute__((annotate("LEN:"#len)))
    // which means we have to look for a AnnotateAttr that starts with "LEN:"
    let len = entity
        .get_children()
        .iter()
        .find(|c| {
            c.get_kind() == EntityKind::AnnotateAttr
                && c.get_display_name().unwrap().starts_with("LEN:")
        })
        .map(|a| {
            a.get_display_name()
                .unwrap()
                .trim_start_matches("LEN:")
                .to_string()
        });
    let extends = entity
        .get_children()
        .iter()
        .find(|c| {
            c.get_kind() == EntityKind::AnnotateAttr
                && c.get_display_name().unwrap().starts_with("VK_STRUCT:")
        })
        .map(|a| {
            a.get_display_name()
                .unwrap()
                .trim_start_matches("VK_STRUCT:")
                .to_string()
        });
    let ty = entity.get_type().unwrap();

    let ffi_type = translate_ffi_type(&ty);
    let rs_arg_kind = translate_var(&ty, len, extends);

    VmaFunctionArg {
        name,
        ffi_type,
        rs_arg_kind,
    }
}

/// Description of a vma function
struct VmaFunction {
    /// name to use for inner ffi function
    c_name: Ident,
    /// name to use for outer high-level function
    rs_name: Ident,
    /// documentation string
    docs: Option<LitStr>,
    /// list of arguments
    args: Vec<VmaFunctionArg>,
    /// true if the ffi function returns a result
    returns_result: bool,
}

/// Description of a vma function parameter
#[derive(Clone)]
struct VmaFunctionArg {
    /// name to use for the parameter
    name: Ident,
    /// type of the parameter in the ffi function
    ffi_type: syn::Type,
    /// semantic type of the parameter in the high-level function
    rs_arg_kind: VmaVarKind,
}
