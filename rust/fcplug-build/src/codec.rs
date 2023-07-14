use std::path::{Path, PathBuf};

use crate::{BuildConfig, GenRustForLang, new_cmd};

pub type PbRustCustomize = protobuf_codegen::Customize;

#[derive(Default)]
pub struct PbConfigs {
    pub inputs: Vec<PathBuf>,
    pub includes: Vec<PathBuf>,
    pub rust_customize: Option<PbRustCustomize>,
}

pub enum FbConfig {
    Rust,
    Go,
}

#[derive(Default)]
pub struct FbConfigs {
    pub inputs: Vec<PathBuf>,
    pub configs: Vec<FbConfig>,
}

pub(crate) fn gen_flatbuf_code(config: &BuildConfig) {
    let inputs = config.fb_configs.inputs.iter().map(|v| v.as_path()).collect::<Vec<&Path>>();
    for conf in &config.fb_configs.configs {
        match conf {
            FbConfig::Rust => {
                flatc_rust::run(flatc_rust::Args {
                    lang: "rust",
                    inputs: &inputs,
                    out_dir: config.rust_out_dir(GenRustForLang::Go).as_path(),
                    ..Default::default()
                }).expect("failed to generate flatbuf rust code");
            }
            FbConfig::Go => {
                let go_out_dir = config.go_out_dir();
                flatc_rust::run(flatc_rust::Args {
                    lang: "go",
                    inputs: &inputs,
                    out_dir: go_out_dir.parent().unwrap(),
                    extra: &["--go-namespace", go_out_dir.file_name().unwrap().to_str().unwrap()],
                    ..Default::default()
                }).expect("failed to generate flatbuf go code");
            }
        }
    }
}

pub(crate) fn gen_protobuf_code(config: &BuildConfig) {
    protobuf_codegen::Codegen::new()
        .protoc()
        .out_dir(&config.rust_out_dir(GenRustForLang::Go))
        .includes(&config.pb_configs.includes)
        .inputs(&config.pb_configs.inputs)
        .customize(config.pb_configs.rust_customize
            .clone().unwrap_or_default()
            .gen_mod_rs(false)
            .generate_accessors(true)
        )
        .run()
        .expect("Unable to generate proto file");

    let output = new_cmd()
        .arg(format!(
            "protoc{} --go_out {} {}",
            config.pb_configs.includes
                .iter()
                .map(|i| " --proto_path=".to_string() + i.to_str().unwrap())
                .collect::<String>(),
            config.go_out_dir().to_str().unwrap(),
            config.pb_configs.inputs
                .iter()
                .map(|i| i.to_str().unwrap())
                .collect::<Vec<&str>>()
                .join(" "),
        ))
        .output()
        .unwrap();
    if !output.status.success() {
        eprintln!("gen_protobuf_code: {:?}", output)
    }
}
