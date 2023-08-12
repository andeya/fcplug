use std::cell::RefCell;
use std::sync::Arc;

use crate::{deal_output, exit_with_warning, new_shell_cmd};
use crate::config::{Config, WorkConfig};
use crate::os_arch::get_go_os_arch_from_env;

#[derive(Debug, Clone)]
pub(crate) struct Generator {
    pub(crate) config: WorkConfig,
    pub(crate) go_pkg_code: Arc<RefCell<String>>,
    pub(crate) go_main_code: Arc<RefCell<String>>,
    pub(crate) rust_impl_rustffi_code: Arc<RefCell<String>>,
    pub(crate) rust_impl_goffi_code: Arc<RefCell<String>>,
}

unsafe impl Send for Generator {}

impl Generator {
    pub(crate) fn generate(config: Config) {
        Self {
            config: WorkConfig::new(config),
            go_pkg_code: Arc::new(RefCell::new(String::new())),
            go_main_code: Arc::new(RefCell::new(String::new())),
            rust_impl_rustffi_code: Arc::new(RefCell::new("".to_string())),
            rust_impl_goffi_code: Arc::new(RefCell::new("".to_string())),
        }
            .gen_code();
    }
    fn gen_code(self) {
        self.config.create_crate_dir_all();
        self.config.rerun_if_changed();
        self._gen_code();
    }
    pub(crate) fn gen_rust_clib(&self, with_after_include: &str) {
        let _ = cbindgen::Builder::new()
            .with_src(self.config.rust_mod_gen_file())
            .with_language(cbindgen::Language::C)
            .with_after_include(with_after_include)
            .generate()
            .inspect(|b| {
                let _ = b.write_to_file(self.config.rust_clib_h_path());
            })
            .inspect_err(|e| {
                exit_with_warning(254, format!("failed to generate rust clib: {e:?}"))
            });
    }
    pub(crate) fn gen_go_clib(&self) {
        if !self.config.has_goffi {
            return;
        }
        let clib_name = self.config.go_clib_a_path();
        let clib_name_str = clib_name.file_name().unwrap().to_str().unwrap();
        if !self.config.rust_clib_a_path().exists() {
            println!("cargo:warning='{}' file does not exist, should re-execute 'cargo build'", clib_name_str);
        } else {
            let mut cmd = new_shell_cmd();
            match get_go_os_arch_from_env() {
                Ok((os, arch)) => {
                    cmd
                        .env("GOOS", os.as_ref())
                        .env("GOARCH", arch.as_ref());
                }
                Err(e) => { println!("cargo:warning={e}") }
            }
            deal_output(
                cmd
                    .env("CGO_ENABLED", "1")
                    .arg(format!(
                        "{} build -buildmode=c-archive -o {} {}",
                        self.config.go_cmd_path("go"),
                        clib_name.to_str().unwrap(),
                        self.config.go_main_dir().to_str().unwrap(),
                    ))
                    .output()
            );
            if !clib_name.exists() {
                println!("cargo:warning=failed to execute 'go build -buildmode=c-archive', should re-execute 'cargo build' to ensure the correctness of '{}'", clib_name_str);
            }
            self.config.rustc_link();
        }
        if self.config.update_crate_modified() {
            println!("cargo:warning=The crate files has changed, it is recommended to re-execute 'cargo build' to ensure the correctness of '{}'", clib_name_str);
        }
    }
}
