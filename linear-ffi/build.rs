fn main() {
    // Compile the C file and link to OpenBLAS
    cc::Build::new()
        .file("src/matmul.c")
        .flag_if_supported("-O3")
        .compile("libmatmul.a");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("src/matmul.h")
        .allowlist_function("matmul_blas")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .unwrap();

    // Link to OpenBLAS
    println!("cargo:rustc-link-lib=openblas");
}
