use std::fs;
use std::path::PathBuf;

use anyhow::anyhow;
use pilota_build::Output;
use pilota_thrift_parser::{File, Item, parser::Parser};

use crate::ffidl::gen_rust::RustMakeBackend;

mod gen_rust;

#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub file_path: PathBuf,
    pub rust_out_path: PathBuf,
    pub impl_rustffi_for_unit_struct: Option<&'static str>,
}

#[derive(Debug)]
pub struct FFIDL {
    config: Config,
    file_source: *mut str,
}

impl Drop for FFIDL {
    fn drop(&mut self) {
        let _ = String::from(unsafe { &mut *self.file_source });
    }
}

impl FFIDL {
    fn generate(config: Config) -> anyhow::Result<()> {
        Self { file_source: fs::read_to_string(&config.file_path)?.leak(), config }
            .check_idl()?
            .gen_rust_and_go()?;
        Ok(())
    }
    fn check_idl(self) -> anyhow::Result<Self> {
        let (_, file) = <File as Parser>::parse(unsafe { &*self.file_source })?;
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
    fn gen_rust_and_go(self) -> anyhow::Result<Self> {
        fs::create_dir_all(&self.config.rust_out_path.parent().unwrap())?;
        pilota_build::Builder::thrift_with_backend(RustMakeBackend { config: self.config.clone() })
            .ignore_unused(true)
            .compile(
                [&self.config.file_path],
                Output::File(self.config.rust_out_path.clone()),
            );
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::ffidl::{Config, FFIDL};

    #[test]
    fn test_thriftast() {
        FFIDL::generate(Config {
            file_path: "/Users/henrylee2cn/rust/fcplug/ffidl_demo/ffidl.thrift"
                .into(),
            rust_out_path:
            "/Users/henrylee2cn/rust/fcplug/ffidl_demo/src/gen/ffidl.rs".into(),
            // impl_rustffi_for_unit_struct: None,
            impl_rustffi_for_unit_struct: Some("crate::gen::MyImplRustFfi"),
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
