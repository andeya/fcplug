use fcplug_build::{Config, generate_code, UnitLikeStructPath};

fn main() {
    generate_code(Config {
        idl_file: "./ffidl.proto".into(),
        output_dir: "./src/gen".into(),
        go_root: Some("/Users/henrylee2cn/.gvm/gos/go1.19.9/bin".into()),
        go_mod: "github.com/andeya/fcplug/demo/src/gen",
        go_object_impl: None,
        rust_unitstruct_impl: Some(UnitLikeStructPath("crate::Test")),
    })
        .unwrap();
}
