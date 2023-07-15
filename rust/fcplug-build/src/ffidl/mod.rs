use std::cell::RefCell;
use std::fs;
use std::ops::DerefMut;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

use anyhow::anyhow;
use pilota_build::{CodegenBackend, Context, DefId, MakeBackend, Output, rir::Enum, rir::Message, rir::Method, rir::NewType, rir::Service};
use pilota_thrift_parser::{File, Item, parser::Parser};

use crate::ffidl::gen_go::GoCodegenBackend;
use crate::ffidl::gen_rust::RustCodegenBackend;

mod gen_rust;
mod gen_go;

#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub file_path: PathBuf,
    pub rust_out_path: PathBuf,
    pub go_out_path: PathBuf,
    pub go_root_path: Option<PathBuf>,
    pub impl_rustffi_for_unit_struct: Option<UnitLikeStructPath>,
}

#[derive(Debug, Clone, Copy)]
pub struct UnitLikeStructPath(pub &'static str);

#[derive(Debug, Clone)]
pub struct FFIDL {
    config: Arc<Config>,
    go_code: Arc<RefCell<String>>,
}

unsafe impl Send for FFIDL {}

impl FFIDL {
    fn generate(config: Config) -> anyhow::Result<()> {
        Self { config: Arc::new(config), go_code: Arc::new(RefCell::new(String::new())) }
            .check_idl()?
            .gen_rust_and_go()
    }
    fn check_idl(self) -> anyhow::Result<Self> {
        let file_source = fs::read_to_string(&self.config.file_path)?.leak();
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
    fn gen_rust_and_go(self) -> anyhow::Result<()> {
        fs::create_dir_all(&self.config.rust_out_path.parent().unwrap())?;

        self.go_code.borrow_mut().push_str(&format!(
            "package {}\n",
            self.config.go_out_path.parent().unwrap().file_name().unwrap().to_str().unwrap()));

        pilota_build::Builder::thrift_with_backend(self.clone())
            .ignore_unused(true)
            .compile(
                [&self.config.file_path],
                Output::File(self.config.rust_out_path.clone()),
            );

        fs::write(&self.config.go_out_path, self.go_code.borrow().as_str())?;

        let output = Command::new(self.config.go_root_path.as_ref().map_or("gofmt".to_string(), |p| p.join("gofmt").to_str().unwrap().to_string()))
            .arg("-l").arg("-w").arg(self.config.go_out_path.to_str().unwrap())
            .output()
            .unwrap();
        if !output.status.success() {
            eprintln!("{:?}", output);
        }
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
            go_code: self.go_code,
        }
    }
}

#[derive(Clone)]
pub struct FFIDLBackend {
    config: Arc<Config>,
    context: Arc<Context>,
    rust: RustCodegenBackend,
    go: GoCodegenBackend,
    go_code: Arc<RefCell<String>>,
}

unsafe impl Send for FFIDLBackend {}

impl CodegenBackend for FFIDLBackend {
    fn cx(&self) -> &Context {
        self.context.as_ref()
    }
    fn codegen_struct_impl(&self, def_id: DefId, stream: &mut String, s: &Message) {
        self.rust.codegen_struct_impl(def_id, stream, s);
        self.go.codegen_struct_impl(def_id, self.go_code.borrow_mut().deref_mut(), s);
    }
    fn codegen_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        self.rust.codegen_service_impl(def_id, stream, s);
        self.go.codegen_service_impl(def_id, self.go_code.borrow_mut().deref_mut(), s);
    }
    fn codegen_service_method(&self, service_def_id: DefId, method: &Method) -> String {
        self.rust.codegen_service_method(service_def_id, method)
    }
    fn codegen_enum_impl(&self, _def_id: DefId, _stream: &mut String, _e: &Enum) {}
    fn codegen_newtype_impl(&self, _def_id: DefId, _stream: &mut String, _t: &NewType) {}
}


#[cfg(test)]
mod tests {
    use crate::ffidl::{Config, FFIDL, UnitLikeStructPath};

    #[test]
    fn test_thriftast() {
        FFIDL::generate(Config {
            file_path: "/Users/henrylee2cn/rust/fcplug/ffidl_demo/ffidl.thrift".into(),
            rust_out_path: "/Users/henrylee2cn/rust/fcplug/ffidl_demo/src/gen/ffidl.rs".into(),
            go_out_path: "/Users/henrylee2cn/rust/fcplug/ffidl_demo/src/gen/ffidl.go".into(),
            // impl_rustffi_for_unit_struct: None,
            go_root_path: Some("/Users/henrylee2cn/.gvm/gos/go1.19.9/bin".into()),
            impl_rustffi_for_unit_struct: Some(UnitLikeStructPath("crate::gen::MyImplRustFfi")),
        })
            .unwrap();
    }

    #[test]
    fn test_gen_header() {
        cbindgen::Builder::new()
            .with_crate("/Users/henrylee2cn/rust/fcplug/ffidl_demo")
            .with_src("/Users/henrylee2cn/rust/fcplug/rust/fcplug/src/ctypes.rs")
            .with_language(cbindgen::Language::C)
            .generate()
            .unwrap()
            .write_to_file("/Users/henrylee2cn/rust/fcplug/ffidl_demo/src/gen/ffidl.h");
    }
}
