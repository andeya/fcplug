use fcplug_build::{Config, generate_code, UnitLikeStructPath};

fn main() {
    generate_code(Config {
        idl_file: "./ffidl.proto".into(),
        go_root_path: Some("/Users/henrylee2cn/.gvm/gos/go1.19.9/bin".into()),
        go_mod_parent: "github.com/andeya/fcplug",
        rust_unitstruct_impl: Some(UnitLikeStructPath("crate::Test")),
        target_crate_dir: None,
    })
        .unwrap();
}
