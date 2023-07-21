use fcplug_build::{Config, generate_code, UnitLikeStructPath};

fn main() {
    generate_code(Config {
        idl_file_path: "./ffidl.proto".into(),
        output_dir: "./src/gen".into(),
        impl_ffi_for_unitstruct: Some(UnitLikeStructPath("crate::Test")),
        // impl_ffi_for_unitstruct: None,
        go_mod_path: "github.com/andeya/fcplug/demo/src/gen",
        go_root_path: Some("/Users/henrylee2cn/.gvm/gos/go1.19.9/bin".into()),
        goffi_impl_of_object: None,
    })
        .unwrap();
}
