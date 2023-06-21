use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub use rust_callee::gen_rust_callee_code;

pub use crate::codec::{FbConfig, FbConfigs, PbConfigs};

mod go_caller;
mod codec;
mod rust_callee;

pub struct BuildConfig {
    pub go_out_dir: PathBuf,
    pub rust_out_dir: PathBuf,
    pub pb_configs: PbConfigs,
    pub fb_configs: FbConfigs,
}


#[derive(Debug)]
pub struct Report {
    pub rust_c_header_filename: String,
    pub rust_c_lib_filename: String,
}


pub fn build_files(config: BuildConfig) {
    fs::create_dir_all(&config.go_out_dir).unwrap();
    fs::create_dir_all(&config.rust_out_dir).unwrap();
    codec::gen_flatbuf_code(&config);
    codec::gen_protobuf_code(&config);
    let report = rust_callee::gen_rust_callee_code();
    go_caller::gen_go_caller_code(&config, &report);
    gen_gen_sh(&config, &report);
}


fn gen_gen_sh(config: &BuildConfig, report: &Report) {
    let go_out_dir = config.go_out_dir.to_str().unwrap();
    let mut f = fs::File::create("gen.sh").expect("Couldn't create gen.sh");
    f.write("#!/bin/bash\n\n".as_bytes()).unwrap();
    f.write(format!("mkdir -p {}\n", go_out_dir).as_bytes()).unwrap();
    f.write(format!("cp -rf {} {}\n", report.rust_c_header_filename, go_out_dir).as_bytes()).unwrap();
    f.write(format!("cp -rf {} {}\n", report.rust_c_lib_filename, go_out_dir).as_bytes()).unwrap();
    f.flush().unwrap();
    let mut permissions = f.metadata().unwrap().permissions();
    permissions.set_mode(0o777);
    if let Err(ref e) = f.set_permissions(permissions) {
        println!("failed to set permissions of build.sh: {}", e);
    }
}


#[test]
fn test() {
    env::set_var("CARGO_PKG_NAME", "demo");
    env::set_var("CARGO_MANIFEST_DIR", "/Users/henrylee2cn/rust/fcplug/demo");
    env::set_var("OUT_DIR", "/Users/henrylee2cn/rust/fcplug/target/debug/build/demo-f8ee84ac19343019/out");

    let _ = rust_callee::gen_rust_callee_code();
}
