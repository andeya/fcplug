#![allow(dead_code)]

fn main() {
    // gen_callee()
    gen_caller()
}

fn gen_callee() {
    let cargo_pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_parse_expand(&[cargo_pkg_name.as_str()])
        .generate()
        .expect("Unable to generate C header file")
        .write_to_file("./feasibility.h");
}

fn gen_caller() {
    // This is the directory where the `c` library is located.
    let libdir_path = std::path::PathBuf::from("/Users/henrylee2cn/rust/fcplug-byted/feasibility")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");

    // This is the path to the `c` headers file.
    let headers_path = libdir_path.join("feasibility.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    // This is the path to the static library file.
    let lib_path = libdir_path.join("libfeasibility.a");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());

    // Tell cargo to tell rustc to link our `feasibility` library. Cargo will
    // automatically know it must look for a `libfeasibility.a` file.
    println!("cargo:rustc-link-lib=feasibility");

    // Tell cargo to invalidate the built crate whenever the header changes.
    println!("cargo:rerun-if-changed={}", headers_path_str);
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.

    bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(headers_path_str)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings")
        .write_to_file(std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Couldn't write bindings!");
    println!("OUT_FILE={}", std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs").to_str().unwrap());
}
