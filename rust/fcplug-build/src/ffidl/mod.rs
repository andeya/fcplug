use std::{env, fs};
use std::cell::RefCell;
use std::env::temp_dir;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::{Command, Output as CmdOutput};
use std::sync::Arc;
use std::cell::Cell;
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
    pub idl_file: PathBuf,
    /// Target crate directory for code generation
    pub target_crate_dir: Option<PathBuf>,
    pub go_root_path: Option<PathBuf>,
    pub go_mod_parent: &'static str,
    /// Unit-like struct implementing two Rust traits, RustFfi and GoFfi
    pub rust_unitstruct_impl: Option<UnitLikeStructPath>,
}

impl Config {
    fn pkg_dir(&self) -> PathBuf {
        if let Some(target_crate_dir) = &self.target_crate_dir {
            target_crate_dir.clone()
        } else {
            env::var("CARGO_MANIFEST_DIR").unwrap().into()
        }
    }
    fn file_name_base(&self) -> String {
        let idl_name = self.idl_file.file_name().unwrap().to_str().unwrap();
        let pkg_name_prefix = idl_name
            .rsplit_once(".")
            .map_or(idl_name, |(idl_name, _)| idl_name)
            .replace(".", "_")
            .replace("-", "_")
            .trim_start_matches("_").to_string()
            .trim_end_matches("_").to_string();
        format!("{pkg_name_prefix}_gen")
    }
    fn go_mod_name(&self) -> String {
        self.pkg_dir()
            .file_name().unwrap().to_str().unwrap()
            .replace(".", "_")
            .replace("-", "_")
            .trim_start_matches("_").to_string()
            .trim_end_matches("_").to_string()
    }
    fn go_mod_path(&self) -> String {
        format!("{}/{}", self.go_mod_parent.trim_end_matches("/"), self.go_mod_name())
    }
    fn go_cmd_path(&self, cmd: &'static str) -> String {
        if let Some(go_root_path) = &self.go_root_path {
            go_root_path.join(cmd).to_str().unwrap().to_string()
        } else {
            cmd.to_string()
        }
    }
    fn rust_mod_file(&self) -> PathBuf {
        self.pkg_dir().join("src").join(self.file_name_base() + ".rs")
    }
    fn go_mod_file(&self) -> PathBuf {
        self.pkg_dir().join("go.mod")
    }
    fn go_lib_file(&self) -> PathBuf {
        self.pkg_dir().join(self.file_name_base() + ".go")
    }
    fn go_main_dir(&self) -> PathBuf {
        self.pkg_dir().join(CGOBIN)
    }
    fn go_main_file(&self) -> PathBuf {
        self.go_main_dir().join("clib_goffi_gen.go")
    }
    fn go_main_impl_file(&self) -> PathBuf {
        self.go_main_dir().join("clib_goffi_impl.go")
    }
}

/// unit-like struct path, e.g. `::mycrate::Abc`
#[derive(Debug, Clone)]
pub struct UnitLikeStructPath(pub &'static str);

#[derive(Debug, Clone)]
pub struct GoObjectPath {
    /// e.g. `github.com/xxx/mypkg`
    pub import: String,
    /// e.g. `mypkg.Abc`
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
    has_goffi: bool,
}

unsafe impl Send for FFIDL {}

impl FFIDL {
    pub(crate) fn generate(config: Config) -> anyhow::Result<()> {
        Self {
            config: Arc::new(config),
            go_pkg_code: Arc::new(RefCell::new(String::new())),
            go_main_code: Arc::new(RefCell::new(String::new())),
            rust_c_header_name_base: Arc::new(RefCell::new("".to_string())),
            go_c_header_name_base: Arc::new(RefCell::new("".to_string())),
            clib_dir: Arc::new(RefCell::new(Default::default())),
            has_goffi: false,
        }
            .check_idl()?
            .set_clib_paths()
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
                        "goffi" => { self.has_goffi = true }
                        "rustffi" => {}
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
    fn has_goffi(&self) -> bool {
        self.has_goffi
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

    fn output_to_result(output: CmdOutput) -> anyhow::Result<()> {
        if !output.status.success() {
            return Err(anyhow!(format!("{:?}",output)));
        }
        Ok(())
    }

    fn crate_project(&self) -> anyhow::Result<()> {
        Ok(fs::create_dir_all(&self.config.go_main_dir())?)
    }
    fn gen_rust_and_go(self) -> anyhow::Result<()> {
        self.crate_project()?;
        let pkg_dir = self.config.pkg_dir();

        let temp_dir = temp_dir();
        let temp_idl = temp_dir.join(self.config.go_mod_name().clone() + ".proto");
        fs::copy(&self.config.idl_file, &temp_idl)?;
        Self::output_to_result(new_shell_cmd()
            .arg(format!(
                "protoc --proto_path={} --go_out {} {}",
                temp_dir.to_str().unwrap(),
                pkg_dir.to_str().unwrap(),
                temp_idl.to_str().unwrap(),
            ))
            .output()?)?;


        let import_gen_pkg = self.config.go_mod_path();
        let import_gen_pkg_var = format!("_ {}.ResultCode", self.config.go_mod_name());
        let rust_c_header_name_base = self.rust_c_header_name_base.borrow().to_string();
        let rust_c_lib_dir = self.clib_dir.borrow().as_os_str().to_str().unwrap().to_string();

        *self.go_main_code.borrow_mut() = format!(r###"// Code generated by fcplug. DO NOT EDIT.

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

        "###);

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

            "###);

        pilota_build::Builder::protobuf_with_backend(self.clone())
            // pilota_build::Builder::thrift_with_backend(self.clone())
            .doc_header("// Code generated by fcplug. DO NOT EDIT.".to_string())
            .include_dirs(vec![self.include_dir()])
            .plugin(AutoDerivePlugin::new(Arc::new(["#[derive(::serde::Serialize, ::serde::Deserialize)]".into()]), |_| PredicateResult::GoOn))
            .ignore_unused(true)
            .compile(
                [&self.config.idl_file],
                Output::File(self.config.rust_mod_file()),
            );

        self.gen_rust_clib()?;

        fs::write(
            &self.config.go_lib_file(),
            self.go_pkg_code.borrow().as_str(),
        )?;

        let go_mod_file = self.config.go_mod_file();
        if !go_mod_file.exists() {
            fs::write(
                &go_mod_file,
                format!(r###"module {}

            go 1.19

            require (
                github.com/andeya/gust v1.5.2
                github.com/bytedance/sonic v1.9.2
                github.com/golang/protobuf v1.5.3
            )

            "###, self.config.go_mod_path()),
            )?;
        }

        if self.has_goffi() {
            fs::write(
                self.config.go_main_file(),
                self.go_main_code.borrow().as_str(),
            )?;
            if !self.config.go_main_impl_file().exists() {
                fs::write(self.config.go_main_impl_file(), format!(r###"package main

            func init() {{
                // TODO: Replace with your own implementation, then re-execute `cargo build`
                GlobalGoFfi = _UnimplementedGoFfi{{}}
            }}

            "###))?;
            }
        }

        Self::output_to_result(Command::new(self.config.go_cmd_path("gofmt"))
            .arg("-l")
            .arg("-w")
            .arg(pkg_dir.to_str().unwrap())
            .output()?).unwrap();

        Self::output_to_result(new_shell_cmd()
            .current_dir(&pkg_dir)
            .arg("go mod tidy").arg(pkg_dir.to_str().unwrap())
            .output()?).unwrap();

        self.gen_go_clib().unwrap();
        Ok(())
    }
    fn gen_rust_clib(&self) -> anyhow::Result<()> {
        cbindgen::Builder::new()
            .with_src(self.config.rust_mod_file())
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
        if !self.has_goffi() {
            return Ok(());
        }
        Self::output_to_result(new_shell_cmd()
            .env("CGO_ENABLED", "1")
            .arg(format!(
                "{} build -buildmode=c-archive -o {} {}",
                self.config.go_cmd_path("go"),
                self.clib_dir.borrow().join("lib".to_string() + &self.go_c_header_name_base.borrow() + ".a").to_str().unwrap(),
                self.config.go_main_dir().to_str().unwrap(),
            ))
            .output()?)?;

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
                generated_ust: Cell::new(false),
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
            idl_file: "/Users/henrylee2cn/rust/fcplug/demo/ffidl.proto".into(),
            rust_unitstruct_impl: Some(UnitLikeStructPath("crate::Test")),
            go_mod_parent: "github.com/andeya/fcplug",
            go_root_path: Some("/Users/henrylee2cn/.gvm/gos/go1.19.9/bin".into()),
            target_crate_dir: Some("/Users/henrylee2cn/rust/fcplug/demo".into()),
        })
            .unwrap();
    }
}
