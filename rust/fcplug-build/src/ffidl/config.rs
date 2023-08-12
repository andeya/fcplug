use std::{env, fs};
use std::path::PathBuf;
use std::str::FromStr;

use pilota_build::ir::ItemKind;
use pilota_build::parser::{Parser, ProtobufParser, ThriftParser};

use crate::{BUILD_MODE, exit_with_warning, GEN_MODE, GenMode};

const CGOBIN: &'static str = "cgobin";

#[derive(Debug, Clone)]
pub struct Config {
    pub idl_file: PathBuf,
    /// Target crate directory for code generation
    pub target_crate_dir: Option<PathBuf>,
    /// go command dir, default to find from $GOROOT > $PATH
    pub go_root_path: Option<PathBuf>,
    pub go_mod_parent: &'static str,
}

pub(crate) enum IdlType {
    Proto,
    Thrift,
    ProtoNoCodec,
    ThriftNoCodec,
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

#[derive(Default, Debug, Clone)]
pub(crate) struct FfiSet {
    pub(crate) has_goffi: bool,
    pub(crate) has_rustffi: bool,
}


impl Config {
    pub(crate) fn idl_type(&self) -> IdlType {
        match self.idl_file.extension().unwrap().to_str().unwrap() {
            "thrift" => match GEN_MODE {
                GenMode::Codec => IdlType::Thrift,
                GenMode::NoCodec => IdlType::ThriftNoCodec,
            },
            "proto" => match GEN_MODE {
                GenMode::Codec => IdlType::Proto,
                GenMode::NoCodec => IdlType::ProtoNoCodec,
            },
            x => {
                println!("cargo:warning=unsupported idl file extension: {x}");
                std::process::exit(404);
            }
        }
    }
    pub(crate) fn pkg_dir(&self) -> PathBuf {
        if let Some(target_crate_dir) = &self.target_crate_dir {
            target_crate_dir.clone()
        } else {
            env::var("CARGO_MANIFEST_DIR").unwrap().into()
        }
    }
    fn pkg_name_prefix(&self) -> String {
        let idl_name = self.idl_file.file_name().unwrap().to_str().unwrap();
        idl_name
            .rsplit_once(".")
            .map_or(idl_name, |(idl_name, _)| idl_name)
            .replace(".", "_")
            .replace("-", "_")
            .trim_start_matches("_")
            .to_string()
            .trim_end_matches("_")
            .to_string()
    }
    fn file_name_base(&self) -> String {
        let pkg_name_prefix = self.pkg_name_prefix();
        format!("{pkg_name_prefix}_gen")
    }
    pub(crate) fn go_mod_name(&self) -> String {
        self.pkg_dir()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".", "_")
            .replace("-", "_")
            .trim_start_matches("_")
            .to_string()
            .trim_end_matches("_")
            .to_string()
    }
    pub(crate) fn go_mod_path(&self) -> String {
        format!(
            "{}/{}",
            self.go_mod_parent.trim_end_matches("/"),
            self.go_mod_name()
        )
    }
    pub(crate) fn go_cmd_path(&self, cmd: &'static str) -> String {
        if let Some(go_root_path) = &self.go_root_path {
            go_root_path
                .join("bin")
                .join(cmd)
                .to_str()
                .unwrap()
                .to_string()
        } else if let Ok(go_root_path) = env::var("GOROOT") {
            PathBuf::from_str(&go_root_path)
                .unwrap()
                .join("bin")
                .join(cmd)
                .to_str()
                .unwrap()
                .to_string()
        } else {
            cmd.to_string()
        }
    }
    pub(crate) fn rust_mod_dir(&self) -> PathBuf {
        self.pkg_dir()
            .join("src")
            .join(self.pkg_name_prefix() + "_ffi")
    }
    pub(crate) fn rust_mod_gen_file(&self) -> PathBuf {
        self.rust_mod_dir().join(self.file_name_base() + ".rs")
    }
    pub(crate) fn rust_mod_impl_file(&self) -> PathBuf {
        self.rust_mod_dir().join("mod.rs")
    }
    pub(crate) fn rust_mod_impl_name(&self) -> &'static str {
        "FfiImpl"
    }
    pub(crate) fn rust_mod_gen_name(&self) -> String {
        self.file_name_base()
    }
    pub(crate) fn go_mod_file(&self) -> PathBuf {
        self.pkg_dir().join("go.mod")
    }
    pub(crate) fn go_lib_file(&self) -> PathBuf {
        self.pkg_dir().join(self.file_name_base() + ".go")
    }
    pub(crate) fn go_main_dir(&self) -> PathBuf {
        self.pkg_dir().join(CGOBIN)
    }
    pub(crate) fn go_main_file(&self) -> PathBuf {
        self.go_main_dir().join("clib_goffi_gen.go")
    }
    pub(crate) fn go_main_impl_file(&self) -> PathBuf {
        self.go_main_dir().join("clib_goffi_impl.go")
    }
    pub(crate) fn include_dir(&self) -> PathBuf {
        self.idl_file.parent().unwrap().to_path_buf()
    }
    pub(crate) fn check_idl(&self) -> FfiSet {
        let mut ret = match self.idl_type() {
            IdlType::Proto | IdlType::ProtoNoCodec => {
                let mut parser = ProtobufParser::default();
                Parser::include_dirs(&mut parser, vec![self.include_dir()]);
                Parser::input(&mut parser, &self.idl_file);
                let (descs, ret) = parser.parse_and_typecheck();
                for desc in descs {
                    if desc.package.is_some() {
                        exit_with_warning(-1, "IDL-Check: The 'package' should not be configured");
                    }
                    if let Some(opt) = desc.options.as_ref() {
                        if opt.go_package.is_some() {
                            exit_with_warning(-1, "IDL-Check: The 'option go_package' should not be configured");
                        }
                    }
                }
                ret
            }
            IdlType::Thrift | IdlType::ThriftNoCodec => {
                let mut parser = ThriftParser::default();
                Parser::include_dirs(&mut parser, vec![self.include_dir()]);
                Parser::input(&mut parser, &self.idl_file);
                let ret = parser.parse();
                ret
            }
        };

        let file = ret.files.pop().unwrap();
        if !file.uses.is_empty() {
            match self.idl_type() {
                IdlType::Proto | IdlType::ProtoNoCodec => exit_with_warning(-1, "IDL-Check: Does not support Protobuf 'import'."),
                IdlType::Thrift | IdlType::ThriftNoCodec => exit_with_warning(-1, "IDL-Check: Does not support Thrift 'include'."),
            }
        }
        let mut ffi_set = FfiSet::default();
        for item in &file.items {
            match &item.kind {
                ItemKind::Message(_) => {}
                ItemKind::Service(service_item) => match service_item.name.to_lowercase().as_str() {
                    "goffi" => ffi_set.has_goffi = true,
                    "rustffi" => ffi_set.has_rustffi = true,
                    _ => exit_with_warning(-1, "IDL-Check: Protobuf Service name can only be: 'GoFFI', 'RustFFI'."),
                }
                _ => match self.idl_type() {
                    IdlType::Proto | IdlType::ProtoNoCodec => exit_with_warning(
                        -1,
                        format!("IDL-Check: Protobuf Item '{}' not supported.", format!("{:?}", item)
                            .trim_start_matches("Item { kind: ")
                            .split_once("(")
                            .unwrap()
                            .0
                            .to_lowercase()),
                    ),
                    IdlType::Thrift | IdlType::ThriftNoCodec => exit_with_warning(
                        -1,
                        format!("Thrift Item '{}' not supported.", format!("{:?}", item)
                            .split_once("(")
                            .unwrap()
                            .0
                            .to_lowercase()
                        )),
                }
            }
        }
        ffi_set
    }
    pub(crate) fn create_crate_dir_all(&self) {
        let _ = self.
            _create_crate_dir_all()
            .inspect_err(|e| exit_with_warning(-2, format!("failed create crate directories to {e:?}")));
    }
    fn _create_crate_dir_all(&self) -> anyhow::Result<()> {
        fs::create_dir_all(&self.go_main_dir())?;
        fs::create_dir_all(&self.rust_mod_dir())?;
        Ok(())
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct CLibConfig {
    pub(crate) rust_c_header_name_base: String,
    pub(crate) go_c_header_name_base: String,
    pub(crate) clib_dir: PathBuf,
    pub(crate) crate_modified: String,
}

impl CLibConfig {
    pub(crate) fn new(config: &Config) -> CLibConfig {
        let mut c = CLibConfig::default();
        c.crate_modified = Self::new_crate_modified(config);
        c.rust_c_header_name_base = env::var("CARGO_PKG_NAME").unwrap().replace("-", "_");
        c.go_c_header_name_base = "go_".to_string() + &env::var("CARGO_PKG_NAME").unwrap().replace("-", "_");

        let target_dir = env::var("CARGO_TARGET_DIR")
            .map_or_else(
                |_| {
                    PathBuf::from(env::var("CARGO_WORKSPACE_DIR")
                        .unwrap_or_else(|_| {
                            let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap_or_default());
                            let mdir = env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
                            if out_dir.starts_with(&mdir) {
                                mdir
                            } else {
                                let mut p = PathBuf::new();
                                let mut coms = Vec::new();
                                let mut start = false;
                                for x in out_dir.components().rev() {
                                    if !start && x.as_os_str() == "target" {
                                        start = true;
                                        continue;
                                    }
                                    if start {
                                        coms.insert(0, x);
                                    }
                                }
                                for x in coms {
                                    p = p.join(x);
                                }
                                p.to_str().unwrap().to_string()
                            }
                        }))
                        .join("target")
                },
                PathBuf::from,
            );

        let full_target_dir = target_dir.join(env::var("TARGET").unwrap());
        c.clib_dir = if full_target_dir.is_dir() && PathBuf::from(env::var("OUT_DIR").unwrap())
            .canonicalize()
            .unwrap()
            .starts_with(full_target_dir.canonicalize().unwrap()) {
            full_target_dir
        } else {
            target_dir
        }.join(BUILD_MODE);
        c
    }
    fn new_crate_modified(config: &Config) -> String {
        walkdir::WalkDir::new(config.pkg_dir())
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                if entry
                    .path()
                    .extension()
                    .map(|ext| ext == "go" || ext == "rs" || ext == "toml" || ext == "proto")
                    .unwrap_or_default()
                {
                    if let Ok(metadata) = entry.metadata() {
                        return metadata.is_file();
                    }
                };
                return false;
            })
            .fold(String::new(), |acc, m| {
                let digest = md5::compute(fs::read(m.path()).unwrap());
                format!("{acc}|{digest:x}")
            })
    }
    pub(crate) fn update_crate_modified(&self) -> bool {
        let crate_modified_path = self.clib_dir.join("crate_modified");
        if fs::read_to_string(&crate_modified_path).unwrap_or_default() != self.crate_modified {
            fs::write(crate_modified_path, self.crate_modified.as_str()).unwrap();
            return true;
        }
        return false;
    }
    pub(crate) fn rust_clib_a_path(&self) -> PathBuf {
        self.clib_dir.join("lib".to_string() + self.rust_c_header_name_base.as_str() + ".a")
    }
    pub(crate) fn rust_clib_h_path(&self) -> PathBuf {
        self.clib_dir.join(self.rust_c_header_name_base.clone() + ".h")
    }
    pub(crate) fn go_clib_a_path(&self) -> PathBuf {
        self.clib_dir.join("lib".to_string() + &self.go_c_header_name_base.as_str() + ".a")
    }
    pub(crate) fn rustc_link(&self) {
        println!("cargo:rustc-link-search={}", self.clib_dir.to_str().unwrap());
        println!("cargo:rustc-link-lib={}", self.go_c_header_name_base);
    }
}
