fn main() {
    use fcplug_build::{BuildConfig, FbConfig, FbConfigs, PbConfigs};
    fcplug_build::build_files(BuildConfig {
        go_out_dir: "go_gen".into(),
        rust_out_dir: "src".into(),
        pb_configs: PbConfigs {
            inputs: vec!["idl.proto".into()],
            includes: vec![],
            rust_customize: None,
        },
        fb_configs: FbConfigs {
            inputs: vec!["idl.fbs".into()],
            configs: vec![
                FbConfig::Rust,
                FbConfig::Go,
            ],
        },
    })
}
