#![allow(unused_imports)]

fn main() {
    use fcplug_build::{generate_code, Config, UnitLikeStructPath};
    generate_code(Config {
        idl_file: "./echo.proto".into(),
        target_crate_dir: None,
        go_root_path: None,
        // go_root_path: Some("/Users/henrylee2cn/.gvm/gos/go1.18.10".into()),
        go_mod_parent: "github.com/andeya/fcplug/samples",
        use_goffi_cdylib: false,
        add_clib_to_git: false,
    });
}
