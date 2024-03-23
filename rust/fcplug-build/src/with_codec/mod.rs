use std::fs;
use std::process::Command;

use crate::config::IdlType;
use crate::generator::{Generator, ImportPkg, MidOutput};
use crate::{deal_output, deal_result, CODE_IO};

mod gen_go_codec;
mod gen_rust_codec;

impl Generator {
    pub(crate) fn _gen_code(self) -> MidOutput {
        self.gen_go_codec_code();
        MidOutput {
            rust_clib_includes: r###"
typedef int8_t ResultCode;

typedef struct Buffer {
  uint8_t *ptr;
  uintptr_t len;
  uintptr_t cap;
} Buffer;

typedef struct RustFfiResult {
  ResultCode code;
  struct Buffer data;
} RustFfiResult;

typedef struct GoFfiResult {
  ResultCode code;
  uintptr_t data_ptr;
} GoFfiResult;

void free_buffer(struct Buffer buf);
uintptr_t leak_buffer(struct Buffer buf);

"###
            .to_string(),
            mod_requires: vec![
                "github.com/andeya/gust@v1.5.2".to_string(),
                "github.com/bytedance/sonic@latest".to_string(),
                match self.config.idl_type {
                    IdlType::Proto | IdlType::ProtoNoCodec => "google.golang.org/protobuf@v1.26.0",
                    IdlType::Thrift | IdlType::ThriftNoCodec => "github.com/apache/thrift@v0.13.0",
                }
                .to_string(),
            ],
            imports: vec![
                ImportPkg {
                    in_main: true,
                    in_lib: false,
                    import_path: "github.com/andeya/gust".to_string(),
                    use_code: "var _ gust.EnumResult[any, any]".to_string(),
                },
                ImportPkg {
                    in_main: false,
                    in_lib: true,
                    import_path: "github.com/andeya/gust/valconv".to_string(),
                    use_code: "var _ valconv.ReadonlyBytes".to_string(),
                },
                ImportPkg {
                    in_main: false,
                    in_lib: true,
                    import_path: "github.com/bytedance/sonic".to_string(),
                    use_code: "var _ = sonic.Marshal".to_string(),
                },
                match self.config.idl_type {
                    IdlType::Proto | IdlType::ProtoNoCodec => ImportPkg {
                        in_main: false,
                        in_lib: true,
                        import_path: "google.golang.org/protobuf/proto".to_string(),
                        use_code: "var _ = proto.Marshal".to_string(),
                    },
                    IdlType::Thrift | IdlType::ThriftNoCodec => ImportPkg::default(),
                },
                ImportPkg {
                    in_main: true,
                    in_lib: false,
                    import_path: self.config.gomod_path,
                    use_code: format!("var _ {}.ResultCode", self.config.gomod_name),
                },
            ],
        }
    }
    fn gen_go_codec_code(&self) {
        match self.config.idl_type {
            IdlType::Proto | IdlType::ProtoNoCodec => {
                deal_output(
                    Command::new("protoc")
                        .arg(format!(
                            "--proto_path={}",
                            self.config.target_out_dir.to_str().unwrap()
                        ))
                        .arg(format!(
                            "--go_out={}",
                            self.config.pkg_dir.to_str().unwrap()
                        ))
                        .arg(self.config.idl_file.as_os_str())
                        .output(),
                );
            }
            IdlType::Thrift | IdlType::ThriftNoCodec => {
                deal_output(
                    Command::new("thriftgo")
                        .arg(format!("-g=go"))
                        .arg(format!(
                            "-o={}",
                            self.config
                                .target_out_dir
                                .join("gen-thrift")
                                .to_str()
                                .unwrap()
                        ))
                        .arg(self.config.idl_file.as_os_str())
                        .output(),
                );
                let go_mod_name = &self.config.gomod_name;
                deal_result(
                    CODE_IO,
                    fs::rename(
                        self.config
                            .target_out_dir
                            .join("gen-thrift")
                            .join(&go_mod_name)
                            .join(&format!("{go_mod_name}.go")),
                        self.config
                            .pkg_dir
                            .join(&format!("{go_mod_name}.thrift.go")),
                    ),
                );
            }
        };
    }
}
