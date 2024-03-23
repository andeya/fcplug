#![allow(unused_imports)]

use fcplug_build::{generate_code, Config, UnitLikeStructPath};

fn main() {
    generate_code(Config {
        idl_file: "./echo.thrift".into(),
        go_root_path: None,
        go_mod_parent: "github.com/andeya/fcplug/samples",
        target_crate_dir: None,
        use_goffi_cdylib: false,
        add_clib_to_git: false,
    });
}
