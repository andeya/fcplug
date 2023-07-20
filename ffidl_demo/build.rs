use fcplug_build::UnitLikeStructPath;

fn main() {
    fcplug_build::generate_code(fcplug_build::Config {
        idl_file_path: "./ffidl.proto".into(),
        output_dir: "./src/gen".into(),
        impl_ffi_for_unitstruct: Some(UnitLikeStructPath("crate::Test")),
        // rustffi_impl_of_unit_struct: None,
        go_mod_path: "github.com/andeya/fcplug/ffidl_demo/src/gen",
        go_root_path: Some("/Users/henrylee2cn/.gvm/gos/go1.19.9/bin".into()),
        goffi_impl_of_object: None,
    })
        .unwrap();
}
