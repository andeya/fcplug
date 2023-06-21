use std::path::{Path, PathBuf};
use std::process::Command;

use crate::BuildConfig;

pub type PbRustCustomize = protoc_rust::Customize;

pub struct PbConfigs {
    pub inputs: Vec<PathBuf>,
    pub includes: Vec<PathBuf>,
    pub rust_customize: Option<PbRustCustomize>,
}

pub enum FbConfig {
    Rust,
    Go,
}

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
                    out_dir: config.rust_out_dir.as_path(),
                    ..Default::default()
                }).expect("failed to generate flatbuf rust code");
            }
            FbConfig::Go => {
                let out_dir = config.go_out_dir.canonicalize().unwrap();
                flatc_rust::run(flatc_rust::Args {
                    lang: "go",
                    inputs: &inputs,
                    out_dir: out_dir.parent().unwrap(),
                    extra: &["--go-namespace", out_dir.file_name().unwrap().to_str().unwrap()],
                    ..Default::default()
                }).expect("failed to generate flatbuf go code");
            }
        }
    }
}

pub(crate) fn gen_protobuf_code(config: &BuildConfig) {
    protoc_rust::Codegen::new()
        .out_dir(&config.rust_out_dir)
        .includes(&config.pb_configs.includes)
        .inputs(&config.pb_configs.inputs)
        .customize(config.pb_configs.rust_customize.as_ref().unwrap_or(&PbRustCustomize {
            carllerche_bytes_for_bytes: Some(true),
            generate_accessors: Some(true),
            ..Default::default()
        }).clone())
        .run()
        .expect("Unable to generate proto file");

    let go_out_dir = config.go_out_dir.to_str().unwrap();

    let mut param = ("sh", "-c");
    if cfg!(target_os = "windows") {
        param.0 = "cmd";
        param.1 = "/c";
    }
    Command::new(param.0).arg(param.1)
        .arg(format!(
            "protoc{} --go_out {} {}",
            config.pb_configs.includes
                .iter()
                .map(|i| " --proto_path=".to_string() + i.to_str().unwrap())
                .collect::<String>(),
            go_out_dir,
            config.pb_configs.inputs
                .iter()
                .map(|i| i.to_str().unwrap())
                .collect::<Vec<&str>>()
                .join(" "),
        ))
        .output()
        .unwrap();
}
