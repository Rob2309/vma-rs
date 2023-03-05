fn main() {
    cc::Build::new()
        .cpp(true)
        .include("vendor/VKHeaders/include")
        .include("vendor/VMA/include")
        .file("src/vma.cpp")
        .compile("vma");
}
