use std::{env, fs};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Component, MAIN_SEPARATOR_STR, Path, PathBuf};

use cbindgen::Language;
use flatc_rust;
use protoc_rust::{Codegen, Customize};

use crate::ffi_go::gen_go_code;
pub use crate::ffi_go::PbGoConfig;

mod ffi_go;

pub struct BuildConfig {
    pub go_out_dir: PathBuf,
    pub rust_out_dir: PathBuf,
    pub pb_rust_configs: Vec<PbRustConfig>,
    pub pb_go_config: Option<PbGoConfig>,
    pub fb_configs: FbConfigs,
}

pub type PbCustomize = Customize;

pub struct PbRustConfig {
    pub inputs: Vec<PathBuf>,
    pub include: Option<PathBuf>,
    pub customize: Option<PbCustomize>,
}


#[derive(Debug)]
pub struct Report {
    pub c_header_filename: String,
    pub c_lib_filename: String,
}


pub fn build_files(config: BuildConfig) {
    fs::create_dir_all(&config.go_out_dir).unwrap();
    fs::create_dir_all(&config.rust_out_dir).unwrap();
    let report = gen_c_header();
    gen_rust_protobuf(&config);
    gen_flatbuf(&config);
    gen_go_code(&config, &report);
    gen_gen_sh(&config, &report);
}


fn gen_rust_protobuf(config: &BuildConfig) {
    config.pb_rust_configs.iter().for_each(|pb_conf| {
        Codegen::new()
            .out_dir(&config.rust_out_dir)
            .include(pb_conf.include.clone().unwrap_or("./".into()))
            .inputs(&pb_conf.inputs)
            .customize(pb_conf.customize.as_ref().unwrap_or(&Customize {
                carllerche_bytes_for_bytes: Some(true),
                generate_accessors: Some(true),
                ..Default::default()
            }).clone())
            .run()
            .expect("Unable to generate proto file");
    });
}

pub fn gen_c_header() -> Report {
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
        .with_after_include(if cargo_pkg_name != "fcplug-rclib" {
            r#"
typedef enum OriginType {
  Vec = 0,
  FlatBuffer = 1,
} OriginType;

typedef enum ResultCode {
  NoError = 0,
  Decode = 1,
  Encode = 2,
} ResultCode;

typedef struct Buffer {
  uint8_t *ptr;
  uintptr_t len;
  uintptr_t cap;
} Buffer;

typedef struct LeakBuffer {
  enum OriginType free_type;
  uintptr_t free_ptr;
  struct Buffer buffer;
} LeakBuffer;

typedef struct FFIResult {
  enum ResultCode code;
  struct LeakBuffer data;
} FFIResult;

void free_buffer(enum OriginType free_type, uintptr_t free_ptr);
"#
        } else { "" })
        .generate()
        .expect("Unable to generate C header file")
        .write_to_file(&report.c_header_filename);
    report
}


fn gen_gen_sh(config: &BuildConfig, report: &Report) {
    let go_out_dir = config.go_out_dir.to_str().unwrap();
    let mut f = fs::File::create("gen.sh").expect("Couldn't create gen.sh");
    f.write("#!/bin/bash\n\n".as_bytes()).unwrap();
    f.write(format!("mkdir -p {}\n", go_out_dir).as_bytes()).unwrap();
    f.write(format!("cp -rf {} {}\n", report.c_header_filename, go_out_dir).as_bytes()).unwrap();
    f.write(format!("cp -rf {} {}\n", report.c_lib_filename, go_out_dir).as_bytes()).unwrap();
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

pub enum FbConfig {
    Rust,
    Go,
}

pub struct FbConfigs {
    pub inputs: Vec<PathBuf>,
    pub configs: Vec<FbConfig>,
}

fn gen_flatbuf(config: &BuildConfig) {
    let inputs = config.fb_configs.inputs.iter().map(|v| v.as_path()).collect::<Vec<&Path>>();
    for conf in &config.fb_configs.configs {
        match conf {
            FbConfig::Rust => {
                flatc_rust::run(flatc_rust::Args {
                    lang: "rust",
                    inputs: &inputs,
                    out_dir: config.rust_out_dir.as_path(),
                    ..Default::default()
                }).expect("failed to generate flatbuf rust code");
            }
            FbConfig::Go => {
                let out_dir = config.go_out_dir.canonicalize().unwrap();
                flatc_rust::run(flatc_rust::Args {
                    lang: "go",
                    inputs: &inputs,
                    out_dir: out_dir.parent().unwrap(),
                    extra: &["--go-namespace", out_dir.file_name().unwrap().to_str().unwrap()],
                    ..Default::default()
                }).expect("failed to generate flatbuf go code");
            }
        }
    }
}

#[test]
fn test() {
    env::set_var("CARGO_PKG_NAME", "demo");
    env::set_var("CARGO_MANIFEST_DIR", "/Users/henrylee2cn/rust/fcplug/demo");
    env::set_var("OUT_DIR", "/Users/henrylee2cn/rust/fcplug/target/debug/build/demo-f8ee84ac19343019/out");

    let _ = gen_c_header();
}
