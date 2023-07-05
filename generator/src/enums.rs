use clang::{Entity, EntityKind};
use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{LitInt, LitStr};

/// Generates bindings for all Vma enums.
///
/// Enums that end with "...FlagBits" are interpreted as bitfield enums
/// and will get bitwise operators.
pub fn generate_enums(tu: &Entity) -> TokenStream {
    let mut vma_enums = Vec::new();

    for item in tu.get_children() {
        if item.get_kind() == EntityKind::EnumDecl && item.get_name().unwrap().starts_with("Vma") {
            vma_enums.push(parse_enum(&item));
        }
    }

    let generated = vma_enums.iter().map(generate_enum);

    quote! {
        #(#generated)*
    }
}

/// Generates the rust code for a single parsed enum
fn generate_enum(e: &VmaEnum) -> TokenStream {
    let name = &e.name;

    // transform doc string into #[doc = "..."] attribute
    let docs = e.docs.as_ref().map(|docs| quote! { #[doc = #docs] });

    // generate code for every variant
    let variants = e.variants.iter().map(|variant| {
        let name = &variant.name;
        let docs = variant.docs.as_ref().map(|docs| quote! { #[doc = #docs] });
        let val = &variant.value;

        quote! {
            #docs
            pub const #name: Self = Self(#val);
        }
    });

    // bitwise operators if this a bitfield enum
    let bit_impls = e.is_bitfield.then(|| {
        quote! {
            impl ::std::ops::BitOr<#name> for #name {
                type Output = Self;
                fn bitor(self, rhs: #name) -> Self::Output {
                    Self(self.0 | rhs.0)
                }
            }
            impl ::std::ops::BitOrAssign<#name> for #name {
                fn bitor_assign(&mut self, rhs: #name) {
                    self.0 |= rhs.0;
                }
            }
            impl ::std::ops::BitAnd<#name> for #name {
                type Output = Self;
                fn bitand(self, rhs: #name) -> Self::Output {
                    Self(self.0 & rhs.0)
                }
            }
            impl ::std::ops::BitAndAssign<#name> for #name {
                fn bitand_assign(&mut self, rhs: #name) {
                    self.0 &= rhs.0;
                }
            }
            impl ::std::ops::BitXor<#name> for #name {
                type Output = Self;
                fn bitxor(self, rhs: #name) -> Self::Output {
                    Self(self.0 ^ rhs.0)
                }
            }
            impl ::std::ops::BitXorAssign<#name> for #name {
                fn bitxor_assign(&mut self, rhs: #name) {
                    self.0 ^= rhs.0;
                }
            }
            impl ::std::ops::Not for #name {
                type Output = Self;
                fn not(self) -> Self::Output {
                    Self(!self.0)
                }
            }

            impl #name {
                /// Checks whether `other` is a subset of `self`
                pub fn contains(self, other: Self) -> bool {
                    (self.0 & other.0) == other.0
                }

                /// Checks whether `other` and `self` have bits in common
                pub fn intersects(self, other: Self) -> bool {
                    (self.0 & other.0) != 0
                }
            }
        }
    });

    quote! {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        #docs
        pub struct #name(u32);

        impl #name {
            #(#variants)*

            pub fn empty() -> Self {
                Self(0)
            }

            pub fn into_raw(self) -> u32 {
                self.0
            }

            pub fn from_raw(v: u32) -> Self {
                Self(v)
            }
        }

        #bit_impls
    }
}

/// Parses the libclang definition of an `enum`
fn parse_enum(item: &Entity) -> VmaEnum {
    // Remove "Vma" prefix and transform "...FlagBits" to "...Flags"
    let name = item
        .get_name()
        .unwrap()
        .trim_start_matches("Vma")
        .to_string();
    let name = if let Some(name) = name.strip_suffix("Bits") {
        format_ident!("{name}s")
    } else {
        format_ident!("{name}")
    };

    let is_bitfield = item.get_name().unwrap().ends_with("Bits");

    // Get documentation comment and remove "/**" and "*/" for better readability.
    // Also remove four consecutive spaces as these lines are interpreted as rust
    // code blocks, breaking doc tests
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
    
    // Parse all enum variants, except "...MAX_ENUM"
    let variants = item
        .get_children()
        .iter()
        .filter(|child| {
            child.get_kind() == EntityKind::EnumConstantDecl
                && !child.get_name().unwrap().ends_with("MAX_ENUM")
        })
        .map(|variant| parse_variant(&name.to_string(), variant))
        .collect::<Vec<_>>();

    VmaEnum {
        name,
        is_bitfield,
        docs,
        variants,
    }
}

/// Parses a single variant of an enum declaration
fn parse_variant(struct_name: &str, variant: &Entity) -> VmaEnumVariant {
    // Determine the prefix of the variant name to be removed.
    // For example, for VMA_ALLOCATOR_CREATE_EXTERNALLY_SYNCHRONIZED_BIT
    // VMA_ALLOCATOR_CREATE_ should be removed.
    // Also remove trailing _BIT
    let prefix = format!(
        "VMA_{}_",
        struct_name
            .trim_end_matches("Flags")
            .to_case(Case::UpperSnake)
    );
    let name = format_ident!(
        "{}",
        variant
            .get_name()
            .unwrap()
            .trim_start_matches(&prefix)
            .trim_end_matches("_BIT")
    );

    // Get documentation comment and remove "/**" and "*/" for better readability.
    // Also remove four consecutive spaces as these lines are interpreted as rust
    // code blocks, breaking doc tests
    let docs = variant
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

    let value = format!("{}", variant.get_enum_constant_value().unwrap().1);
    let value = syn::parse_str(&value).unwrap();

    VmaEnumVariant { name, docs, value }
}

/// Description of a Vma enum
struct VmaEnum {
    /// rust name of the enum
    name: Ident,
    /// whether bitwise operations should be implemented
    is_bitfield: bool,
    /// documentation of the enum
    docs: Option<LitStr>,
    /// list of variants contained in the enum
    variants: Vec<VmaEnumVariant>,
}

/// Description of a single enum variant
struct VmaEnumVariant {
    /// rust name of the variant
    name: Ident,
    /// documentation of the variant
    docs: Option<LitStr>,
    /// the discriminant of the variant
    value: LitInt,
}
