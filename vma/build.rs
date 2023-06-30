
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
        .warnings(false)
        .extra_warnings(false)
        .flag("-Wno-everything")
        .compile("vma");
}
