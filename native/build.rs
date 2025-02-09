fn main () {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("native")
        .csharp_namespace("UniXCF")
        .csharp_class_accessibility("public");
}
