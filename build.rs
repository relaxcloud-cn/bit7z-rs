use std::env;
// use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let lib = format!("{}/lib", manifest_dir);
    let cpp_dir = format!("{}/cpp", manifest_dir);
    let include_dir = format!("{}/include", manifest_dir);

    cxx_build::bridge("src/lib.rs")
        .file("cpp/bridge.cpp")
        .file("cpp/implementations.cpp")
        .include(&cpp_dir) 
        .include(&include_dir)
        .flag_if_supported("-std=c++17")
        .define("BIT7Z_AUTO_FORMAT", None)
        .compile("extract_bridge");


    println!("cargo:rustc-link-search=native={}", lib);
    println!("cargo:rustc-link-lib=bit7z64"); 

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=cpp/bridge.cpp");
    println!("cargo:rerun-if-changed=cpp/implementations.cpp");
    println!("cargo:rerun-if-changed=cpp/bridge.hpp");

    Ok(())
}
