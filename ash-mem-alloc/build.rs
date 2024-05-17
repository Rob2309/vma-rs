const VMA_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/vma.cpp");

const INCLUDES: &[&str] = &[
    concat!(env!("CARGO_MANIFEST_DIR"), "/vendor/vma/include"),
    concat!(env!("CARGO_MANIFEST_DIR"), "/vendor/vk-headers/include"),
];

fn main() {
    cc::Build::new()
        .cpp(true)
        .file(VMA_FILE)
        .includes(INCLUDES)
        .warnings(false)
        .compile("vma");
}
