use std::fs;
use std::path::PathBuf;

use anyhow::anyhow;
use pilota_build::Output;
use pilota_thrift_parser::{File, Item, parser::Parser, Service, Struct};

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
    structs: Vec<Struct>,
    go_service: Option<Service>,
    rust_service: Option<Service>,
    file_source: *mut str,
    go_gen_code: String,
    rust_gen_code: String,
}

impl Drop for FFIDL {
    fn drop(&mut self) {
        let _ = String::from(unsafe { &mut *self.file_source });
    }
}

impl FFIDL {
    fn generate(config: Config) -> anyhow::Result<()> {
        Self::parse_from_file(config)?.gen_go()?.gen_rust()?;
        Ok(())
    }
    fn gen_go(self) -> anyhow::Result<Self> {
        Ok(self)
    }
    fn gen_rust(self) -> anyhow::Result<Self> {
        fs::create_dir_all(&self.config.rust_out_path.parent().unwrap())?;
        pilota_build::Builder::thrift_with_backend(RustMakeBackend { config: self.config.clone() })
            .ignore_unused(true)
            .compile(
                [&self.config.file_path],
                Output::File(self.config.rust_out_path.clone()),
            );
        Ok(self)
    }
    fn parse_from_file(config: Config) -> anyhow::Result<Self> {
        let mut ffidl = Self {
            structs: vec![],
            go_service: None,
            rust_service: None,
            file_source: fs::read_to_string(&config.file_path)?.leak(),
            config,
            go_gen_code: "".to_string(),
            rust_gen_code: "".to_string(),
        };
        let (_, file) = <File as Parser>::parse(unsafe { &*ffidl.file_source })?;
        for item in file.items {
            match item {
                Item::Struct(struct_item) => ffidl.structs.push(struct_item),
                Item::Service(service_item) => {
                    match service_item.name.as_str().to_lowercase().as_str() {
                        "goffi" => {
                            let _ = ffidl.go_service.insert(service_item);
                        }
                        "rustffi" => {
                            let _ = ffidl.rust_service.insert(service_item);
                        }
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
        Ok(ffidl)
    }
}
