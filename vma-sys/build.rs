fn main() {
    cc::Build::new()
        .cpp(true)
        .include("vendor/VKHeaders/include")
        .include("vendor/VMA/include")
        .warnings(false)
        .file("src/vma.cpp")
        .compile("vma");
}
