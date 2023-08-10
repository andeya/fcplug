#![allow(unused_imports)]

use fcplug_build::{generate_code, Config, UnitLikeStructPath};

fn main() {
    // println!("cargo:rustc-link-search=/Users/henrylee2cn/rust/fcplug/target/debug");
    // println!("cargo:rustc-link-lib=go_echo");
    generate_code(Config {
        idl_file: "./echo.proto".into(),
        target_crate_dir: None,
        go_root_path: None,
        // go_root_path: Some("/Users/henrylee2cn/.gvm/gos/go1.18.10".into()),
        go_mod_parent: "github.com/andeya/fcplug/samples",
    })
    .unwrap();
}
