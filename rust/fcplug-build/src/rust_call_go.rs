use crate::{BuildConfig, new_cmd};

pub(crate) fn gen_code(config: &BuildConfig) {
    gen_go_callee_code(config);
    // gen_rust_caller_code(config);
}

fn gen_go_callee_code(config: &BuildConfig) {
    let output = new_cmd()
        .arg(format!("CGO_ENABLED=1 go build -buildmode=c-archive -o {}",
                     config.rust_out_dir.canonicalize().unwrap().join("go_ffi.a").to_str().unwrap()))
        .output()
        .unwrap();
    if !output.status.success() {
        eprintln!("gen_go_callee_code: {:?}", output)
    }
}

#[allow(dead_code)]
fn gen_rust_caller_code(config: &BuildConfig) {
    // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search={}", config.rust_out_dir.canonicalize().unwrap().to_str().unwrap());
    //
    // // Tell cargo to tell rustc to link the system bzip2
    // // shared library.
    // println!("cargo:rustc-link-lib=bz2");

    // // Tell cargo to invalidate the built crate whenever the go_ffi changes
    // println!("cargo:rerun-if-changed={}", config.rust_out_dir.canonicalize().unwrap().to_str().unwrap());

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(config.rust_out_dir.join("go_ffi.h").canonicalize().unwrap().to_str().unwrap())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate go_ffi.rs");

    bindings
        .write_to_file(config.rust_out_dir.join("go_ffi.rs"))
        .expect("Couldn't write go_ffi.rs!");
}
