use clang::{Entity, EntityKind};
use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::LitStr;

use crate::parsing::{translate_ffi_type, translate_var, ArrayLen, VmaVarKind};

/// Generates rust bindings for every Vma struct.
///
/// Also generates corresponding builder structs.
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

/// Generates rust code for a parsed Vma struct
fn generate_struct(item: &VmaStruct) -> TokenStream {
    let name = &item.name;

    // convert doc string to #[doc = "..."] attribute
    let docs = item.docs.as_ref().map(|docs| quote! { #[doc = #docs] });

    // generate code for each field
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
    let getters = generate_getters(item);

    let lifetime = needs_lifetime(item).then(|| quote! { <'a> });
    let phantom = needs_lifetime(item).then(|| quote! { _p: ::std::marker::PhantomData<&'a ()>, });

    quote! {
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        #docs
        pub struct #name #lifetime {
            #(#fields,)*
            #phantom
        }

        impl #lifetime Default for #name #lifetime {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }

        #getters

        #builder
    }
}

fn needs_lifetime(item: &VmaStruct) -> bool {
    item.fields.iter().any(|field| match &field.kind {
        VmaVarKind::Normal => field.needs_lifetime,
        VmaVarKind::Len => false,
        VmaVarKind::Ref(_) => true,
        VmaVarKind::RefMut(_) => true,
        VmaVarKind::Array(_, _) => true,
        VmaVarKind::ArrayMut(_, _) => true,
        VmaVarKind::ConstantArray(_, _) => false,
        VmaVarKind::Str => true,
        VmaVarKind::PNext(_, _) => true,
        _ => panic!("Unexpected struct field kind"),
    })
}

/// Generates getters for certain special fields
/// - for array fields, generates getters returning a slice
/// - for string fields, generates getters returning a &CStr
fn generate_getters(item: &VmaStruct) -> TokenStream {
    let name = &item.name;

    let lifetime = needs_lifetime(item).then(|| quote! { <'a> });

    let getters = item
        .fields
        .iter()
        .filter_map(|field| match &field.kind {
            VmaVarKind::ArrayMut(element_ty, ArrayLen::Adjacent(len_field)) => {
                let field_name = &field.name;
                let getter_name =
                    format_ident!("get_{}", field_name.to_string().trim_start_matches("p_"));
                let getter_name_mut =
                    format_ident!("get_{}_mut", field_name.to_string().trim_start_matches("p_"));
                Some(quote! {
                    pub fn #getter_name(&self) -> &[#element_ty] {
                        unsafe { ::std::slice::from_raw_parts(self.#field_name, self.#len_field as _) }
                    }
                    pub fn #getter_name_mut(&mut self) -> &mut [#element_ty] {
                        unsafe { ::std::slice::from_raw_parts_mut(self.#field_name, self.#len_field as _) }
                    }
                })
            }
            VmaVarKind::Str => {
                let field_name = &field.name;
                let getter_name =
                    format_ident!("get_{}", field_name.to_string().trim_start_matches("p_"));
                Some(quote! {
                    pub fn #getter_name(&self) -> Option<&::std::ffi::CStr> {
                        if !self.#field_name.is_null() { Some(unsafe { ::std::ffi::CStr::from_ptr(self.#field_name) }) } else { None }
                    }
                })
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    if !getters.is_empty() {
        quote! {
            impl #lifetime #name #lifetime {
                #(#getters)*
            }
        }
    } else {
        quote! {}
    }
}

/// Generates a builder struct for a parsed Vma struct
fn generate_builder(item: &VmaStruct) -> TokenStream {
    let name = &item.name;

    let lifetime = needs_lifetime(item).then(|| quote! { <'a> });

    let functions = item.fields.iter().filter_map(|field| {
        // remove ugly prefixes for builder function name
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
            VmaVarKind::Normal => {
                let name = &field.name;
                let ty = &field.ty;
                Some(quote! {
                    pub fn #func_name(mut self, #name: #ty) -> Self {
                        self.#name = #name;
                        self
                    }
                })
            }
            VmaVarKind::PNext(trait_name, is_const) => {
                let name = &field.name;
                let mutability = (!is_const).then(|| quote! {mut});
                let ptr_spec = if *is_const {
                    quote! {const}
                } else {
                    quote! {mut}
                };
                let null_func = if *is_const {
                    quote! { null }
                } else {
                    quote! { null_mut }
                };

                Some(quote! {
                    pub fn #func_name(mut self, #name: Option<&'a #mutability impl #trait_name>) -> Self {
                        self.#name = #name.map_or(::std::ptr::#null_func(), |p| p as *#ptr_spec _ as *#ptr_spec _);
                        self
                    }
                })
            }
            VmaVarKind::Len => None, // Length fields are set when the corresponding array field is set
            VmaVarKind::Ref(ty) => {
                let name = &field.name;
                Some(quote! {
                    pub fn #func_name(mut self, #name: &'a #ty) -> Self {
                        self.#name = #name;
                        self
                    }
                })
            }
            VmaVarKind::RefMut(ty) => {
                let name = &field.name;
                Some(quote! {
                    pub fn #func_name(mut self, #name: &'a mut #ty) -> Self {
                        self.#name = #name;
                        self
                    }
                })
            }
            VmaVarKind::Array(ty, ArrayLen::Adjacent(len)) => {
                // Array fields with length specified by an adjacent struct field will
                // also set the corresponding length field
                let name = &field.name;
                Some(quote! {
                    pub fn #func_name(mut self, #name: &'a [#ty]) -> Self {
                        self.#name = #name.as_ptr();
                        self.#len = #name.len() as _;
                        self
                    }
                })
            }
            VmaVarKind::Array(ty, _) => {
                // Array fields with length specified by external variables
                // are just accepted without checking the length
                let name = &field.name;
                Some(quote! {
                    pub fn #func_name(mut self, #name: &'a [#ty]) -> Self {
                        self.#name = #name.as_ptr();
                        self
                    }
                })
            }
            VmaVarKind::ArrayMut(ty, ArrayLen::Adjacent(len)) => {
                // Array fields with length specified by an adjacent struct field will
                // also set the corresponding length field
                let name = &field.name;
                Some(quote! {
                    pub fn #func_name(mut self, #name: &'a mut [#ty]) -> Self {
                        self.#name = #name.as_mut_ptr();
                        self.#len = #name.len() as _;
                        self
                    }
                })
            }
            VmaVarKind::ArrayMut(ty, _) => {
                // Array fields with length specified by external variables
                // are just accepted without checking the length
                let name = &field.name;
                Some(quote! {
                    pub fn #func_name(mut self, #name: &'a mut [#ty]) -> Self {
                        self.#name = #name.as_mut_ptr();
                        self
                    }
                })
            }
            VmaVarKind::ConstantArray(ty, len) => {
                let name = &field.name;
                Some(quote! {
                    pub fn #func_name(mut self, #name: [#ty; #len]) -> Self {
                        self.#name = #name;
                        self
                    }
                })
            }
            VmaVarKind::Str => {
                // String fields take a &CStr and convert it to a pointer for convenience
                let name = &field.name;
                Some(quote! {
                    pub fn #func_name(mut self, #name: Option<&'a ::std::ffi::CStr>) -> Self {
                        self.#name = #name.map_or(::std::ptr::null(), |s| s.as_ptr());
                        self
                    }
                })
            }
            _ => panic!("Unexpected struct field kind"),
        }
    });

    quote! {
        impl #lifetime #name #lifetime {
            #(#functions)*
        }
    }
}

/// parses the libclang definition of a Vma struct.
fn parse_struct(item: &Entity) -> VmaStruct {
    // remove "Vma" prefix
    let name = format_ident!("{}", item.get_name().unwrap().trim_start_matches("Vma"));
    // parse doc string
    let docs = item
        .get_comment()
        .map(|comment| {
            comment
                .trim_start_matches('/')
                .trim_start_matches('*')
                .trim_end_matches('/')
                .trim_end_matches('*')
                .trim()
                .replace('\r', "")
                .replace("    ", " ")
        })
        .map(|comment| syn::parse2::<LitStr>(quote! {#comment}).unwrap());

    // parse all fields
    let mut fields = item
        .get_children()
        .iter()
        .filter(|child| child.get_kind() == EntityKind::FieldDecl)
        .map(parse_field)
        .collect::<Vec<_>>();

    // detect which struct fields are length fields and mark them accordingly
    let len_fields = fields
        .iter()
        .filter_map(|field| match &field.kind {
            VmaVarKind::Array(_, ArrayLen::Adjacent(len))
            | VmaVarKind::ArrayMut(_, ArrayLen::Adjacent(len)) => Some(len.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();

    for field in &mut fields {
        if len_fields.contains(&field.name) {
            field.kind = VmaVarKind::Len;
        }
    }

    VmaStruct { name, docs, fields }
}

/// Parses the libclang definition of a single struct field.
fn parse_field(field: &Entity) -> VmaStructField {
    // convert field name to snake_case
    let name = format_ident!("{}", field.get_name().unwrap().to_case(Case::Snake));
    // parse doc string
    let docs = field
        .get_comment()
        .map(|comment| {
            comment
                .trim_start_matches('/')
                .trim_start_matches('*')
                .trim_end_matches('/')
                .trim_end_matches('*')
                .trim()
                .replace('\r', "")
                .replace("    ", " ")
        })
        .map(|comment| syn::parse2::<LitStr>(quote! {#comment}).unwrap());

    // Try to find a VMA_LEN_IF_NOT_NULL(...) attribute, which describes
    // the required length for a given array pointer.
    // The generator uses #define VMA_LEN_IF_NOT_NULL(len) __attribute__((annotate("LEN:"#len)))
    // which means we have to look for a AnnotateAttr that starts with "LEN:"
    let len_attr = field
        .get_children()
        .iter()
        .find(|child| {
            child.get_kind() == EntityKind::AnnotateAttr
                && child.get_display_name().unwrap().starts_with("LEN:")
        })
        .map(|attr| {
            attr.get_display_name()
                .unwrap()
                .trim_start_matches("LEN:")
                .to_string()
        });
    let extends_attr = field
        .get_children()
        .iter()
        .find(|child| {
            child.get_kind() == EntityKind::AnnotateAttr
                && child.get_display_name().unwrap().starts_with("VK_STRUCT:")
        })
        .map(|attr| {
            attr.get_display_name()
                .unwrap()
                .trim_start_matches("VK_STRUCT:")
                .to_string()
        });

    let field_type = field.get_type().unwrap();
    let (ty, needs_lifetime) = translate_ffi_type(&field_type);
    let kind = translate_var(&field_type, len_attr, extends_attr);

    VmaStructField {
        name,
        docs,
        ty,
        kind,
        needs_lifetime,
    }
}

/// Description of a Vma struct
struct VmaStruct {
    /// rust name of the struct
    name: Ident,
    /// documentation string
    docs: Option<LitStr>,
    /// list of struct fields
    fields: Vec<VmaStructField>,
}

/// Description of a single struct field
struct VmaStructField {
    /// rust name of the field
    name: Ident,
    /// documentation string
    docs: Option<LitStr>,
    /// ffi type to use for this field
    ty: syn::Type,
    /// semantic usage of the field
    kind: VmaVarKind,
    needs_lifetime: bool,
}
