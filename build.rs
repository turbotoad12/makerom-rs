use bindgen::Builder;
use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to link the makerom C library
    // Adjust this path based on where you've compiled the makerom C code
    // println!("cargo:rustc-link-search=native=/path/to/makerom/build");
    // println!("cargo:rustc-link-lib=makerom");

    // Tell cargo to invalidate the built crate whenever this build script changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=makerom-headers.h");

    // Generate bindings
    let bindings = Builder::default()
        .header("makerom-headers.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs");
        .expect("Couldn't write bindings!");
}
