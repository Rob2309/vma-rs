const VMA_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/vma.cpp");

const INCLUDES: &[&str] = &[
    concat!(env!("CARGO_MANIFEST_DIR"), "/vendor/vma/include"),
    concat!(env!("CARGO_MANIFEST_DIR"), "/vendor/vk-headers/include"),
];

fn main() {
    cc::Build::new()
        .file(VMA_FILE)
        .cpp(true)
        .includes(INCLUDES)
        .flag_if_supported("-Wno-everything")
        .compile("vma");
}
