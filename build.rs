use std::env;
use vcpkg;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cpp_dir = format!("{}/cpp", manifest_dir);
    let include_dir = format!("{}/include", manifest_dir);

    // using vcpkg to find bit7z
    let lib = vcpkg::Config::new()
        .emit_includes(true)
        .find_package("bit7z")?;

    // building cxx bridge
    let mut build = cxx_build::bridge("src/lib.rs");
    build
        .file("cpp/bridge.cpp")
        .file("cpp/implementations.cpp")
        .include(&cpp_dir)
        .include(&include_dir)
        .flag_if_supported("-std=c++17")
        .define("BIT7Z_AUTO_FORMAT", None);

    // add vcpkg to find packages
    for include in lib.include_paths {
        build.include(&include);
    }

    build.compile("extract_bridge");

    // println!("cargo:rustc-link-search") 和 println!("cargo:rustc-link-lib")

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=cpp/bridge.cpp");
    println!("cargo:rerun-if-changed=cpp/implementations.cpp");
    println!("cargo:rerun-if-changed=cpp/bridge.hpp");
    println!("cargo:rerun-if-changed=vcpkg.json");

    Ok(())
}
