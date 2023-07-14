fn main() {
    use fcplug_build::{BuildConfig, FbConfig, FbConfigs, PbConfigs};
    fcplug_build::build_files(BuildConfig::new().go_mod_dir("./")
        .pb_configs(PbConfigs {
            inputs: vec!["idl.proto".into()],
            includes: vec!["./".into()],
            rust_customize: None,
        })
        .fb_configs(FbConfigs {
            inputs: vec!["idl.fbs".into()],
            configs: vec![
                FbConfig::Rust,
                FbConfig::Go,
            ],
        })
    )
}
