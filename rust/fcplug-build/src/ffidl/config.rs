use std::{env, fs};
use std::path::PathBuf;

const CGOBIN: &'static str = "cgobin";

#[derive(Debug, Clone)]
pub struct Config {
    pub idl_file: PathBuf,
    /// Target crate directory for code generation
    pub target_crate_dir: Option<PathBuf>,
    pub go_root_path: Option<PathBuf>,
    pub go_mod_parent: &'static str,
}

pub(crate) enum IdlType {
    Proto,
    Thrift,
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

impl Config {
    pub(crate) fn idl_type(&self) -> IdlType {
        match self.idl_file.extension().unwrap().to_str().unwrap() {
            "proto" => IdlType::Proto,
            "thrift" => IdlType::Thrift,
            _ => IdlType::Proto,
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
    pub(crate) fn new_crate_modified(&self) -> String {
        walkdir::WalkDir::new(self.pkg_dir())
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
}
