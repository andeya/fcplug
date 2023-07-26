#![allow(unused_imports)]

use fcplug_build::{Config, generate_code, UnitLikeStructPath};

fn main() {
    generate_code(Config {
        idl_file: "./echo.proto".into(),
        go_root_path: Some("/Users/henrylee2cn/.gvm/gos/go1.18.10".into()),
        go_mod_parent: "github.com/andeya/fcplug/samples",
        rust_unitstruct_impl: Some(UnitLikeStructPath("crate::ImplFfi")), // Unit-like struct implementing two Rust traits, RustFfi and GoFfi
        target_crate_dir: None,
    })
        .unwrap();
}
