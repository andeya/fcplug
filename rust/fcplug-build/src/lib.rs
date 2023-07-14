#![feature(fs_try_exists)]

use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub use crate::codec::{FbConfig, FbConfigs, PbConfigs};

mod codec;
mod go_call_rust;
mod rust_call_go;
mod ffidl;

#[derive(Default)]
pub struct BuildConfig {
    pub(crate) go_mod_dir: PathBuf,
    pub(crate) pb_configs: PbConfigs,
    pub(crate) fb_configs: FbConfigs,
    go_out_dir: RefCell<Option<PathBuf>>,
    rust_out_dir: RefCell<Option<PathBuf>>,
}

impl BuildConfig {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn go_mod_dir<T: Into<PathBuf>>(mut self, go_mod_dir: T) -> Self {
        self.go_mod_dir = go_mod_dir.into();
        self
    }
    pub fn pb_configs(mut self, pb_configs: PbConfigs) -> Self {
        self.pb_configs = pb_configs;
        self
    }
    pub fn fb_configs(mut self, fb_configs: FbConfigs) -> Self {
        self.fb_configs = fb_configs;
        self
    }
}

enum GenRustForLang {
    Go,
}

impl BuildConfig {
    fn go_out_dir(&self) -> PathBuf {
        if self.go_out_dir.borrow().is_none() {
            let p = self.go_mod_dir
                .join("internal")
                .join("rsffi_gen");
            fs::remove_dir_all(&p).unwrap();
            fs::create_dir_all(&p).unwrap();
            self.go_out_dir.replace(Some(p.canonicalize().unwrap()));
        }
        self.go_out_dir.clone().into_inner().unwrap()
    }
    fn rust_out_dir(&self, lang: GenRustForLang) -> PathBuf {
        if self.rust_out_dir.borrow().is_none() {
            // let mut p = PathBuf::from(env::var("OUT_DIR").unwrap());
            let mut p = PathBuf::new();
            match lang {
                GenRustForLang::Go => {
                    p.push("goffi_gen");
                    p.push("src");
                }
            }
            // fs::remove_dir_all(&p).unwrap();
            fs::create_dir_all(&p).unwrap();
            self.rust_out_dir.replace(Some(p.canonicalize().unwrap()));
        }
        self.rust_out_dir.clone().into_inner().unwrap()
    }
}

#[derive(Debug)]
pub struct Report {
    pub rust_c_header_filename: String,
    pub rust_c_lib_filename: String,
}

pub fn build_files(config: BuildConfig) {
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
