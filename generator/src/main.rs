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

const OUT_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../ash-mem-alloc/src/bindings.rs");

mod enums;
mod functions;
mod structs;

fn main() {
    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, false, true);

    let tu = index
        .parser(HEADER)
        .skip_function_bodies(true)
        .arguments(&[VMA_INCLUDES, VK_INCLUDES])
        .parse()
        .unwrap();

    let enums = enums::generate_enums(&tu.get_entity());
    let structs = structs::generate_structs(&tu.get_entity());
    let functions = functions::generate_functions(&tu.get_entity());

    let res = quote! {
        use ash::vk;
        use crate::handles::*;
        use crate::function_ptrs::*;

        #enums
        #structs
        #functions
    };

    std::fs::write(OUT_FILE, res.to_string()).unwrap();

    std::process::Command::new("rustfmt")
        .arg(OUT_FILE)
        .status()
        .unwrap();
}
