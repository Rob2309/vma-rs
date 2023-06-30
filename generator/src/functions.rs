use clang::{Entity, EntityKind, Type, TypeKind};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, LitStr};

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

fn generate_func(func: VmaFunction) -> TokenStream {
    let docs = func.docs.map(|doc| quote! {#[doc = #doc]});
    let rs_name = func.rs_name;
    let c_name = func.c_name;

    let rs_args = func.args.iter().filter_map(|arg| {
        let name = &arg.name;

        match &arg.rs_arg_kind {
            VmaFunctionArgKind::Normal => {
                let ty = &arg.ffi_type;
                Some(quote! {#name: #ty})
            }
            VmaFunctionArgKind::Ref(ref_ty) => Some(quote! {#name: &#ref_ty}),
            VmaFunctionArgKind::InArray(array_ty, _) => Some(quote! {#name: &[#array_ty]}),
            _ => None,
        }
    });
    let c_args = func.args.iter().map(|arg| {
        let name = &arg.name;
        let ty = &arg.ffi_type;
        quote! {#name: #ty}
    });

    let return_type = {
        let return_args = func
            .args
            .iter()
            .filter_map(|arg| match &arg.rs_arg_kind {
                VmaFunctionArgKind::Out(ty) => Some(quote! { #ty }),
                VmaFunctionArgKind::OutArray(ty, _) => Some(quote! { Vec<#ty> }),
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
    let c_return_type = if func.returns_result {
        quote! { -> vk::Result }
    } else {
        quote! {}
    };

    let buffers = func.args.iter().filter_map(|arg| {
        let name = &arg.name;
        match &arg.rs_arg_kind {
            VmaFunctionArgKind::Out(_) => Some(quote! {
                let mut #name = ::std::mem::zeroed();
            }),
            VmaFunctionArgKind::OutArray(_, len) => {
                let len_field_name = format_ident!("{name}_len");

                let len_init = match len {
                    ArrayLen::Adjacent(len_field) => {
                        let first_array = &func
                            .args
                            .iter()
                            .find(|arg| {
                                if let VmaFunctionArgKind::InArray(_, ArrayLen::Adjacent(len)) =
                                    &arg.rs_arg_kind
                                {
                                    len == len_field
                                } else {
                                    false
                                }
                            })
                            .unwrap()
                            .name;

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
            _ => None,
        }
    });

    let c_call = {
        let catch_result = if func.returns_result {
            quote! {let result = }
        } else {
            quote! {}
        };

        let pass_args = func.args.iter().map(|arg| {
            let name = &arg.name;
            match &arg.rs_arg_kind {
                VmaFunctionArgKind::Normal => quote! {#name},
                VmaFunctionArgKind::Len => {
                    let first_array = &func
                        .args
                        .iter()
                        .find(|arg| {
                            if let VmaFunctionArgKind::InArray(_, ArrayLen::Adjacent(len)) =
                                &arg.rs_arg_kind
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
                VmaFunctionArgKind::Ref(_) => quote! {#name},
                VmaFunctionArgKind::Out(_) => quote! {&mut #name},
                VmaFunctionArgKind::OutArray(_, _) => quote! {#name.as_mut_ptr()},
                VmaFunctionArgKind::InArray(_, _) => quote! {#name.as_ptr()},
            }
        });

        quote! {
            #catch_result #c_name(#(#pass_args),*);
        }
    };

    let return_statement = {
        let arg_returns = func
            .args
            .iter()
            .filter_map(|arg| match &arg.rs_arg_kind {
                VmaFunctionArgKind::Out(_) | VmaFunctionArgKind::OutArray(_, _) => Some(&arg.name),
                _ => None,
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

fn parse_function(entity: &Entity) -> VmaFunction {
    let name = entity.get_name().unwrap();
    let c_name = syn::parse_str(&name).unwrap();
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
                .to_string()
        })
        .map(|comment| syn::parse2::<LitStr>(quote! {#comment}).unwrap());

    let mut args = entity
        .get_children()
        .iter()
        .filter(|child| child.get_kind() == EntityKind::ParmDecl)
        .map(parse_arg)
        .collect::<Vec<_>>();

    let len_fields = args
        .iter()
        .filter_map(|arg| match &arg.rs_arg_kind {
            VmaFunctionArgKind::OutArray(_, ArrayLen::Adjacent(len_field))
            | VmaFunctionArgKind::InArray(_, ArrayLen::Adjacent(len_field)) => {
                Some(len_field.clone())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    for arg in &mut args {
        if let VmaFunctionArgKind::Normal = &arg.rs_arg_kind {
            if len_fields.contains(&arg.name) {
                arg.rs_arg_kind = VmaFunctionArgKind::Len;
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

fn parse_arg(entity: &Entity) -> VmaFunctionArg {
    let name = syn::parse_str(&entity.get_name().unwrap().to_case(Case::Snake)).unwrap();

    let len = entity
        .get_children()
        .iter()
        .find(|c| c.get_kind() == EntityKind::AnnotateAttr)
        .map(|a| a.get_display_name().unwrap());
    let ty = entity.get_type().unwrap();

    let ffi_type = translate_ffi_arg(&ty);
    let rs_arg_kind = translate_arg(&ty, len);

    VmaFunctionArg {
        name,
        ffi_type,
        rs_arg_kind,
    }
}

fn translate_arg(ty: &Type, len_attr: Option<String>) -> VmaFunctionArgKind {
    match ty.get_kind() {
        TypeKind::Typedef => VmaFunctionArgKind::Normal,
        TypeKind::Pointer => {
            let pointee = ty.get_pointee_type().unwrap();
            let converted_pointee = translate_ffi_arg(&pointee);
            let is_const = ty
                .get_canonical_type()
                .get_pointee_type()
                .unwrap()
                .is_const_qualified(); // weird work around because libclang drops const on const VmaAllocation*

            let len = len_attr.map(|l| parse_array_len(&l));

            match (is_const, pointee.get_kind(), len) {
                (false, TypeKind::Void, _) => VmaFunctionArgKind::Normal, // exception for *mut c_void
                (false, _, Some(len)) => VmaFunctionArgKind::OutArray(converted_pointee, len),
                (false, _, None) => VmaFunctionArgKind::Out(converted_pointee),
                (true, TypeKind::CharS | TypeKind::CharU, _) => VmaFunctionArgKind::Normal, // exception for *const c_char
                (true, _, Some(len)) => VmaFunctionArgKind::InArray(converted_pointee, len),
                (true, _, None) => VmaFunctionArgKind::Ref(converted_pointee),
            }
        }
        _ => panic!("Unsupported arg type: {}", ty.get_display_name()),
    }
}

pub(crate) fn parse_array_len(val: &str) -> ArrayLen {
    match val {
        "\"VkPhysicalDeviceMemoryProperties::memoryHeapCount\"" => ArrayLen::MemoryHeapCount,
        "\"VkPhysicalDeviceMemoryProperties::memoryTypeCount\"" => ArrayLen::MemoryTypeCount,
        _ => ArrayLen::Adjacent(syn::parse_str(&val.to_case(Case::Snake)).unwrap()),
    }
}

#[derive(Clone)]
pub(crate) enum ArrayLen {
    Adjacent(Ident),
    MemoryHeapCount,
    MemoryTypeCount,
}

pub(crate) fn translate_ffi_arg(ty: &Type) -> syn::Type {
    match ty.get_kind() {
        TypeKind::Typedef => convert_typedef(&ty.get_typedef_name().unwrap()),
        TypeKind::Pointer => {
            let pointee = ty.get_pointee_type().unwrap();
            let converted = translate_ffi_arg(&pointee);
            if ty
                .get_canonical_type()
                .get_pointee_type()
                .unwrap()
                .is_const_qualified()
            {
                syn::parse2(quote! {*const #converted}).unwrap()
            } else {
                syn::parse2(quote! {*mut #converted}).unwrap()
            }
        }
        TypeKind::ConstantArray => {
            let content_type = ty.get_element_type().unwrap();
            let converted = translate_ffi_arg(&content_type);

            let len = ty.get_size().unwrap();

            syn::parse2(quote! { [#converted; #len] }).unwrap()
        }
        TypeKind::CharS | TypeKind::CharU => syn::parse2(quote! {::std::ffi::c_char}).unwrap(),
        TypeKind::Void => syn::parse2(quote! {::std::ffi::c_void}).unwrap(),
        TypeKind::Float => syn::parse2(quote! { f32 }).unwrap(),
        _ => panic!("Unsupported arg type: {}", ty.get_display_name()),
    }
}

pub(crate) fn convert_typedef(name: &str) -> syn::Type {
    if let Some(name) = name.strip_prefix("Vk") {
        syn::parse_str(&format!("vk::{name}")).unwrap()
    } else if let Some(name) = name.strip_prefix("PFN_vk") {
        syn::parse_str(&format!(
            "Option<vk::PFN_vk{}>",
            name.trim_end_matches("KHR")
        ))
        .unwrap()
    } else if let Some(name) = name.strip_prefix("Vma") {
        syn::parse_str(name).unwrap()
    } else if name.starts_with("PFN_vma") {
        syn::parse_str(name).unwrap()
    } else {
        let converted = match name {
            "uint32_t" => "u32",
            "size_t" => "usize",
            _ => panic!("Unknown typedef: {name}"),
        };
        syn::parse_str(converted).unwrap()
    }
}

struct VmaFunction {
    c_name: Ident,
    rs_name: Ident,
    docs: Option<LitStr>,
    args: Vec<VmaFunctionArg>,
    returns_result: bool,
}

#[derive(Clone)]
struct VmaFunctionArg {
    name: Ident,
    ffi_type: syn::Type,
    rs_arg_kind: VmaFunctionArgKind,
}

#[derive(Clone)]
enum VmaFunctionArgKind {
    Normal,
    Len,
    Ref(syn::Type),
    Out(syn::Type),
    OutArray(syn::Type, ArrayLen),
    InArray(syn::Type, ArrayLen),
}
