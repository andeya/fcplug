use std::{env, fs};
use std::cell::RefCell;
use std::env::temp_dir;
use std::path::PathBuf;
use std::process::{Command, Output as CmdOutput};
use std::sync::Arc;

use anyhow::anyhow;
use pilota_build::fmt::fmt_file;
use pilota_build::ir::ItemKind;
use pilota_build::Output;
use pilota_build::parser::{Parser, ProtobufParser};
use pilota_build::plugin::{AutoDerivePlugin, PredicateResult};

pub use config::{Config, GoObjectPath, UnitLikeStructPath};

mod gen_go;
mod gen_rust;
mod config;
mod make_backend;

#[cfg(not(debug_assertions))]
const MODE: &'static str = "release";
#[cfg(debug_assertions)]
const MODE: &'static str = "debug";

#[derive(Debug, Clone)]
pub(crate) struct FFIDL {
    config: Arc<Config>,
    rust_c_header_name_base: Arc<RefCell<String>>,
    go_c_header_name_base: Arc<RefCell<String>>,
    clib_dir: Arc<RefCell<PathBuf>>,
    go_pkg_code: Arc<RefCell<String>>,
    go_main_code: Arc<RefCell<String>>,
    has_goffi: bool,
    has_rustffi: bool,
    rust_impl_rustffi_code: Arc<RefCell<String>>,
    rust_impl_goffi_code: Arc<RefCell<String>>,
    crate_modified: String,
}

unsafe impl Send for FFIDL {}

impl FFIDL {
    pub(crate) fn generate(config: Config) -> anyhow::Result<()> {
        let crate_modified = config.new_crate_modified();
        Self {
            config: Arc::new(config),
            go_pkg_code: Arc::new(RefCell::new(String::new())),
            go_main_code: Arc::new(RefCell::new(String::new())),
            rust_c_header_name_base: Arc::new(RefCell::new("".to_string())),
            go_c_header_name_base: Arc::new(RefCell::new("".to_string())),
            clib_dir: Arc::new(RefCell::new(Default::default())),
            has_goffi: false,
            has_rustffi: false,
            rust_impl_rustffi_code: Arc::new(RefCell::new("".to_string())),
            rust_impl_goffi_code: Arc::new(RefCell::new("".to_string())),
            crate_modified,
        }
            .set_clib_paths()
            .check_idl()?
            .gen_rust_and_go()
    }
    fn include_dir(&self) -> PathBuf {
        self.config.idl_file.parent().unwrap().to_path_buf()
    }
    fn check_idl(mut self) -> anyhow::Result<Self> {
        let mut parser = ProtobufParser::default();
        // let mut parser = ThriftParser::default();
        Parser::include_dirs(&mut parser, vec![self.include_dir()]);
        Parser::input(&mut parser, &self.config.idl_file);
        let file = Parser::parse(parser).files.pop().unwrap();
        if !file.uses.is_empty() {
            return Err(anyhow!("Does not support Protobuf 'import'."));
        }
        for item in &file.items {
            match &item.kind {
                ItemKind::Message(_) => {}
                ItemKind::Service(service_item) => {
                    match service_item.name.to_lowercase().as_str() {
                        "goffi" => self.has_goffi = true,
                        "rustffi" => self.has_rustffi = true,
                        _ => {
                            return Err(anyhow!(
                                "Protobuf Service name can only be: 'GoFFI', 'RustFFI'."
                            ));
                        }
                    }
                }
                _ => {
                    return Err(anyhow!(
                        "Protobuf Item '{}' not supported.",
                        format!("{:?}", item)
                            .trim_start_matches("Item { kind: ")
                            .split_once("(")
                            .unwrap()
                            .0
                            .to_lowercase()
                    ));
                }
            }
        }
        Ok(self)
    }
    fn set_clib_paths(self) -> Self {
        *self.rust_c_header_name_base.borrow_mut() =
            env::var("CARGO_PKG_NAME").unwrap().replace("-", "_");
        *self.go_c_header_name_base.borrow_mut() =
            "go_".to_string() + &env::var("CARGO_PKG_NAME").unwrap().replace("-", "_");
        *self.clib_dir.borrow_mut() = env::var("CARGO_TARGET_DIR")
            .map_or_else(
                |_| {
                    PathBuf::from(
                        env::var("CARGO_WORKSPACE_DIR")
                            .unwrap_or(env::var("CARGO_MANIFEST_DIR").unwrap_or_default()),
                    )
                        .join("target")
                },
                PathBuf::from,
            )
            .join(MODE);
        println!(
            "cargo:rerun-if-changed={}",
            self.clib_dir.borrow().to_str().unwrap(),
        );
        self
    }

    fn output_to_result(output: CmdOutput) -> anyhow::Result<()> {
        if !output.status.success() {
            return Err(anyhow!(format!("{:?}", output)));
        }
        Ok(())
    }

    fn crate_project(&self) -> anyhow::Result<()> {
        Ok(fs::create_dir_all(&self.config.go_main_dir())?)
    }
    fn gen_rust_code(&self) -> anyhow::Result<()> {
        let rust_mod_file = self.config.rust_mod_file();
        let rust_impl_file = self.config.rust_impl_file();

        pilota_build::Builder::protobuf_with_backend(self.clone())
            // pilota_build::Builder::thrift_with_backend(self.clone())
            .doc_header("// Code generated by fcplug. DO NOT EDIT.".to_string())
            .include_dirs(vec![self.include_dir()])
            .plugin(AutoDerivePlugin::new(
                Arc::new(["#[derive(::serde::Serialize, ::serde::Deserialize)]".into()]),
                |_| PredicateResult::GoOn,
            ))
            .ignore_unused(true)
            .compile([&self.config.idl_file], Output::File(rust_mod_file.clone()));

        let mut rust_code = fs::read_to_string(&rust_mod_file).unwrap();
        if !self.has_rustffi {
            rust_code.push_str(&format!("pub trait RustFfi {{}}"));
        }
        if !self.has_goffi {
            rust_code.push_str(&format!("pub trait GoFfi {{}}"));
        }
        let rust_impl_name = self.config.rust_impl_name();
        rust_code.push_str(&format!(
            r###"pub trait Ffi: RustFfi + GoFfi {{}}

        pub(crate) struct {rust_impl_name};

        impl Ffi for {rust_impl_name} {{}}
        "###
        ));
        fs::write(&rust_mod_file, rust_code).unwrap();
        fmt_file(rust_mod_file);

        if !rust_impl_file.exists() {
            let rust_impl_rustffi_code = self.rust_impl_rustffi_code.borrow();
            let rust_impl_goffi_code = self.rust_impl_goffi_code.borrow();
            fs::write(
                &rust_impl_file,
                &format!(r###"
                {rust_impl_rustffi_code}

                {rust_impl_goffi_code}
                "###
                ),
            )
                .unwrap();
            fmt_file(rust_impl_file);
        }
        Ok(())
    }

    fn gen_rust_and_go(self) -> anyhow::Result<()> {
        self.crate_project()?;
        let pkg_dir = self.config.pkg_dir();

        let temp_dir = temp_dir();
        let temp_idl = temp_dir.join(self.config.go_mod_name().clone() + ".proto");
        fs::copy(&self.config.idl_file, &temp_idl)?;
        Self::output_to_result(
            new_shell_cmd()
                .arg(format!(
                    "protoc --proto_path={} --go_opt=M{} --go_out {} {}",
                    temp_dir.to_str().unwrap(),
                    format!(
                        "{};{}",
                        pkg_dir.to_str().unwrap(),
                        self.config.go_mod_name()
                    ),
                    pkg_dir.to_str().unwrap(),
                    temp_idl.to_str().unwrap(),
                ))
                .output()?,
        )?;

        let import_gen_pkg = self.config.go_mod_path();
        let import_gen_pkg_var = format!("_ {}.ResultCode", self.config.go_mod_name());
        let rust_c_header_name_base = self.rust_c_header_name_base.borrow().to_string();
        let rust_c_lib_dir = self
            .clib_dir
            .borrow()
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string();

        *self.go_main_code.borrow_mut() = format!(
            r###"// Code generated by fcplug. DO NOT EDIT.

        package main

        /*
        #cgo CFLAGS: -I{rust_c_lib_dir}
        #cgo LDFLAGS: -L{rust_c_lib_dir} -l{rust_c_header_name_base}

        #include "{rust_c_header_name_base}.h"
        */
        import "C"
        import (
            "reflect"
            "unsafe"

            "{import_gen_pkg}"
            "github.com/andeya/gust"
        )

        // main function is never called by C to.
        func main() {{}}

        var (
            _ reflect.SliceHeader
            _ unsafe.Pointer
            _ gust.EnumResult[any, any]
            {import_gen_pkg_var}
        )

        "###
        );

        let go_mod_name = self.config.go_mod_name();
        *self.go_pkg_code.borrow_mut() = format!(
            r###"// Code generated by fcplug. DO NOT EDIT.

            package {go_mod_name}
            /*
            #cgo CFLAGS: -I{rust_c_lib_dir}
            #cgo LDFLAGS: -L{rust_c_lib_dir} -l{rust_c_header_name_base}

            #include "{rust_c_header_name_base}.h"
            */
            import "C"

            import (
                "errors"
                "fmt"
                "reflect"
                "unsafe"

                "github.com/andeya/gust/valconv"
                "github.com/bytedance/sonic"
                "github.com/golang/protobuf/proto"
            )

            var (
                _ = errors.New
                _ = fmt.Sprintf
                _ reflect.SliceHeader
                _ unsafe.Pointer
                _ valconv.ReadonlyBytes
                _ = sonic.Marshal
                _ = proto.Marshal
            )

            "###
        );

        self.gen_rust_code()?;
        self.gen_rust_clib()?;

        fs::write(
            &self.config.go_lib_file(),
            self.go_pkg_code.borrow().as_str(),
        )?;

        let go_mod_file = self.config.go_mod_file();
        if !go_mod_file.exists() {
            fs::write(
                &go_mod_file,
                format!(
                    r###"module {}

            go 1.18

            require (
                github.com/andeya/gust v1.5.2
                github.com/bytedance/sonic v1.9.2
                github.com/golang/protobuf v1.5.3
            )

            "###,
                    self.config.go_mod_path()
                ),
            )?;
        }

        if self.has_goffi {
            fs::write(
                self.config.go_main_file(),
                self.go_main_code.borrow().as_str(),
            )?;
            if !self.config.go_main_impl_file().exists() {
                fs::write(
                    self.config.go_main_impl_file(),
                    format!(
                        r###"package main

            func init() {{
                // TODO: Replace with your own implementation, then re-execute `cargo build`
                GlobalGoFfi = _UnimplementedGoFfi{{}}
            }}

            "###
                    ),
                )?;
            }

            println!(
                "cargo:rerun-if-changed={}",
                self.config.go_main_impl_file().to_str().unwrap(),
            );
        }

        Self::output_to_result(
            Command::new(self.config.go_cmd_path("gofmt"))
                .arg("-l")
                .arg("-w")
                .arg(pkg_dir.to_str().unwrap())
                .output()?,
        )
            .unwrap();

        Self::output_to_result(
            new_shell_cmd()
                .current_dir(&pkg_dir)
                .arg(self.config.go_cmd_path("go") + " mod tidy")
                .arg(pkg_dir.to_str().unwrap())
                .output()?,
        )
            .unwrap();

        self.gen_go_clib();

        println!(
            "cargo:rerun-if-changed={}",
            self.clib_dir.borrow().to_str().unwrap(),
        );
        Ok(())
    }
    fn gen_rust_clib(&self) -> anyhow::Result<()> {
        cbindgen::Builder::new()
            .with_src(self.config.rust_mod_file())
            .with_language(cbindgen::Language::C)
            .with_after_include(
                r###"
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

"###,
            )
            .generate()?
            .write_to_file(
                self.clib_dir
                    .borrow()
                    .join(self.rust_c_header_name_base.borrow().to_string() + ".h"),
            );
        Ok(())
    }
    fn gen_go_clib(&self) {
        if !self.has_goffi {
            return;
        }
        let clib_name = self
            .clib_dir
            .borrow()
            .join("lib".to_string() + &self.go_c_header_name_base.borrow() + ".a");
        let output = Self::output_to_result(
            new_shell_cmd()
                .env("CGO_ENABLED", "1")
                .env(
                    "GOROOT",
                    self.config.go_root_path.clone().unwrap_or_default(),
                )
                .arg(format!(
                    "{} build -buildmode=c-archive -o {} {}",
                    self.config.go_cmd_path("go"),
                    clib_name.to_str().unwrap(),
                    self.config.go_main_dir().to_str().unwrap(),
                ))
                .output()
                .unwrap(),
        );
        println!(
            "cargo:rustc-link-search={}",
            self.clib_dir.borrow().to_str().unwrap()
        );
        println!(
            "cargo:rustc-link-lib={}",
            self.go_c_header_name_base.borrow()
        );
        let mut re_execute = false;
        if !clib_name.exists() {
            if let Err(e) = output {
                println!(
                    "cargo:warning=failed to execute 'go build -buildmode=c-archive ...', {:?}",
                    e
                );
            }
            re_execute = true;
        }
        let crate_modified_path = self.clib_dir.borrow().join("crate_modified");
        if fs::read_to_string(&crate_modified_path).unwrap_or_default() != self.crate_modified {
            fs::write(crate_modified_path, self.crate_modified.as_str()).unwrap();
            re_execute = true
        }
        if re_execute {
            println!("cargo:warning=It is recommended to re-execute 'cargo build' to ensure the correctness of '{}'", clib_name.file_name().unwrap().to_str().unwrap());
        }
        println!(
            "cargo:rerun-if-changed={}",
            self.config.pkg_dir().to_str().unwrap(),
        );
    }
}

fn new_shell_cmd() -> Command {
    let mut param = ("sh", "-c");
    if cfg!(target_os = "windows") {
        param.0 = "cmd";
        param.1 = "/c";
    }
    let mut cmd = Command::new(param.0);
    cmd.arg(param.1);
    cmd
}

// #[cfg(test)]
// mod tests {
//     use crate::ffidl::{Config, FFIDL, UnitLikeStructPath};
//
//     #[test]
//     fn test_idl() {
//         FFIDL::generate(Config {
//             idl_file: "/Users/henrylee2cn/rust/fcplug/demo/ffidl.proto".into(),
//             rust_unitstruct_impl: Some(UnitLikeStructPath("crate::Test")),
//             go_mod_parent: "github.com/andeya/fcplug",
//             go_root_path: Some("/Users/henrylee2cn/.gvm/gos/go1.19.9/bin".into()),
//             target_crate_dir: Some("/Users/henrylee2cn/rust/fcplug/demo".into()),
//         })
//             .unwrap();
//     }
// }
