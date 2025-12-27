fn main() {
    // 1. Get the absolute path to your 'onnx-libs' folder
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_path = std::path::Path::new(&manifest_dir)
        .join("resources")
        .join("onnx-libs");

    // 2. Tell the linker to search in that folder
    println!("cargo:rustc-link-search=native={}", lib_path.display());

    // 3. For macOS: Set the rpath so the executable knows where to find
    // the dylib at runtime relative to itself.
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path/../Resources/onnx-libs");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path.display());
    }

    tauri_build::build()
}
