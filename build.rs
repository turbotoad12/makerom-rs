use cc;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let makerom_src = "makerom/src";

    // Get all C source files from makerom/src
    let c_files: Vec<String> = std::fs::read_dir(makerom_src)
        .expect("Failed to read makerom/src directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "c") {
                path.to_str().map(|s| s.to_string())
            } else {
                None
            }
        })
        .collect();

    println!("Compiling {} C source files", c_files.len());

    // Build configuration
    let mut build = cc::Build::new();
    build
        .files(&c_files)
        .include("makerom/src")
        .include("makerom/deps/libmbedtls/include")
        .include("makerom/deps/libblz/include")
        .include("makerom/deps/libyaml/include")
        .opt_level(2)
        .flag("-std=c11")
        .flag("-fPIC")
        .warnings(false);  // Suppress warnings for cleaner output

    // Platform-specific configuration
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    
    if target_os == "windows" {
        build.define("_WIN32", None);
        build.define("_GNU_SOURCE", None);
    } else {
        build.define("_GNU_SOURCE", None);
    }

    build.compile("makerom_c");

    // Link against compiled libraries
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=makerom_c");
    
    // Link against dependency libraries if they're compiled
    println!("cargo:rustc-link-search=native=makerom/deps/libmbedtls/bin");
    println!("cargo:rustc-link-search=native=makerom/deps/libblz/bin");
    println!("cargo:rustc-link-search=native=makerom/deps/libyaml/bin");

    // Tell cargo to invalidate the built crate whenever build script changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", makerom_src);
}
