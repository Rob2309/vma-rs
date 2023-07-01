use clang::{Entity, EntityKind, Type, TypeKind};
use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::LitStr;

use crate::functions::{parse_array_len, translate_ffi_arg, ArrayLen};

pub fn generate_structs(tu: &Entity) -> TokenStream {
    let mut vma_structs = Vec::new();

    for item in tu.get_children() {
        if item.get_kind() == EntityKind::StructDecl {
            if let Some(name) = item.get_name() {
                if name.starts_with("Vma") && !name.ends_with("_T") {
                    vma_structs.push(parse_struct(&item));
                }
            }
        }
    }

    let generated = vma_structs.iter().map(generate_struct);

    quote! {
        #(#generated)*
    }
}

fn generate_struct(item: &VmaStruct) -> TokenStream {
    let name = &item.name;

    let docs = item.docs.as_ref().map(|docs| quote! { #[doc = #docs] });

    let fields = item.fields.iter().map(|field| {
        let name = &field.name;
        let docs = field.docs.as_ref().map(|docs| quote! { #[doc = #docs] });

        let ty = &field.ty;

        quote! {
            #docs
            pub #name: #ty
        }
    });

    let builder = generate_builder(item);

    quote! {
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        #docs
        pub struct #name {
            #(#fields),*
        }

        impl Default for #name {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }

        #builder
    }
}

fn generate_builder(item: &VmaStruct) -> TokenStream {
    let name = &item.name;
    let builder_name = format_ident!("{name}Builder");

    let functions = item.fields.iter().filter_map(|field| {
        let func_name = format_ident!(
            "{}",
            field
                .name
                .to_string()
                .trim_start_matches("p_")
                .trim_start_matches("pp_")
                .trim_start_matches("pfn_")
                .trim_start_matches("vk_")
        );
        match &field.kind {
            VmaStructFieldKind::Normal => {
                let name = &field.name;
                let ty = &field.ty;
                Some(quote! {
                    pub fn #func_name(mut self, #name: #ty) -> Self {
                        self.inner.#name = #name;
                        self
                    }
                })
            }
            VmaStructFieldKind::Len => None,
            VmaStructFieldKind::Ref(ty) => {
                let name = &field.name;
                Some(quote! {
                    pub fn #func_name(mut self, #name: &'a #ty) -> Self {
                        self.inner.#name = #name;
                        self
                    }
                })
            }
            VmaStructFieldKind::RefMut(ty) => {
                let name = &field.name;
                Some(quote! {
                    pub fn #func_name(mut self, #name: &'a mut #ty) -> Self {
                        self.inner.#name = #name;
                        self
                    }
                })
            }
            VmaStructFieldKind::ArrayRef(ty, ArrayLen::Adjacent(len)) => {
                let name = &field.name;
                Some(quote! {
                    pub fn #func_name(mut self, #name: &'a [#ty]) -> Self {
                        self.inner.#name = #name.as_ptr();
                        self.inner.#len = #name.len() as _;
                        self
                    }
                })
            }
            VmaStructFieldKind::ArrayRef(ty, _) => {
                let name = &field.name;
                Some(quote! {
                    pub fn #func_name(mut self, #name: &'a [#ty]) -> Self {
                        self.inner.#name = #name.as_ptr();
                        self
                    }
                })
            }
            VmaStructFieldKind::ArrayRefMut(ty, ArrayLen::Adjacent(len)) => {
                let name = &field.name;
                Some(quote! {
                    pub fn #func_name(mut self, #name: &'a mut [#ty]) -> Self {
                        self.#name = #name.as_mut_ptr();
                        self.inner.#len = #name.len() as _;
                        self
                    }
                })
            }
            VmaStructFieldKind::ArrayRefMut(ty, _) => {
                let name = &field.name;
                Some(quote! {
                    pub fn #func_name(mut self, #name: &'a mut [#ty]) -> Self {
                        self.inner.#name = #name.as_mut_ptr();
                        self
                    }
                })
            }
            VmaStructFieldKind::Array(ty, len) => {
                let name = &field.name;
                Some(quote! {
                    pub fn #func_name(mut self, #name: [#ty; #len]) -> Self {
                        self.inner.#name = #name;
                        self
                    }
                })
            }
        }
    });

    quote! {
        impl #name {
            pub fn builder<'a>() -> #builder_name<'a> {
                #builder_name::default()
            }
        }

        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, Default)]
        pub struct #builder_name<'a> {
            inner: #name,
            _p: ::std::marker::PhantomData<&'a ()>,
        }

        impl<'a> #builder_name<'a> {
            #(#functions)*
        }

        impl<'a> ::std::ops::Deref for #builder_name<'a> {
            type Target = #name;
            fn deref(&self) -> &#name {
                &self.inner
            }
        }
        impl<'a> ::std::ops::DerefMut for #builder_name<'a> {
            fn deref_mut(&mut self) -> &mut #name {
                &mut self.inner
            }
        }
    }
}

fn parse_struct(item: &Entity) -> VmaStruct {
    let name = format_ident!("{}", item.get_name().unwrap().trim_start_matches("Vma"));
    let docs = item
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

    let mut fields = item
        .get_children()
        .iter()
        .filter(|child| child.get_kind() == EntityKind::FieldDecl)
        .map(parse_field)
        .collect::<Vec<_>>();

    let len_fields = fields
        .iter()
        .filter_map(|field| match &field.kind {
            VmaStructFieldKind::ArrayRef(_, ArrayLen::Adjacent(len))
            | VmaStructFieldKind::ArrayRefMut(_, ArrayLen::Adjacent(len)) => Some(len.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();

    for field in &mut fields {
        if len_fields.contains(&field.name) {
            field.kind = VmaStructFieldKind::Len;
        }
    }

    VmaStruct { name, docs, fields }
}

fn parse_field(field: &Entity) -> VmaStructField {
    let name = format_ident!("{}", field.get_name().unwrap().to_case(Case::Snake));
    let docs = field
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

    let len_attr = field
        .get_children()
        .iter()
        .find(|child| child.get_kind() == EntityKind::AnnotateAttr)
        .map(|attr| attr.get_display_name().unwrap());

    let field_type = field.get_type().unwrap();
    let ty = translate_ffi_arg(&field_type);
    let kind = parse_field_kind(&field_type, len_attr);

    VmaStructField {
        name,
        docs,
        ty,
        kind,
    }
}

fn parse_field_kind(ty: &Type, len_attr: Option<String>) -> VmaStructFieldKind {
    match ty.get_kind() {
        TypeKind::Typedef | TypeKind::Float => VmaStructFieldKind::Normal,
        TypeKind::Pointer => {
            let pointee = ty.get_pointee_type().unwrap();
            let converted_pointee = translate_ffi_arg(&pointee);
            let is_const = pointee.is_const_qualified();

            let len = len_attr.map(|l| parse_array_len(&l));

            match (is_const, pointee.get_kind(), len) {
                (false, TypeKind::Void, _) => VmaStructFieldKind::Normal, // exception for *mut c_void
                (false, _, Some(len)) => VmaStructFieldKind::ArrayRefMut(converted_pointee, len),
                (false, _, None) => VmaStructFieldKind::RefMut(converted_pointee),
                (true, TypeKind::CharS | TypeKind::CharU, _) => VmaStructFieldKind::Normal, // exception for *const c_char
                (true, _, Some(len)) => VmaStructFieldKind::ArrayRef(converted_pointee, len),
                (true, _, None) => VmaStructFieldKind::Ref(converted_pointee),
            }
        }
        TypeKind::ConstantArray => {
            let content = ty.get_element_type().unwrap();
            let converted = translate_ffi_arg(&content);

            let len = ty.get_size().unwrap();

            VmaStructFieldKind::Array(converted, len)
        }
        _ => panic!("Unsupported field type: {}", ty.get_display_name()),
    }
}

struct VmaStruct {
    name: Ident,
    docs: Option<LitStr>,
    fields: Vec<VmaStructField>,
}

struct VmaStructField {
    name: Ident,
    docs: Option<LitStr>,
    ty: syn::Type,
    kind: VmaStructFieldKind,
}

enum VmaStructFieldKind {
    Normal,
    Len,
    Ref(syn::Type),
    RefMut(syn::Type),
    ArrayRef(syn::Type, ArrayLen),
    ArrayRefMut(syn::Type, ArrayLen),
    Array(syn::Type, usize),
}
