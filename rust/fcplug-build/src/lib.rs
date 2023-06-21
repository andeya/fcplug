use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub use crate::codec::{FbConfig, FbConfigs, PbConfigs};

mod codec;
mod go_call_rust;
mod rust_call_go;

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
    go_call_rust::gen_code(&config);
    rust_call_go::gen_code(&config);
}

fn new_cmd() -> Command {
    let mut param = ("sh", "-c");
    if cfg!(target_os = "windows") {
        param.0 = "cmd";
        param.1 = "/c";
    }
    let mut cmd = Command::new(param.0);
    cmd.arg(param.1);
    cmd
}

#[test]
fn test() {
    std::env::set_var("CARGO_PKG_NAME", "demo");
    std::env::set_var("CARGO_MANIFEST_DIR", "/Users/henrylee2cn/rust/fcplug/demo");
    std::env::set_var("OUT_DIR", "/Users/henrylee2cn/rust/fcplug/target/debug/build/demo-f8ee84ac19343019/out");

    let _ = go_call_rust::gen_rust_callee_code();
}
