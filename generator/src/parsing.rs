use clang::{Type, TypeKind};
use convert_case::{Case, Casing};
use quote::quote;
use syn::Ident;

/// Describes the semantic usage of a variable
#[derive(Clone)]
pub enum VmaVarKind {
    /// Any non-special variable
    Normal,
    /// A variable that describes the length of another variable
    Len,
    /// A `*const` to some type
    Ref(syn::Type),
    /// A `*mut` to some type
    RefMut(syn::Type),
    /// A `*const` to some type with a length
    Array(syn::Type, ArrayLen),
    /// A `*mut` to some type with a length
    ArrayMut(syn::Type, ArrayLen),
    /// A `[T; N]` with a compile time size
    ConstantArray(syn::Type, usize),
    /// A `*const c_char`
    Str,
    /// A `*mut *const c_char`
    StrMut,
    /// A `*const c_void` or `*mut c_void` that extends a Vulkan structure
    PNext(syn::Type, bool),
}

/// Describes how the length of an array field is determined
#[derive(Clone)]
pub enum ArrayLen {
    /// Length is determined by a variable in the same struct or a parameter in the same function
    Adjacent(Ident),
    /// Length is determined by `VkPhysicalDeviceMemoryProperties::memoryHeapCount`
    MemoryHeapCount,
    /// Length is determined by `VkPhysicalDeviceMemoryProperties::memoryTypeCount`
    MemoryTypeCount,
}

impl ArrayLen {
    /// Parses the content of a length attribute
    pub fn parse(val: &str) -> ArrayLen {
        match val {
            "\"VkPhysicalDeviceMemoryProperties::memoryHeapCount\"" => Self::MemoryHeapCount,
            "\"VkPhysicalDeviceMemoryProperties::memoryTypeCount\"" => Self::MemoryTypeCount,
            _ => Self::Adjacent(syn::parse_str(&val.to_case(Case::Snake)).unwrap()),
        }
    }
}

/// Translates the raw type of a variable into a semantic type.
///
/// `len_attr` is the content of a VMA_LEN_IF_NOT_NULL(...) attribute, if present
pub fn translate_var(
    ty: &Type,
    len_attr: Option<String>,
    extends_attr: Option<String>,
) -> VmaVarKind {
    match ty.get_kind() {
        // almost all types used by vma are typedefs, treat them as normal variables
        TypeKind::Typedef => VmaVarKind::Normal,
        // pointers are either references or arrays, depending on the presence of `len_attr`
        TypeKind::Pointer => {
            let pointee = ty.get_pointee_type().unwrap();
            let converted_pointee = translate_ffi_type(&pointee);
            let is_const = ty
                .get_canonical_type()
                .get_pointee_type()
                .unwrap()
                .is_const_qualified(); // weird work around because libclang drops const on const VmaAllocation*

            let len = len_attr.map(|l| ArrayLen::parse(&l));

            match (is_const, pointee.get_kind(), len) {
                (false, TypeKind::Void, _) => {
                    // *mut c_void should not be treated like a reference
                    if let Some(extends_attr) = extends_attr {
                        let trait_name = format!(
                            "vk::Extends{}",
                            extends_attr
                                .trim_start_matches("Vk")
                                .trim_end_matches("KHR")
                        );
                        VmaVarKind::PNext(syn::parse_str(&trait_name).unwrap(), false)
                    } else {
                        VmaVarKind::Normal
                    }
                }
                (false, _, Some(len)) => VmaVarKind::ArrayMut(converted_pointee, len),
                (false, TypeKind::Pointer, None) => {
                    // *mut *const c_char is a mutable string, do not treat like a normal reference
                    let inner_pointee = pointee.get_pointee_type().unwrap();
                    if inner_pointee.is_const_qualified()
                        && matches!(inner_pointee.get_kind(), TypeKind::CharS | TypeKind::CharU)
                    {
                        VmaVarKind::StrMut
                    } else {
                        VmaVarKind::RefMut(converted_pointee)
                    }
                }
                (false, _, None) => VmaVarKind::RefMut(converted_pointee),
                (true, TypeKind::Void, _) => {
                    if let Some(extends_attr) = extends_attr {
                        let trait_name = format!(
                            "vk::Extends{}",
                            extends_attr
                                .trim_start_matches("Vk")
                                .trim_end_matches("KHR")
                        );
                        VmaVarKind::PNext(syn::parse_str(&trait_name).unwrap(), true)
                    } else {
                        panic!("const void* unexpected");
                    }
                }
                (true, TypeKind::CharS | TypeKind::CharU, _) => VmaVarKind::Str, // *const c_char is a string, not a reference
                (true, _, Some(len)) => VmaVarKind::Array(converted_pointee, len),
                (true, _, None) => VmaVarKind::Ref(converted_pointee),
            }
        }
        // Arrays with a fixed size are only possible in structs
        TypeKind::ConstantArray => {
            let content = ty.get_element_type().unwrap();
            let converted = translate_ffi_type(&content);

            let len = ty.get_size().unwrap();

            VmaVarKind::ConstantArray(converted, len)
        }
        TypeKind::Float => VmaVarKind::Normal,
        _ => panic!("Unsupported var type: {}", ty.get_display_name()),
    }
}

/// Translates a libclang type into a rust definition of the same type.
/// For example, converts `const Foo*` into `*const Foo`.
///
/// Also convert raw type names into namespaced ones, e.g. `VkDeviceCreateInfo` into `vk::DeviceCreateInfo`
pub fn translate_ffi_type(ty: &Type) -> syn::Type {
    match ty.get_kind() {
        TypeKind::Typedef => convert_typedef(&ty.get_typedef_name().unwrap()),
        TypeKind::Pointer => {
            let pointee = ty.get_pointee_type().unwrap();
            let converted = translate_ffi_type(&pointee);
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
            let converted = translate_ffi_type(&content_type);

            let len = ty.get_size().unwrap();

            syn::parse2(quote! { [#converted; #len] }).unwrap()
        }
        TypeKind::CharS | TypeKind::CharU => syn::parse2(quote! {::std::ffi::c_char}).unwrap(),
        TypeKind::Void => syn::parse2(quote! {::std::ffi::c_void}).unwrap(),
        TypeKind::Float => syn::parse2(quote! { f32 }).unwrap(),
        _ => panic!("Unsupported arg type: {}", ty.get_display_name()),
    }
}

/// Convertes the name of a typedef to the corresponding name in rust.
///
/// E.g. `VkDeviceCreateInfo` to `vk::DeviceCreateInfo`
/// or `uint32_t` to `u32`
pub fn convert_typedef(name: &str) -> syn::Type {
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
