use std::fs;
use std::io::Write;

use crate::{BuildConfig, GenRustForLang, new_cmd};

pub(crate) fn gen_code(config: &BuildConfig) {
    gen_go_callee_code(config);
    // FIXME
    gen_rust_caller_code(config);
    gen_rust_crate(config);
}

const GO_FFI_NAME: &'static str = "go_ffi";

fn gen_go_callee_code(config: &BuildConfig) {
    let output = new_cmd()
        .arg(format!(
            "CGO_ENABLED=1 go build -buildmode=c-archive -o {}",
            config.rust_out_dir(GenRustForLang::Go).join(GO_FFI_NAME.to_owned() + ".a").to_str().unwrap()
        ))
        .output()
        .unwrap();
    if !output.status.success() {
        eprintln!("gen_go_callee_code: {:?}", output)
    }
}

#[allow(dead_code)]
fn gen_rust_caller_code(config: &BuildConfig) {
    let rust_out_dir = config.rust_out_dir(GenRustForLang::Go);
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", rust_out_dir.to_str().unwrap());

    // Tell cargo to tell rustc to link our `{GO_FFI_NAME}` library. Cargo will
    // automatically know it must look for a `{GO_FFI_NAME}.a` file.
    println!("cargo:rustc-link-lib={}", GO_FFI_NAME);

    let header_path_str = rust_out_dir.join(GO_FFI_NAME.to_owned() + ".h").to_str().unwrap().to_string();
    // Tell cargo to invalidate the built crate whenever the header changes.
    println!("cargo:rerun-if-changed={}", header_path_str);

    bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(header_path_str)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect(&format!("Unable to generate {}.rs", GO_FFI_NAME))
        .write_to_file(rust_out_dir.join(GO_FFI_NAME.to_owned() + ".rs"))
        .expect(&format!("Couldn't write {}.rs!", GO_FFI_NAME));
}

fn gen_rust_crate(config: &BuildConfig) {
    let dir = config.rust_out_dir(GenRustForLang::Go).canonicalize().unwrap();
    let depth = dir.components().count() + 1;
    let code_path = dir.join("lib.rs");
    let mut code_file = fs::File::create(&code_path).unwrap();
    code_file.write_all(b"#![allow(warnings)]\n").unwrap();
    for entry in fs::read_dir(&dir).unwrap() {
        let entry = entry.unwrap();
        if entry.file_name() == "lib.rs" {
            continue;
        }
        let path = entry.path().canonicalize().unwrap();
        if path.components().count() != depth {
            continue;
        }
        if entry.metadata().unwrap().is_file() && path.extension().unwrap() != "rs" {
            continue;
        }
        if path.is_dir() && !fs::try_exists(path.join("lib.rs")).unwrap_or_default() {
            continue;
        }
        code_file.write_all(format!("pub mod {};\n", path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .trim_end_matches(".rs"))
            .as_bytes())
            .unwrap();
    }
    new_cmd().arg(format!("rustfmt --edition=2021 {}", code_path.to_str().unwrap())).output().unwrap();
    fs::write(dir.parent().unwrap().join("Cargo.toml"), format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
fcplug = "*"
protobuf = "3.2.0"
flatbuffers = "23.5.26"
"#,
        dir.parent().unwrap().file_name().unwrap().to_str().unwrap()))
        .unwrap();
}

// #[allow(dead_code)]
// fn gen_rust_crate(config: &BuildConfig) {
//     let dir = config.rust_out_dir(GenRustForLang::Go).canonicalize().unwrap();
//     struct Module {
//         is_root: bool,
//         name: String,
//         code: Vec<u8>,
//         modules: BTreeMap<String, Module>,
//     }
//     impl Module {
//         fn write_to(&self, f: &mut fs::File) {
//             if !self.is_root {
//                 f.write_all(format!("pub mod {} {{\n", self.name).as_bytes()).unwrap();
//             }
//             f.write_all(self.code.as_slice()).unwrap();
//             for x in &self.modules {
//                 x.1.write_to(f);
//             }
//             if !self.is_root {
//                 f.write_all(b"}\n").unwrap();
//             }
//         }
//     }
//     let crate_name = dir.file_name().unwrap().to_owned().into_string().unwrap();
//     let mut root = Module {
//         is_root: true,
//         name: crate_name.clone(),
//         code: vec![],
//         modules: BTreeMap::new(),
//     };
//     for entry in fs::read_dir(&dir).unwrap() {
//         let entry = entry.unwrap();
//         let path = entry.path().canonicalize().unwrap();
//         if path.extension().unwrap() != "rs" || !entry.metadata().unwrap().is_file() {
//             continue;
//         }
//         let suffix = path.strip_prefix(&dir).unwrap();
//         let mut cur_module = &mut root;
//         for component in suffix.components() {
//             let mod_name = component.as_os_str().to_str().unwrap().trim_end_matches(".rs");
//             if cur_module.modules.get(mod_name).is_none() {
//                 cur_module.modules.insert(mod_name.to_string(), Module {
//                     is_root: false,
//                     name: mod_name.to_string(),
//                     code: vec![],
//                     modules: Default::default(),
//                 });
//             }
//             cur_module = cur_module.modules.get_mut(mod_name).unwrap()
//         }
//         cur_module.code.append(&mut fs::read(&path).unwrap());
//         cur_module.code.push(b'\n');
//     }
//     let code_path = dir.join("lib.rs");
//     let mut code_file = fs::File::create(&code_path).unwrap();
//     code_file.write_all(b"#![allow(warnings)]\n").unwrap();
//     root.write_to(&mut code_file);
//     new_cmd().arg(format!("rustfmt --edition=2021 {}", code_path.to_str().unwrap())).output().unwrap();
// }

#[test]
fn it_works() {}
