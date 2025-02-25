use cmake::Config;

fn main() {
    // Compile FastPFOR using CMake
    let lib_path = Config::new("cpp").build().join("lib");
    let lib_path = lib_path.to_str().unwrap();
    println!("cargo:rerun-if-changed=cpp");

    // Compile the bridge
    cxx_build::bridge("src/lib.rs")
        .include("cpp/headers")
        .include("src/bridge")
        .file("src/bridge/fastpfor_bridge.cc")
        .std("c++14")
        .compile("fastpfor_bridge");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/bridge/fastpfor_bridge.cc");
    println!("cargo:rerun-if-changed=src/bridge/fastpfor_bridge.h");

    // Link the FastPFOR library - must be done after the bridge is compiled
    println!("cargo:rustc-link-search=native={lib_path}");
    println!("cargo:rustc-link-lib=static=FastPFOR");
}
