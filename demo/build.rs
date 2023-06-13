fn main() {
    use fcplug_build::{FbConfig, FbConfigs, PbGoConfig, BuildConfig, PbRustConfig};
    fcplug_build::build_files(BuildConfig {
        go_out_dir: "go_gen".into(),
        rust_out_dir: "src".into(),
        pb_rust_configs: vec![
            PbRustConfig {
                inputs: vec!["idl.proto".into()],
                include: None,
                customize: None,
            }
        ],
        pb_go_config: Some(PbGoConfig {
            filename: "idl.proto".to_string(),
        }),
        fb_configs: FbConfigs {
            inputs: vec!["idl.fbs".into()],
            configs: vec![
                FbConfig::Rust,
                FbConfig::Go,
            ],
        },
    })
}
