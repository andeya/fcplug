use std::{env, fs};
use std::cell::RefCell;
use std::env::temp_dir;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

use anyhow::anyhow;
use pilota_build::{CodegenBackend, Context, DefId, MakeBackend, Output, ProtobufBackend, rir::Enum, rir::Message, rir::Method, rir::NewType, rir::Service};
use pilota_build::db::RirDatabase;
use pilota_build::ir::ItemKind;
use pilota_build::parser::{Parser, ProtobufParser};
use pilota_build::plugin::{AutoDerivePlugin, PredicateResult};
use pilota_build::rir::{Arg, Item};
use pilota_build::ty::TyKind;

use crate::ffidl::gen_go::GoCodegenBackend;
use crate::ffidl::gen_rust::RustCodegenBackend;

mod gen_rust;
mod gen_go;

const CGOBIN: &'static str = "cgobin";
#[cfg(not(debug_assertions))]
const MODE: &'static str = "release";
#[cfg(debug_assertions)]
const MODE: &'static str = "debug";

#[derive(Debug, Clone)]
pub struct Config {
    pub idl_file_path: PathBuf,
    pub output_dir: PathBuf,
    pub impl_ffi_for_unitstruct: Option<UnitLikeStructPath>,
    pub go_root_path: Option<PathBuf>,
    pub go_mod_path: &'static str,
    pub goffi_impl_of_object: Option<GoObjectPath>,
}

impl Config {
    fn dir_name(&self) -> String {
        self.output_dir.file_name().unwrap().to_os_string().into_string().unwrap()
    }
    fn go_mod_name(&self) -> String {
        self.go_mod_path.rsplit_once("/").expect("invalid go mod path").1.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct UnitLikeStructPath(pub &'static str);

#[derive(Debug, Clone)]
pub struct GoObjectPath {
    pub import: String,
    pub object_ident: String,
}

#[derive(Debug, Clone)]
pub(crate) struct FFIDL {
    config: Arc<Config>,
    rust_c_header_name_base: Arc<RefCell<String>>,
    go_c_header_name_base: Arc<RefCell<String>>,
    clib_dir: Arc<RefCell<PathBuf>>,
    go_pkg_code: Arc<RefCell<String>>,
    go_main_code: Arc<RefCell<String>>,
}

unsafe impl Send for FFIDL {}

impl FFIDL {
    pub(crate) fn generate(config: Config) -> anyhow::Result<()> {
        if config.go_mod_name() != config.dir_name() {
            return Err(anyhow!("The directory name of 'output_dir' is inconsistent with the mod name of 'go_mod_path'"));
        }
        Self {
            config: Arc::new(config),
            go_pkg_code: Arc::new(RefCell::new(String::new())),
            go_main_code: Arc::new(RefCell::new(String::new())),
            rust_c_header_name_base: Arc::new(RefCell::new("".to_string())),
            go_c_header_name_base: Arc::new(RefCell::new("".to_string())),
            clib_dir: Arc::new(RefCell::new(Default::default())),
        }
            .check_idl()?
            .set_clib_paths()
            .gen_rust_and_go()
    }
    fn include_dir(&self) -> PathBuf {
        self.config.idl_file_path.parent().unwrap().to_path_buf()
    }
    fn check_idl(self) -> anyhow::Result<Self> {
        let mut parser = ProtobufParser::default();
        // let mut parser = ThriftParser::default();
        Parser::include_dirs(&mut parser, vec![self.include_dir()]);
        Parser::input(&mut parser, &self.config.idl_file_path);
        let file = Parser::parse(parser).files.pop().unwrap();
        if !file.uses.is_empty() {
            return Err(anyhow!("Does not support Protobuf 'import'."));
        }
        for item in &file.items {
            match &item.kind {
                ItemKind::Message(_) => {}
                ItemKind::Service(service_item) => {
                    match service_item.name.to_lowercase().as_str() {
                        "goffi" | "rustffi" => {}
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
        *self.rust_c_header_name_base.borrow_mut() = env::var("CARGO_PKG_NAME").unwrap().replace("-", "_");
        *self.go_c_header_name_base.borrow_mut() = "go_".to_string() + &env::var("CARGO_PKG_NAME").unwrap().replace("-", "_");
        *self.clib_dir.borrow_mut() = env::var("CARGO_TARGET_DIR").map_or_else(
            |_|
                PathBuf::from(env::var("CARGO_WORKSPACE_DIR")
                    .unwrap_or(env::var("CARGO_MANIFEST_DIR").unwrap_or_default())
                ).join("target"),
            PathBuf::from,
        )
            .join(MODE);
        self
    }

    fn gen_rust_and_go(self) -> anyhow::Result<()> {
        fs::create_dir_all(&self.config.output_dir.join(CGOBIN))?;
        let pkg_name = self.config.dir_name();

        let temp_dir = temp_dir();
        let temp_idl = temp_dir.join(pkg_name.clone() + ".proto");
        fs::copy(&self.config.idl_file_path, &temp_idl)?;
        let output = new_shell_cmd()
            .arg(format!(
                "protoc --proto_path={} --go_out {} {}",
                temp_dir.to_str().unwrap(),
                self.config.output_dir.to_str().unwrap(),
                temp_idl.to_str().unwrap(),
            ))
            .output()
            .unwrap();
        if !output.status.success() {
            eprintln!("gen_protobuf_code: {:?}", output)
        }

        let import_gen_pkg = self.config.go_mod_path;
        let rust_c_header_name_base = self.rust_c_header_name_base.borrow().to_string();
        let rust_c_lib_dir = self.clib_dir.borrow().as_os_str().to_str().unwrap().to_string();
        *self.go_main_code.borrow_mut() = format!(r###"package main

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
	_ gen.ResultCode
)

        "###);

        *self.go_pkg_code.borrow_mut() = format!(
            r###"package {pkg_name}
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

            "###);

        pilota_build::Builder::protobuf_with_backend(self.clone())
            // pilota_build::Builder::thrift_with_backend(self.clone())
            .include_dirs(vec![self.include_dir()])
            .plugin(AutoDerivePlugin::new(Arc::new(["#[derive(::serde::Serialize, ::serde::Deserialize)]".into()]), |_| PredicateResult::GoOn))
            .ignore_unused(true)
            .compile(
                [&self.config.idl_file_path],
                Output::File(self.config.output_dir.join("mod.rs")),
            );

        self.gen_rust_clib()?;

        fs::write(
            &self.config.output_dir.join(pkg_name + ".go"),
            self.go_pkg_code.borrow().as_str(),
        )?;
        fs::write(
            &self.config.output_dir.join("go.mod"),
            format!(r###"module {}

            go 1.19

            require (
                github.com/andeya/gust v1.5.2
                github.com/bytedance/sonic v1.9.2
                github.com/golang/protobuf v1.5.3
            )

            "###, self.config.go_mod_path),
        )?;
        fs::write(
            self.config.output_dir.join(CGOBIN).join("main.go"),
            self.go_main_code.borrow().as_str(),
        )?;

        let output = Command::new(self.config.go_root_path.as_ref().map_or("gofmt".to_string(), |p| p.join("gofmt").to_str().unwrap().to_string()))
            .arg("-l")
            .arg("-w")
            .arg(self.config.output_dir.to_str().unwrap())
            .output()?;
        if !output.status.success() {
            eprintln!("{:?}", output);
        }

        let output = new_shell_cmd()
            .current_dir(&self.config.output_dir)
            .arg("go mod tidy").arg(self.config.output_dir.to_str().unwrap())
            .output()?;
        if !output.status.success() {
            eprintln!("{:?}", output);
        }

        self.gen_go_clib()?;
        Ok(())
    }
    fn gen_rust_clib(&self) -> anyhow::Result<()> {
        cbindgen::Builder::new()
            // .with_crate(env::var("CARGO_MANIFEST_DIR").unwrap())
            .with_src(self.config.output_dir.join("mod.rs"))
            // .with_src("/Users/henrylee2cn/rust/fcplug/rust/fcplug/src/lib.rs")
            .with_language(cbindgen::Language::C)
            .with_after_include(r###"
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

"###)
            .generate()?
            .write_to_file(self.clib_dir.borrow().join(self.rust_c_header_name_base.borrow().to_string() + ".h"));
        Ok(())
    }
    fn gen_go_clib(&self) -> anyhow::Result<()> {
        let output = new_shell_cmd()
            .arg(format!(
                "CGO_ENABLED=1 go build -buildmode=c-archive -o {} {}",
                self.clib_dir.borrow().join("lib".to_string() + &self.go_c_header_name_base.borrow() + ".a").to_str().unwrap(),
                self.config.output_dir.join(CGOBIN).to_str().unwrap(),
            ))
            .output()
            .unwrap();
        if !output.status.success() {
            return Err(anyhow!(format!("{:?}",output)));
        }
        println!("cargo:rustc-link-search={}", self.clib_dir.borrow().to_str().unwrap());
        println!("cargo:rustc-link-lib={}", self.go_c_header_name_base.borrow());
        println!(
            "cargo:rerun-if-changed={}",
            self.clib_dir.borrow().join("lib".to_string() + &self.go_c_header_name_base.borrow() + ".h").to_str().unwrap(),
        );
        Ok(())
    }
}


impl MakeBackend for FFIDL {
    type Target = FFIDLBackend;

    fn make_backend(self, context: Context) -> Self::Target {
        let protobuf = ProtobufBackend::new(context.clone());
        let context = Arc::new(context);
        FFIDLBackend {
            rust: RustCodegenBackend {
                config: self.config.clone(),
                context: Cx(context.clone()),
            },
            go: GoCodegenBackend {
                config: self.config.clone(),
                context: Cx(context.clone()),
                go_pkg_code: self.go_pkg_code.clone(),
                go_main_code: self.go_main_code.clone(),
            },
            protobuf,
            context: Cx(context),
            config: self.config,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct FFIDLBackend {
    config: Arc<Config>,
    context: Cx,
    rust: RustCodegenBackend,
    go: GoCodegenBackend,
    protobuf: ProtobufBackend,
}

unsafe impl Send for FFIDLBackend {}

#[derive(Clone)]
pub(crate) struct Cx(Arc<Context>);

enum ServiceType {
    RustFfi,
    GoFfi,
}

impl Cx {
    fn service_type(&self, service_def_id: DefId) -> ServiceType {
        match self.rust_name(service_def_id).to_lowercase().as_str() {
            "rustffi" => ServiceType::RustFfi,
            "goffi" => ServiceType::GoFfi,
            _ => { unreachable!() }
        }
    }
    fn is_empty_ty(&self, kind: &TyKind) -> bool {
        match kind {
            TyKind::Path(path) => {
                if let Item::Message(m) = self.item(path.did).unwrap().as_ref() {
                    m.fields.is_empty()
                } else {
                    false
                }
            }
            TyKind::Void => true,
            _ => false
        }
    }
}

impl Deref for Cx {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl FFIDLBackend {
    fn fix_empty_params(&self, method: &Method) -> Method {
        let mut method = method.clone();
        method.args = method.args.into_iter().filter(|arg| !self.context.is_empty_ty(&arg.ty.kind)).collect::<Vec<Arc<Arg>>>();
        if self.context.is_empty_ty(&method.ret.kind) {
            method.ret.kind = TyKind::Void;
        }
        method
    }
}

impl CodegenBackend for FFIDLBackend {
    fn cx(&self) -> &Context {
        self.context.0.as_ref()
    }
    fn codegen_struct_impl(&self, def_id: DefId, stream: &mut String, s: &Message) {
        self.protobuf.codegen_struct_impl(def_id, stream, s)
    }
    fn codegen_service_impl(&self, service_def_id: DefId, stream: &mut String, s: &Service) {
        let mut s = s.clone();
        s.methods = s.methods.iter().map(|method| Arc::new(self.fix_empty_params(method))).collect::<Vec<Arc<Method>>>();
        self.protobuf.codegen_service_impl(service_def_id, stream, &s);
        self.rust.codegen_service_impl(service_def_id, stream, &s);
        self.go.codegen(service_def_id, &s)
    }
    fn codegen_service_method(&self, service_def_id: DefId, method: &Method) -> String {
        let method = self.fix_empty_params(method);
        self.protobuf.codegen_service_method(service_def_id, &method);
        self.rust.codegen_service_method(service_def_id, &method)
    }
    fn codegen_enum_impl(&self, def_id: DefId, stream: &mut String, e: &Enum) {
        self.protobuf.codegen_enum_impl(def_id, stream, e);
    }
    fn codegen_newtype_impl(&self, def_id: DefId, stream: &mut String, t: &NewType) {
        self.protobuf.codegen_newtype_impl(def_id, stream, t);
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

#[cfg(test)]
mod tests {
    use crate::ffidl::{Config, FFIDL, UnitLikeStructPath};

    #[test]
    fn test_idl() {
        FFIDL::generate(Config {
            idl_file_path: "/Users/henrylee2cn/rust/fcplug/demo/ffidl.proto".into(),
            output_dir: "/Users/henrylee2cn/rust/fcplug/demo/src/gen".into(),
            impl_ffi_for_unitstruct: Some(UnitLikeStructPath("crate::Test")),
            go_mod_path: "github.com/andeya/fcplug/demo/src/gen",
            go_root_path: Some("/Users/henrylee2cn/.gvm/gos/go1.19.9/bin".into()),
            goffi_impl_of_object: None,
        })
            .unwrap();
    }
}
