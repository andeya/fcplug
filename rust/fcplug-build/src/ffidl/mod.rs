use std::{env, fs};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

use anyhow::anyhow;
use pilota_build::{CodegenBackend, Context, DefId, MakeBackend, Output, rir::Enum, rir::Message, rir::Method, rir::NewType, rir::Service};
use pilota_thrift_parser::{File, Item, parser::Parser};

use crate::ffidl::ctypes_code::write_ctypes_file;
use crate::ffidl::gen_go::GoCodegenBackend;
use crate::ffidl::gen_rust::RustCodegenBackend;

mod gen_rust;
mod gen_go;
mod ctypes_code;

#[cfg(not(debug_assertions))]
const MODE: &'static str = "release";
#[cfg(debug_assertions)]
const MODE: &'static str = "debug";

#[derive(Debug, Clone)]
pub struct Config {
    pub idl_file_path: PathBuf,
    pub output_dir: PathBuf,
    pub rustffi_impl_of_unit_struct: Option<UnitLikeStructPath>,
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
    go_pkg_code: Arc<RefCell<String>>,
    go_main_code: Arc<RefCell<String>>,
    rust_c_header_name_base: Arc<RefCell<String>>,
    rust_c_lib_dir: Arc<RefCell<PathBuf>>,
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
            rust_c_lib_dir: Arc::new(RefCell::new(Default::default())),
        }
            .check_idl()?
            .set_rust_clib_path()
            .gen_rust_and_go()
    }
    fn check_idl(self) -> anyhow::Result<Self> {
        let file_source = fs::read_to_string(&self.config.idl_file_path)?.leak();
        let (_, file) = <File as Parser>::parse(file_source)?;
        for item in file.items {
            match item {
                Item::Struct(_) => {}
                Item::Service(service_item) => {
                    match service_item.name.as_str().to_lowercase().as_str() {
                        "goffi" | "rustffi" => {}
                        _ => {
                            return Err(anyhow!(
                                "Thrift Service name can only be: 'GoFFI', 'RustFFI'."
                            ));
                        }
                    }
                }
                _ => {
                    return Err(anyhow!(
                        "Thrift Item '{}' not supported.",
                        format!("{:?}", item)
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
    fn set_rust_clib_path(self) -> Self {
        *self.rust_c_header_name_base.borrow_mut() = env::var("CARGO_PKG_NAME").unwrap().replace("-", "_");
        *self.rust_c_lib_dir.borrow_mut() = env::var("CARGO_TARGET_DIR").map_or_else(
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
        fs::create_dir_all(&self.config.output_dir.join("cgobin"))?;

        let pkg_name = self.config.dir_name();

        let rust_c_header_name_base = self.rust_c_header_name_base.borrow().to_string();
        let rust_c_lib_dir = self.rust_c_lib_dir.borrow().as_os_str().to_str().unwrap().to_string();
        self.go_pkg_code.borrow_mut().push_str(&format!(
            r###"package {pkg_name}
            /*
            #cgo CFLAGS: -I{rust_c_lib_dir}
            #cgo LDFLAGS: -L{rust_c_lib_dir} -l{rust_c_header_name_base}

            #include "{rust_c_header_name_base}.h"
            */
            import "C"

            import (
                "unsafe"

                "github.com/andeya/fcplug/go/ctypes"
            )

            var (
                _ unsafe.Pointer
                _ ctypes.C_DynArray[any]
            )

            "###));

        pilota_build::Builder::thrift_with_backend(self.clone())
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

            replace github.com/andeya/fcplug/go/ctypes => /Users/henrylee2cn/rust/fcplug/go/ctypes

            "###, self.config.go_mod_path),
        )?;
        fs::write(
            self.config.output_dir.join("cgobin").join("main.go"),
            self.go_main_code.borrow().as_str(),
        )?;

        let output = Command::new(self.config.go_root_path.as_ref().map_or("gofmt".to_string(), |p| p.join("gofmt").to_str().unwrap().to_string()))
            .arg("-l")
            .arg("-w")
            .arg(self.config.output_dir.to_str().unwrap())
            .output()
            .unwrap();
        if !output.status.success() {
            eprintln!("{:?}", output);
        }

        let output = new_shell_cmd()
            .current_dir(&self.config.output_dir)
            .arg("go mod tidy").arg(self.config.output_dir.to_str().unwrap())
            .output()
            .unwrap();
        if !output.status.success() {
            eprintln!("{:?}", output);
        }

        Ok(())
    }
    fn gen_rust_clib(&self) -> anyhow::Result<()> {
        cbindgen::Builder::new()
            // .with_crate(env::var("CARGO_MANIFEST_DIR").unwrap())
            .with_src(self.config.output_dir.join("mod.rs"))
            .with_src(write_ctypes_file()?)
            .with_language(cbindgen::Language::C)
            .generate()?
            .write_to_file(self.rust_c_lib_dir.borrow().join(self.rust_c_header_name_base.borrow().to_string() + ".h"));
        Ok(())
    }
}


impl MakeBackend for FFIDL {
    type Target = FFIDLBackend;

    fn make_backend(self, context: Context) -> Self::Target {
        let context = Arc::new(context);
        FFIDLBackend {
            rust: RustCodegenBackend { config: self.config.clone(), context: context.clone() },
            go: GoCodegenBackend { config: self.config.clone(), context: context.clone() },
            config: self.config,
            context,
            go_pkg_code: self.go_pkg_code,
            go_main_code: self.go_main_code,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct FFIDLBackend {
    config: Arc<Config>,
    context: Arc<Context>,
    rust: RustCodegenBackend,
    go: GoCodegenBackend,
    go_pkg_code: Arc<RefCell<String>>,
    go_main_code: Arc<RefCell<String>>,
}

unsafe impl Send for FFIDLBackend {}

impl CodegenBackend for FFIDLBackend {
    fn cx(&self) -> &Context {
        self.context.as_ref()
    }
    fn codegen_struct_impl(&self, def_id: DefId, stream: &mut String, s: &Message) {
        self.rust.codegen_struct_impl(def_id, stream, s);
        self.go.codegen_struct_impl(def_id, self.go_pkg_code.borrow_mut().deref_mut(), s);
    }
    fn codegen_service_impl(&self, service_def_id: DefId, stream: &mut String, s: &Service) {
        self.rust.codegen_service_impl(service_def_id, stream, s);
        self.go.codegen_service_interface(service_def_id, self.go_pkg_code.borrow_mut().deref_mut(), s);
        match self.context.rust_name(service_def_id).to_lowercase().as_str() {
            "goffi" => self.go.codegen_service_export(service_def_id, self.go_main_code.borrow_mut().deref_mut(), s),
            _ => {}
        }
    }
    fn codegen_service_method(&self, service_def_id: DefId, method: &Method) -> String {
        self.rust.codegen_service_method(service_def_id, method)
    }
    fn codegen_enum_impl(&self, _def_id: DefId, _stream: &mut String, _e: &Enum) {}
    fn codegen_newtype_impl(&self, _def_id: DefId, _stream: &mut String, _t: &NewType) {}
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
    fn test_thriftast() {
        FFIDL::generate(Config {
            idl_file_path: "/Users/henrylee2cn/rust/fcplug/ffidl_demo/ffidl.thrift".into(),
            output_dir: "/Users/henrylee2cn/rust/fcplug/ffidl_demo/src/gen".into(),
            rustffi_impl_of_unit_struct: Some(UnitLikeStructPath("crate::gen::MyImplRustFfi")),
            go_mod_path: "github.com/andeya/fcplug/ffidl_demo/src/gen",
            go_root_path: Some("/Users/henrylee2cn/.gvm/gos/go1.19.9/bin".into()),
            goffi_impl_of_object: None,
        })
            .unwrap();
    }
}
