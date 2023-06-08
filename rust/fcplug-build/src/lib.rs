use protoc_rust::{Codegen, Customize};
use std::{env, fs};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Component, MAIN_SEPARATOR_STR, PathBuf};

use cbindgen::Language;

pub type PbCustomize = Customize;

pub struct RustProtoCodeConfig {
    pub out_dir: PathBuf,
    pub inputs: Vec<PathBuf>,
    pub include: Option<PathBuf>,
    pub customize: Option<PbCustomize>,
}

pub struct GoProtoCodeConfig {
    pub out_dir: String,
    pub filename: String,
}

#[derive(Debug)]
pub struct Report {
    pub c_header_filename: String,
    pub c_lib_filename: String,
}

pub fn build_files(rust_proto_code_configs: Vec<RustProtoCodeConfig>, go_proto_code_config: GoProtoCodeConfig) {
    rust_proto_code_configs.into_iter().for_each(gen_proto_code);
    let report = gen_c_code();
    let mut f = fs::File::create("build.sh").expect("Couldn't create build.sh");
    f.write("#!/bin/bash\n\n".as_bytes()).unwrap();
    f.write(format!("cp -rf {} .\n", report.c_header_filename).as_bytes()).unwrap();
    f.write(format!("cp -rf {} .\n", report.c_lib_filename).as_bytes()).unwrap();
    f.write(format!("mkdir -p {}\n", go_proto_code_config.out_dir).as_bytes()).unwrap();
    f.write(format!("protoc --proto_path={} --go_out {} {}",
                    PathBuf::from(&go_proto_code_config.filename).parent().unwrap().to_str().unwrap(),
                    go_proto_code_config.out_dir,
                    go_proto_code_config.filename,
    ).as_bytes()).unwrap();
    f.flush().unwrap();
    let mut permissions = f.metadata().unwrap().permissions();
    permissions.set_mode(0o777);
    if let Err(ref e) = f.set_permissions(permissions) {
        println!("failed to set permissions of build.sh: {}", e);
    }
}

fn target_profile_dir() -> PathBuf {
    let mut p = PathBuf::new();
    PathBuf::from(&std::env::var("OUT_DIR").unwrap())
        .components()
        .rev()
        .skip(3)
        .collect::<Vec<Component>>()
        .into_iter()
        .rev()
        .for_each(|c| p.push(c.as_os_str()));
    p
}

#[test]
fn test() {
    env::set_var("CARGO_PKG_NAME", "demo");
    env::set_var("CARGO_MANIFEST_DIR", "/Users/henrylee2cn/rust/fcplug/demo");
    env::set_var("OUT_DIR", "/Users/henrylee2cn/rust/fcplug/target/debug/build/demo-f8ee84ac19343019/out");

    let _ = gen_c_code();
}

fn gen_c_code() -> Report {
    let base = target_profile_dir().as_os_str()
        .to_str()
        .unwrap()
        .to_string();
    let cargo_pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    let report = Report {
        c_header_filename: base.clone() + MAIN_SEPARATOR_STR + &cargo_pkg_name.replace("-", "_") + ".h",
        c_lib_filename: base + MAIN_SEPARATOR_STR + "lib" + &cargo_pkg_name.replace("-", "_") + ".a",
    };
    println!("build-log: {:?}", report);
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(Language::C)
        .with_parse_expand(&[cargo_pkg_name.as_str()])
        .with_after_include(if cargo_pkg_name != "fcplug" {
            r#"
typedef struct Buffer {
  uint8_t *ptr;
  uintptr_t len;
  uintptr_t cap;
} Buffer;

void free_buffer(struct Buffer buf);
"#
        } else { "" })
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&report.c_header_filename);
    report
}

fn gen_proto_code(config: RustProtoCodeConfig) {
    Codegen::new()
        .out_dir(&config.out_dir)
        .include(config.include.clone().unwrap_or("./".into()))
        .inputs(&config.inputs)
        .customize(config.customize.unwrap_or(Customize {
            carllerche_bytes_for_bytes: Some(true),
            generate_accessors: Some(true),
            ..Default::default()
        }))
        .run()
        .expect("Unable to generate proto file");
}
