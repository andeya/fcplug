use std::fs::OpenOptions;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::{env, fs};

use pilota_build::ir::ItemKind;
use pilota_build::parser::{Parser, ProtobufParser, ThriftParser};

use crate::{
    deal_output, exit_with_warning, os_arch::get_go_os_arch_from_env, GenMode, BUILD_MODE, GEN_MODE,
};

const CGOBIN: &'static str = "cgobin";

#[derive(Default, Debug, Clone)]
pub struct Config {
    pub idl_file: PathBuf,
    /// Target crate directory for code generation
    pub target_crate_dir: Option<PathBuf>,
    /// go command dir, default to find from $GOROOT > $PATH
    pub go_root_path: Option<PathBuf>,
    pub go_mod_parent: &'static str,
    /// If use_goffi_cdylib is true, go will be compiled into a c dynamic library.
    pub use_goffi_cdylib: bool,
    /// If add_clib_to_git is true, the c lib files will be automatically added to the git version management list.
    pub add_clib_to_git: bool,
}

#[derive(Debug, Clone)]
pub(crate) enum IdlType {
    Proto,
    Thrift,
    ProtoNoCodec,
    ThriftNoCodec,
}
impl Default for IdlType {
    fn default() -> Self {
        IdlType::Proto
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

#[derive(Default, Debug, Clone)]
pub(crate) struct WorkConfig {
    config: Config,
    pub(crate) go_buildmode: &'static str,
    pub(crate) rustc_link_kind_goffi: &'static str,
    pub(crate) idl_file: PathBuf,
    pub(crate) idl_include_dir: PathBuf,
    pub(crate) idl_type: IdlType,
    pub(crate) rust_clib_name_base: String,
    pub(crate) go_clib_name_base: String,
    pub(crate) target_out_dir: PathBuf,
    pub(crate) pkg_dir: PathBuf,
    pub(crate) pkg_name: String,
    pub(crate) gomod_name: String,
    pub(crate) gomod_path: String,
    pub(crate) gomod_file: PathBuf,
    pub(crate) rust_mod_dir: PathBuf,
    pub(crate) rust_mod_gen_file: PathBuf,
    pub(crate) rust_mod_impl_file: PathBuf,
    pub(crate) rust_mod_gen_name: String,
    pub(crate) go_lib_file: PathBuf,
    pub(crate) clib_gen_dir: PathBuf,
    pub(crate) go_main_dir: PathBuf,
    pub(crate) go_main_file: PathBuf,
    pub(crate) go_main_impl_file: PathBuf,
    pub(crate) rust_clib_file: PathBuf,
    pub(crate) rust_clib_header: PathBuf,
    pub(crate) go_clib_file: PathBuf,
    pub(crate) go_clib_header: PathBuf,
    pub(crate) has_goffi: bool,
    pub(crate) has_rustffi: bool,
    pub(crate) rust_mod_impl_name: String,
    pub(crate) fingerprint: String,
    pub(crate) fingerprint_path: PathBuf,
}

impl WorkConfig {
    pub(crate) fn new(config: Config) -> WorkConfig {
        let mut c = WorkConfig::default();
        c.config = config;
        c.rust_mod_impl_name = "FfiImpl".to_string();
        c.go_buildmode = if c.config.use_goffi_cdylib {
            "c-shared"
        } else {
            "c-archive"
        };
        c.rustc_link_kind_goffi = if c.config.use_goffi_cdylib {
            "dylib"
        } else {
            "static"
        };
        c.idl_file = c.config.idl_file.clone();
        c.idl_include_dir = c.idl_file.parent().unwrap().to_path_buf();
        c.idl_type = Self::new_idl_type(&c.idl_file);
        c.rust_clib_name_base = env::var("CARGO_PKG_NAME").unwrap().replace("-", "_");
        c.go_clib_name_base = "go_".to_string() + &c.rust_clib_name_base;
        c.target_out_dir = Self::new_target_out_dir();
        c.clib_gen_dir = c.target_out_dir.clone();
        c.fingerprint_path = c.clib_gen_dir.join("fcplug.fingerprint");
        c.pkg_dir = Self::new_pkg_dir(&c.config.target_crate_dir);
        c.gomod_file = c.pkg_dir.join("go.mod");
        c.pkg_name = Self::new_pkg_name(&c.pkg_dir);
        c.gomod_name = c.pkg_name.clone();
        c.gomod_path = format!(
            "{}/{}",
            c.config.go_mod_parent.trim_end_matches("/"),
            c.gomod_name
        );
        c.rust_mod_dir = c.pkg_dir.join("src").join(c.pkg_name.clone() + "_ffi");
        c.rust_mod_gen_name = format!("{}_gen", c.pkg_name.clone());
        let file_name_base = &c.rust_mod_gen_name;
        c.rust_mod_gen_file = c.rust_mod_dir.join(format!("{file_name_base}.rs"));
        c.rust_mod_impl_file = c.rust_mod_dir.join("mod.rs");
        c.go_main_dir = c.pkg_dir.join(CGOBIN);
        let go_file_suffix = match get_go_os_arch_from_env() {
            Ok((os, arch)) => {
                format!("_{}_{}", os.as_ref(), arch.as_ref())
            }
            Err(err) => {
                println!("cargo:warning={}", err);
                String::new()
            }
        };
        c.go_lib_file = c
            .pkg_dir
            .join(format!("{file_name_base}{go_file_suffix}.go"));
        c.go_main_file = c
            .go_main_dir
            .join(format!("clib_goffi_gen{go_file_suffix}.go"));
        c.go_main_impl_file = c.go_main_dir.join("clib_goffi_impl.go");
        c.set_rust_clib_paths();
        c.set_go_clib_paths();
        c.check_go_mod_path();
        c.set_fingerprint();
        c.clean_idl();
        let _ = c
            .init_files()
            .inspect_err(|e| exit_with_warning(-2, format!("failed init files to {e:?}")));
        c.git_add();
        c
    }

    fn new_idl_type(idl_file: &PathBuf) -> IdlType {
        match idl_file.extension().unwrap().to_str().unwrap() {
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

    fn new_target_out_dir() -> PathBuf {
        let target_dir = env::var("CARGO_TARGET_DIR").map_or_else(
            |_| {
                PathBuf::from(env::var("CARGO_WORKSPACE_DIR").unwrap_or_else(|_| {
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
        if full_target_dir.is_dir()
            && PathBuf::from(env::var("OUT_DIR").unwrap())
                .canonicalize()
                .unwrap()
                .starts_with(full_target_dir.canonicalize().unwrap())
        {
            full_target_dir
        } else {
            target_dir
        }
        .join(BUILD_MODE)
        .canonicalize()
        .unwrap()
    }

    fn new_pkg_dir(target_crate_dir: &Option<PathBuf>) -> PathBuf {
        if let Some(target_crate_dir) = target_crate_dir {
            target_crate_dir.clone().canonicalize().unwrap()
        } else {
            PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
                .canonicalize()
                .unwrap()
        }
    }

    fn new_pkg_name(pkg_dir: &PathBuf) -> String {
        pkg_dir
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

    fn set_rust_clib_paths(&mut self) {
        self.rust_clib_file = self
            .clib_gen_dir
            .join(format!("lib{}.a", self.rust_clib_name_base));
        self.rust_clib_header = self
            .clib_gen_dir
            .join(format!("{}.h", self.rust_clib_name_base));
    }

    fn set_go_clib_paths(&mut self) {
        self.go_clib_file = self.clib_gen_dir.join(format!(
            "lib{}{}",
            self.go_clib_name_base,
            if self.config.use_goffi_cdylib {
                ".so"
            } else {
                ".a"
            }
        ));
        self.go_clib_header = self
            .clib_gen_dir
            .join(format!("{}.h", self.go_clib_name_base));
    }

    fn git_add(&self) {
        if !self.config.add_clib_to_git {
            return;
        }
        deal_output(
            Command::new("git")
                .arg("add")
                .arg("-f")
                .args([
                    self.go_clib_header.display().to_string(),
                    self.go_clib_file.display().to_string(),
                    self.rust_clib_header.display().to_string(),
                    self.rust_clib_file.display().to_string(),
                    self.fingerprint_path.display().to_string(),
                ])
                .output(),
        );
    }

    fn set_fingerprint(&mut self) {
        self.fingerprint = walkdir::WalkDir::new(&self.pkg_dir)
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
            });
    }
    pub(crate) fn update_fingerprint(&self) -> bool {
        if fs::read_to_string(&self.fingerprint_path).unwrap_or_default() != self.fingerprint {
            fs::write(&self.fingerprint_path, self.fingerprint.as_str()).unwrap();
            return true;
        }
        return false;
    }

    fn clean_idl(&mut self) {
        let mut ret = match self.idl_type {
            IdlType::Proto | IdlType::ProtoNoCodec => {
                let mut parser = ProtobufParser::default();
                Parser::include_dirs(&mut parser, vec![self.idl_include_dir.clone()]);
                Parser::input(&mut parser, &self.idl_file);
                let (descs, ret) = parser.parse_and_typecheck();
                for desc in descs {
                    if desc.package.is_some() {
                        exit_with_warning(-1, "IDL-Check: The 'package' should not be configured");
                    }
                    if let Some(opt) = desc.options.as_ref() {
                        if opt.go_package.is_some() {
                            exit_with_warning(
                                -1,
                                "IDL-Check: The 'option go_package' should not be configured",
                            );
                        }
                    }
                }
                ret
            }
            IdlType::Thrift | IdlType::ThriftNoCodec => {
                let mut parser = ThriftParser::default();
                Parser::include_dirs(&mut parser, vec![self.idl_include_dir.clone()]);
                Parser::input(&mut parser, &self.idl_file);
                let ret = parser.parse();
                ret
            }
        };

        let file = ret.files.pop().unwrap();
        if !file.uses.is_empty() {
            match self.idl_type {
                IdlType::Proto | IdlType::ProtoNoCodec => {
                    exit_with_warning(-1, "IDL-Check: Does not support Protobuf 'import'.")
                }
                IdlType::Thrift | IdlType::ThriftNoCodec => {
                    exit_with_warning(-1, "IDL-Check: Does not support Thrift 'include'.")
                }
            }
        }

        for item in &file.items {
            match &item.kind {
                ItemKind::Message(_) => {}
                ItemKind::Service(service_item) => {
                    match service_item.name.to_lowercase().as_str() {
                        "goffi" => self.has_goffi = true,
                        "rustffi" => self.has_rustffi = true,
                        _ => exit_with_warning(
                            -1,
                            "IDL-Check: Protobuf Service name can only be: 'GoFFI', 'RustFFI'.",
                        ),
                    }
                }
                _ => match self.idl_type {
                    IdlType::Proto | IdlType::ProtoNoCodec => exit_with_warning(
                        -1,
                        format!(
                            "IDL-Check: Protobuf Item '{}' not supported.",
                            format!("{:?}", item)
                                .trim_start_matches("Item { kind: ")
                                .split_once("(")
                                .unwrap()
                                .0
                                .to_lowercase()
                        ),
                    ),
                    IdlType::Thrift | IdlType::ThriftNoCodec => exit_with_warning(
                        -1,
                        format!(
                            "Thrift Item '{}' not supported.",
                            format!("{:?}", item)
                                .split_once("(")
                                .unwrap()
                                .0
                                .to_lowercase()
                        ),
                    ),
                },
            }
        }
        self.tidy_idl()
    }

    fn tidy_idl(&mut self) {
        let go_mod_name = &self.gomod_name;
        match self.idl_type {
            IdlType::Proto | IdlType::ProtoNoCodec => {
                self.idl_file = self.target_out_dir.join(go_mod_name.clone() + ".proto");
                fs::write(
                    &self.idl_file,
                    fs::read_to_string(&self.config.idl_file).unwrap()
                        + &format!(
                            "\noption go_package=\"./;{go_mod_name}\";\npackage {go_mod_name};\n"
                        ),
                )
                .unwrap();
            }
            IdlType::Thrift | IdlType::ThriftNoCodec => {
                self.idl_file = self.target_out_dir.join(go_mod_name.clone() + ".thrift");
                fs::copy(&self.config.idl_file, &self.idl_file).unwrap();
            }
        };
        self.idl_include_dir = self.idl_file.parent().unwrap().to_path_buf();
    }

    // rustc-link-lib=[KIND=]NAME indicates that the specified value is a library name and should be passed to the compiler as a -l flag. The optional KIND can be one of static, dylib (the default), or framework, see rustc --help for more details.
    //
    // rustc-link-search=[KIND=]PATH indicates the specified value is a library search path and should be passed to the compiler as a -L flag. The optional KIND can be one of dependency, crate, native, framework or all (the default), see rustc --help for more details.
    //
    // rustc-flags=FLAGS is a set of flags passed to the compiler, only -l and -L flags are supported.
    pub(crate) fn rustc_link(&self) {
        println!(
            "cargo:rustc-link-search=native={}",
            self.clib_gen_dir.to_str().unwrap()
        );
        println!(
            "cargo:rustc-link-search=dependency={}",
            self.clib_gen_dir.to_str().unwrap()
        );
        println!(
            "cargo:rustc-link-lib={}={}",
            self.rustc_link_kind_goffi, self.go_clib_name_base
        );
    }

    pub(crate) fn rerun_if_changed(&self) {
        println!("cargo:rerun-if-changed={}", self.pkg_dir.to_str().unwrap());
        println!(
            "cargo:rerun-if-changed={}",
            self.target_out_dir.to_str().unwrap()
        );
    }

    fn check_go_mod_path(&self) {
        let f = &self.gomod_file;
        if f.exists() {
            if !f.is_file() {
                exit_with_warning(
                    253,
                    format!("go mod file {} does not exist", f.to_str().unwrap()),
                );
            } else {
                let p = &self.gomod_path;
                let s = fs::read_to_string(f).unwrap();
                if !s.contains(&format!("module {p}\n"))
                    && !s.contains(&format!("module {p}\t"))
                    && !s.contains(&format!("module {p}\r"))
                    && !s.contains(&format!("module {p} "))
                {
                    exit_with_warning(
                        253,
                        format!("go mod path should be {p}, file={}", f.to_str().unwrap()),
                    );
                }
            }
        }
    }

    fn init_files(&self) -> anyhow::Result<()> {
        fs::create_dir_all(&self.go_main_dir)?;
        fs::create_dir_all(&self.rust_mod_dir)?;
        fs::create_dir_all(&self.clib_gen_dir)?;
        for f in [
            &self.rust_clib_file,
            &self.rust_clib_header,
            &self.go_clib_file,
            &self.go_clib_header,
            &self.fingerprint_path,
        ] {
            OpenOptions::new()
                .write(true)
                .create(true)
                .open(&self.clib_gen_dir.join(f))?;
        }
        Ok(())
    }

    pub(crate) fn go_cmd_path(&self, cmd: &'static str) -> String {
        if let Some(go_root_path) = &self.config.go_root_path {
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
}
