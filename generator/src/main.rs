use clang::{Clang, Index};
use quote::quote;

const HEADER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/header.h");

const VMA_INCLUDES: &str = concat!(
    "-I",
    env!("CARGO_MANIFEST_DIR"),
    "/../ash-mem-alloc/vendor/vma/include"
);

const VK_INCLUDES: &str = concat!(
    "-I",
    env!("CARGO_MANIFEST_DIR"),
    "/../ash-mem-alloc/vendor/vk-headers/include"
);

const ENUMS_OUT_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../ash-mem-alloc/src/enums.rs");
const STRUCTS_OUT_FILE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../ash-mem-alloc/src/structs.rs"
);
const FUNCTIONS_OUT_FILE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../ash-mem-alloc/src/functions.rs"
);
const FFI_OUT_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../ash-mem-alloc/src/ffi.rs");

mod enums;
mod parsing;
mod structs;
mod functions;

fn main() {
    // This generator directly uses libclang instead of bindgen because
    // bindgen does not give us enough control to generate ash-style bindings

    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, false, true);

    let tu = index
        .parser(HEADER)
        .skip_function_bodies(true)
        .arguments(&[VMA_INCLUDES, VK_INCLUDES])
        .parse()
        .unwrap();

    let enums = enums::generate_enums(&tu.get_entity());
    let enums = quote! {
        #enums
    };
    std::fs::write(ENUMS_OUT_FILE, enums.to_string()).unwrap();
    std::process::Command::new("rustfmt")
        .arg(ENUMS_OUT_FILE)
        .status()
        .unwrap();

    let structs = structs::generate_structs(&tu.get_entity());
    let structs = quote! {
        use ash::vk;
        use crate::function_ptrs::*;

        #structs
    };

    std::fs::write(STRUCTS_OUT_FILE, structs.to_string()).unwrap();
    std::process::Command::new("rustfmt")
        .arg(STRUCTS_OUT_FILE)
        .status()
        .unwrap();

    let functions = functions::generate_functions(&tu.get_entity());
    let functions = quote! {
        #![allow(warnings)]

        use ash::vk;

        #functions
    };
    std::fs::write(FUNCTIONS_OUT_FILE, functions.to_string()).unwrap();
    std::process::Command::new("rustfmt")
        .arg(FUNCTIONS_OUT_FILE)
        .status()
        .unwrap();

    let bindings = bindgen::builder()
        .header(HEADER)
        .clang_arg(VMA_INCLUDES)
        .clang_arg(VK_INCLUDES)
        .allowlist_function("vma.*")
        .raw_line("#![allow(warnings)]")
        .generate()
        .expect("Failed to generate ffi bindings");
    bindings
        .write_to_file(FFI_OUT_FILE)
        .expect("Failed to write ffi bindings");
}
