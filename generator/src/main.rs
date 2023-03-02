use std::collections::BTreeMap;

use convert_case::{Case, Casing};
use syn::File;

const HEADER_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../vma-sys/vendor/VMA/include/vk_mem_alloc.h"
);
const SYS_OUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../vma-sys/src/bindings.rs");
const ENUMS_OUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../vma/src/enums.rs");

fn main() {
    let bindings = bindgen::builder()
        .header(HEADER_PATH)
        .allowlist_function("vma.*")
        .allowlist_type("Vma.*")
        .prepend_enum_name(false)
        .generate()
        .expect("Failed to create sys bindings");

    bindings
        .write_to_file(SYS_OUT_PATH)
        .expect("Failed to write sys bindings");

    let bindings =
        std::fs::read_to_string(SYS_OUT_PATH).expect("Failed to read generated sys bindings");
    let bindings =
        syn::parse_str::<syn::File>(&bindings).expect("Failed to parse generated sys bindings");

    generate_enums(&bindings);
}

fn generate_enums(bindings: &File) {
    let flag_structs = generate_flag_enums(bindings);
    let value_structs = generate_value_enums(bindings);

    let output = format!("
use bitflags::bitflags;
use vma_sys::*;

{flag_structs}
{value_structs}
    ");
    std::fs::write(ENUMS_OUT_PATH, output).expect("Failed to write generated enums");
    std::process::Command::new("rustfmt")
        .arg(ENUMS_OUT_PATH)
        .status()
        .expect("Failed to run rustfmt");
}

fn generate_value_enums(bindings: &File) -> String {
    let mut enum_types = BTreeMap::new();
    for item in &bindings.items {
        if let syn::Item::Type(alias) = item {
            let name = alias.ident.to_string();
            if name.ends_with("Flags") || name.ends_with("FlagBits") || !name.starts_with("Vma") {
                continue;
            }
            if let syn::Type::Path(_) = *alias.ty {
                enum_types.insert(name, Vec::new());
            }
        }
    }
    for item in &bindings.items {
        if let syn::Item::Const(value) = item {
            let name = value.ident.to_string();
            if name.ends_with("_BIT") || name.ends_with("MAX_ENUM") || !name.starts_with("VMA_") {
                continue;
            }

            if let Some((enum_name, values)) = enum_types.iter_mut().find(|(enum_name, _)| name.starts_with(&enum_name.to_case(Case::UpperSnake))) {
                let enum_name = enum_name.to_case(Case::UpperSnake);
                let value_name = name[enum_name.len()+1..].to_string();

                values.push((value_name, name));
            }
        }
    }

    enum_types.iter().filter(|(_, values)| !values.is_empty()).map(|(enum_name, values)| {
        let values = values.iter().map(|(rust_name, val_name)| {
            format!("    pub const {rust_name}: Self = Self({val_name});")
        }).collect::<Vec<_>>().join("\n");

        format!("
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct {enum_name}(u32);
impl {enum_name} {{
{values}
}}
        ")
    }).collect::<Vec<_>>().join("\n")
}

fn generate_flag_enums(bindings: &File) -> String {
    let mut flag_types = BTreeMap::new();
    for item in &bindings.items {
        if let syn::Item::Type(alias) = item {
            let name = alias.ident.to_string();
            if name.starts_with("Vma") && name.ends_with("Flags") {
                flag_types.insert(name[3..name.len() - 5].to_string(), Vec::new());
            }
        }
    }
    for (flag_name, elements) in &mut flag_types {
        for item in &bindings.items {
            if let syn::Item::Const(flag_bit) = item {
                let name = flag_bit.ident.to_string();
                if name.ends_with("MAX_ENUM") || !name.starts_with(&format!("VMA_{}", flag_name.to_case(Case::UpperSnake))) {
                    continue;
                }
                if !name.ends_with("_BIT") && !name.ends_with("_MASK") {
                    continue;
                }

                let mut rust_name = name[4+flag_name.to_case(Case::UpperSnake).len()+1..].to_string();
                if rust_name.ends_with("_BIT") {
                    rust_name = rust_name[..rust_name.len()-4].to_string();
                }

                elements.push((rust_name, name));
            }
        }
    }

    flag_types
        .iter()
        .map(|(name, elements)| {
            let name = format!("{name}Flags");

            let elements = elements
                .iter()
                .map(|(name, value)| format!("        const {name} = {value};"))
                .collect::<Vec<_>>()
                .join("\n");

            format!(
                "
bitflags!{{
    #[repr(transparent)]
    pub struct {name}: u32 {{
{elements}
    }}
}}
        "
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
