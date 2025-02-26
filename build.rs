use std::path::Path;

use cmake::Config;

fn main() {
    if !Path::new("cpp/CMakeLists.txt").exists() {
        panic!("FastPFOR submodule not initialized. Run `git submodule update --init`.");
    }

    // Compile FastPFOR using CMake
    let lib_path = Config::new("cpp").build().join("lib");
    let lib_path = lib_path.to_str().unwrap();
    println!("cargo:rerun-if-changed=cpp");

    // Compile the bridge
    cxx_build::bridge("src/cpp.rs")
        .include("cpp/headers")
        .include("src/bridge")
        .std("c++14")
        .compile("fastpfor_bridge");
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=src/bridge/fastpfor_bridge.cc");
    println!("cargo:rerun-if-changed=src/bridge/fastpfor_bridge.h");

    // Link the FastPFOR library - must be done after the bridge is compiled
    println!("cargo:rustc-link-search=native={lib_path}");
    println!("cargo:rustc-link-lib=static=FastPFOR");
}
