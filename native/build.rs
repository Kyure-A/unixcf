fn main () {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("native")
        .generate_csharp_file("../Native.cs")
        .unwrap_or_else(|_| panic!("csbindgen: build failed"));
}
