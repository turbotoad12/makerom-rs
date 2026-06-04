use bindgen::Builder;
use cc;
use std::env;
use std::path::PathBuf;

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
        .warnings(true);

    // Platform-specific configuration
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    
    if target_os == "windows" {
        // For Windows MSVC or MinGW
        build.define("_WIN32", None);
        build.define("_GNU_SOURCE", None);
    } else {
        // For Linux/Unix
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
    println!("cargo:rerun-if-changed=makerom-headers.h");
    println!("cargo:rerun-if-changed={}", makerom_src);

    // Generate bindings
    let mut builder = Builder::default();
    builder = builder
        .header("makerom-headers.h")
        .generate_inline_functions(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    // Add clang arguments for Windows
    if target_os == "windows" {
        builder = builder.clang_arg("-D_GNU_SOURCE");
    }

    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    let bindings_path = PathBuf::from(&out_dir).join("bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");
}
