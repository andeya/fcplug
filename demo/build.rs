use fcplug_build::{build_files, GoProtoCodeConfig, RustProtoCodeConfig};

fn main() {
    build_files(vec![
        RustProtoCodeConfig {
            out_dir: "./src/".into(),
            inputs: vec!["./idl.proto".into()],
            include: None,
            customize: None,
        }
    ], GoProtoCodeConfig {
        out_dir: "go_gen".to_string(),
        filename: "idl.proto".to_string(),
    });
}
