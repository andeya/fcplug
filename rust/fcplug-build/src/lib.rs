mod ffi_go;

use lazy_static::lazy_static;
use regex::Regex;
use protoc_rust::{Codegen, Customize};
use std::{env, fs, str};
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
    fs::create_dir_all(&go_proto_code_config.out_dir).unwrap();
    rust_proto_code_configs.into_iter().for_each(gen_proto_rust);
    let report = gen_c_header();
    gen_gen_sh(&go_proto_code_config, &report);
    gen_go_ffi_code(&go_proto_code_config, &report);
}


fn gen_proto_rust(config: RustProtoCodeConfig) {
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

fn gen_c_header() -> Report {
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


fn gen_gen_sh(go_proto_code_config: &GoProtoCodeConfig, report: &Report) {
    let mut f = fs::File::create("gen.sh").expect("Couldn't create gen.sh");
    f.write("#!/bin/bash\n\n".as_bytes()).unwrap();
    f.write(format!("mkdir -p {}\n", go_proto_code_config.out_dir).as_bytes()).unwrap();
    f.write(format!("cp -rf {} {}\n", report.c_header_filename, go_proto_code_config.out_dir).as_bytes()).unwrap();
    f.write(format!("cp -rf {} {}\n", report.c_lib_filename, go_proto_code_config.out_dir).as_bytes()).unwrap();
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

// Buffer ffi_echo(Buffer args);

fn gen_go_ffi_code(go_proto_code_config: &GoProtoCodeConfig, report: &Report) {
    if env::var("CARGO_PKG_NAME").unwrap() == "fcplug-rclib" {
        return;
    }
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Buffer (?P<c_fn_name>[A-Z_a-z0-9]+)\(Buffer args\);").unwrap();
    }
    let header = fs::read(&report.c_header_filename);
    if header.is_err() {
        println!("{}", header.err().unwrap());
        return;
    }
    let header = header.unwrap();
    let fn_list = RE.captures_iter(str::from_utf8(&header).unwrap()).map(|cap| {
        cap["c_fn_name"].to_string()
    }).collect::<Vec<String>>();

    println!("fn_list: {:?}", fn_list);

    let fn_list = fn_list
        .iter()
        .map(|c_fn_name| ffi_go::FN_TPL.replace("${c_fn_name}", c_fn_name))
        .collect::<Vec<String>>()
        .join("\n");

    let file_txt = ffi_go::FILE_TPL
        .replace(
            "${package}",
            PathBuf::from(&go_proto_code_config.out_dir)
                .canonicalize()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        )
        .replace(
            "${c_header_name_base}",
            PathBuf::from(&report.c_header_filename)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .trim_end_matches(".h"),
        )
        .replace("${fn_list}", &fn_list);

    fs::write(PathBuf::from(&go_proto_code_config.out_dir).join(&ffi_go::FILE_NAME), file_txt.as_bytes()).unwrap();
}


#[test]
fn test() {
    env::set_var("CARGO_PKG_NAME", "demo");
    env::set_var("CARGO_MANIFEST_DIR", "/Users/henrylee2cn/rust/fcplug/demo");
    env::set_var("OUT_DIR", "/Users/henrylee2cn/rust/fcplug/target/debug/build/demo-f8ee84ac19343019/out");

    let _ = gen_c_header();
}
