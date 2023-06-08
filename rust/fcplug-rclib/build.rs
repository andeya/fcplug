use fcplug_build::{build_files, GoProtoCodeConfig, RustProtoCodeConfig};

fn main() {
    build_files(vec![
        RustProtoCodeConfig {
            out_dir: "./src/".into(),
            inputs: vec!["../../fcplug.proto".into()],
            include: Some("../../".into()),
            customize: None,
        }
    ], GoProtoCodeConfig {
        out_dir: "../../go/gocall/".to_string(),
        filename: "../../fcplug.proto".to_string(),
    });
}
