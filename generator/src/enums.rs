use clang::{Entity, EntityKind};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{LitInt, LitStr};

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

fn generate_enum(e: &VmaEnum) -> TokenStream {
    let name = &e.name;

    let docs = e.docs.as_ref().map(|docs| quote!{ #[doc = #docs] });

    let variants = e.variants.iter().map(|variant| {
        let name = &variant.name;
        let docs = variant.docs.as_ref().map(|docs| quote!{ #[doc = #docs] });
        let val = &variant.value;

        quote!{
            #docs
            pub const #name: Self = Self(#val);
        }
    });

    let bit_impls = e.is_bitfield.then(|| {
        quote!{
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
        }
    });

    quote! {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        #docs
        pub struct #name(u32);

        impl #name {
            #(#variants)*
        }

        #bit_impls
    }
}

fn parse_enum(item: &Entity) -> VmaEnum {
    let name = item.get_name().unwrap().trim_start_matches("Vma").to_string();
    let name = if let Some(name) = name.strip_suffix("Bits") {
        format_ident!("{name}s")
    } else {
        format_ident!("{name}")
    };

    let is_bitfield = item.get_name().unwrap().ends_with("Bits");

    let docs = item
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

    let variants = item
        .get_children()
        .iter()
        .filter(|child| child.get_kind() == EntityKind::EnumConstantDecl)
        .map(parse_variant)
        .collect::<Vec<_>>();

    VmaEnum {
        name,
        is_bitfield,
        docs,
        variants,
    }
}

fn parse_variant(variant: &Entity) -> VmaEnumVariant {
    let name = format_ident!("{}", variant.get_name().unwrap());

    let docs = variant
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

    let value = format!("{}", variant.get_enum_constant_value().unwrap().1);
    let value = syn::parse_str(&value).unwrap();

    VmaEnumVariant {
        name,
        docs,
        value,
    }
}

struct VmaEnum {
    name: Ident,
    is_bitfield: bool,
    docs: Option<LitStr>,
    variants: Vec<VmaEnumVariant>,
}

struct VmaEnumVariant {
    name: Ident,
    docs: Option<LitStr>,
    value: LitInt,
}
