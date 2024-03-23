use std::cell::RefCell;
use std::fs;
use std::ops::Deref;
use std::process::Command;
use std::sync::Arc;

use pilota_build::db::RirDatabase;
use pilota_build::fmt::fmt_file;
use pilota_build::plugin::{AutoDerivePlugin, PredicateResult};
use pilota_build::rir::{Arg, Enum, Field, Item, Message, NewType, Service};
use pilota_build::ty::{CodegenTy, TyKind};
use pilota_build::{
    rir::Method, CodegenBackend, Context, DefId, IdentName, MakeBackend, Output, ProtobufBackend,
    ThriftBackend,
};

use crate::config::IdlType;
use crate::config::{Config, WorkConfig};
use crate::os_arch::get_go_os_arch_from_env;
use crate::{deal_output, deal_result, exit_with_warning, CODE_IO};

#[derive(Debug, Clone)]
pub(crate) struct Generator {
    pub(crate) config: WorkConfig,
    pub(crate) go_lib_code: Arc<RefCell<String>>,
    pub(crate) go_main_code: Arc<RefCell<String>>,
    pub(crate) rust_mod_impl_code: Arc<RefCell<String>>,
}

unsafe impl Send for Generator {}

#[derive(Default)]
pub(crate) struct ImportPkg {
    pub(crate) in_main: bool,
    pub(crate) in_lib: bool,
    pub(crate) import_path: String,
    pub(crate) use_code: String,
}

pub(crate) struct MidOutput {
    pub(crate) rust_clib_includes: String,
    pub(crate) mod_requires: Vec<String>,
    pub(crate) imports: Vec<ImportPkg>,
}

impl Generator {
    pub(crate) fn generate(config: Config) {
        Self {
            config: WorkConfig::new(config),
            go_lib_code: Arc::new(RefCell::new(String::new())),
            go_main_code: Arc::new(RefCell::new(String::new())),
            rust_mod_impl_code: Arc::new(RefCell::new("".to_string())),
        }
        .gen_code();
    }
    fn gen_code(self) {
        self.config.rerun_if_changed();

        let clib_dir_relative_root =
            pathdiff::diff_paths(&self.config.clib_gen_dir, &self.config.pkg_dir)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

        let clib_dir_relative_cgobin = 
            pathdiff::diff_paths(&self.config.clib_gen_dir, &self.config.go_main_dir)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

        let go_mod_name = self.config.gomod_name.clone();
        let rust_clib_name_base = self.config.rust_clib_name_base.clone();

        self.build_code_by_idl();
        self.rust_gen_more_code();
        let mid_output = self.clone()._gen_code();

        // build rust c lib
        self.gen_rust_clib(mid_output.rust_clib_includes.as_str());

        // write go lib code
        let mut lib_imports = String::new();
        let mut lib_imports_use = String::new();
        mid_output
            .imports
            .iter()
            .filter(|v| v.in_lib)
            .for_each(|v| {
                lib_imports = format!("{lib_imports}\"{}\"\n   ", v.import_path);
                lib_imports_use = format!("{lib_imports_use}{}\n", v.use_code);
            });
        let go_pkg_code = self.go_lib_code.borrow().clone();
        *self.go_lib_code.borrow_mut() = format!(
            r###"// Code generated by fcplug. DO NOT EDIT.

            package {go_mod_name}
            /*
            #cgo CFLAGS: -I{clib_dir_relative_root}
            #cgo LDFLAGS: -L{clib_dir_relative_root} -l{rust_clib_name_base} -ldl -lm

            #include "{rust_clib_name_base}.h"
            */
            import "C"

            import (
                "errors"
                "fmt"
                "reflect"
                "unsafe"

                {lib_imports}
            )

            var (
                _ = errors.New
                _ = fmt.Sprintf
                _ reflect.SliceHeader
                _ unsafe.Pointer
            )
            {lib_imports_use}

            {go_pkg_code}
            "###
        );
        deal_result(
            CODE_IO,
            std::fs::write(&self.config.go_lib_file, self.go_lib_code.borrow().as_str()),
        );

        // write go main code
        if self.config.has_goffi {
            let mut main_imports = String::new();
            let mut main_imports_use = String::new();
            mid_output
                .imports
                .iter()
                .filter(|v| v.in_main)
                .for_each(|v| {
                    main_imports = format!("{main_imports}\"{}\"\n   ", v.import_path);
                    main_imports_use = format!("{main_imports_use}{}\n", v.use_code);
                });
            let go_main_code = self.go_main_code.borrow().clone();
            *self.go_main_code.borrow_mut() = format!(
                r###"// Code generated by fcplug. DO NOT EDIT.

        package main

        /*
        #cgo CFLAGS: -I{clib_dir_relative_cgobin}
        #cgo LDFLAGS: -L{clib_dir_relative_cgobin} -l{rust_clib_name_base} -ldl -lm

        #include "{rust_clib_name_base}.h"
        */
        import "C"
        import (
            "reflect"
            "unsafe"

            {main_imports}
        )

        // main function is never called by C to.
        func main() {{}}

        var (
            _ reflect.SliceHeader
            _ unsafe.Pointer
        )
        {main_imports_use}

        {go_main_code}
        "###
            );
            deal_result(
                CODE_IO,
                std::fs::write(
                    &self.config.go_main_file,
                    self.go_main_code.borrow().as_str(),
                ),
            );
            if !self.config.go_main_impl_file.exists() {
                deal_result(
                    CODE_IO,
                    std::fs::write(
                        &self.config.go_main_impl_file,
                        format!(
                            r###"package main

            func init() {{
                // TODO: Replace with your own implementation, then re-execute `cargo build`
                GlobalGoFfi = _UnimplementedGoFfi{{}}
            }}

            "###
                        ),
                    ),
                );
            }
        }

        // write go mod
        let _ = if !self.config.gomod_file.exists() {
            let mod_requires = mid_output
                .mod_requires
                .iter()
                .map(|v| v.replace("@", " "))
                .collect::<Vec<String>>()
                .join("\n    ");
            deal_result(
                CODE_IO,
                std::fs::write(
                    &self.config.gomod_file,
                    format!(
                        r###"module {}

            go 1.18

            require (
                {mod_requires}
            )

            "###,
                        &self.config.gomod_path
                    ),
                ),
            );
        } else {
            let mod_content = fs::read_to_string(&self.config.gomod_file).unwrap();
            for mod_require in &mid_output.mod_requires {
                if mod_content.contains(
                    &(mod_require
                        .splitn(2, "@")
                        .next()
                        .unwrap_or_default()
                        .to_string()
                        + " "),
                ) {
                    continue;
                }
                deal_output(
                    Command::new(self.config.go_cmd_path("go"))
                        .env("GO111MODULE", "on")
                        .arg("get")
                        .arg(mod_require)
                        .output(),
                );
            }
        };

        // format go code
        let pkg_dir_str = self.config.pkg_dir.to_str().unwrap().to_string();
        deal_output(
            Command::new(self.config.go_cmd_path("gofmt"))
                .arg("-l")
                .arg("-w")
                .arg(&pkg_dir_str)
                .output(),
        );
        deal_output(
            Command::new(self.config.go_cmd_path("go"))
                .current_dir(&self.config.pkg_dir)
                .arg("mod")
                .arg("tidy")
                .output(),
        );

        // build go c lib
        self.gen_go_clib();
    }

    fn gen_rust_clib(&self, with_after_include: &str) {
        let _ = cbindgen::Builder::new()
            .with_src(&self.config.rust_mod_gen_file)
            .with_language(cbindgen::Language::C)
            .with_after_include(with_after_include)
            .generate()
            .inspect(|b| {
                let _ = b.write_to_file(&self.config.rust_clib_header);
            })
            .inspect_err(|e| {
                exit_with_warning(254, format!("failed to generate rust clib: {e:?}"))
            });
    }
    pub(crate) fn gen_go_clib(&self) {
        if !self.config.has_goffi {
            return;
        }
        let go_clib_file = &self.config.go_clib_file;
        let go_clib_filename = go_clib_file.file_name().unwrap().to_str().unwrap();
        let rust_clib_file = &self.config.rust_clib_file;
        if !rust_clib_file.exists() {
            println!(
                "cargo:warning='{}' file does not exist, should re-execute 'cargo build'",
                rust_clib_file.file_name().unwrap().to_str().unwrap()
            );
        } else {
            let mut cmd = Command::new(self.config.go_cmd_path("go"));
            match get_go_os_arch_from_env() {
                Ok((os, arch)) => {
                    cmd.env("GOOS", os.as_ref()).env("GOARCH", arch.as_ref());
                }
                Err(e) => {
                    println!("cargo:warning={e}")
                }
            }
            deal_output(
                cmd.env("CGO_ENABLED", "1")
                    .arg("build")
                    .arg(format!("-buildmode={}", self.config.go_buildmode))
                    .arg(format!("-o={}", go_clib_file.to_str().unwrap()))
                    .arg(self.config.go_main_dir.to_str().unwrap())
                    .output(),
            );
            // lib{go_clib_name_base}.h -> {go_clib_name_base}.h
            if let Err(err) = fs::rename(
                go_clib_file
                    .parent()
                    .unwrap()
                    .join(go_clib_filename.rsplit_once(".").unwrap().0.to_owned() + ".h"),
                &self.config.go_clib_header,
            ) {
                println!("cargo:warning=failed to fix go C lib file name '{}'", err);
            };
            if !go_clib_file.exists() {
                println!(
                    "cargo:warning=failed to execute 'go build -buildmode={}', should re-execute 'cargo build' to ensure the correctness of '{}'",
                    self.config.go_buildmode, go_clib_filename,
                );
            }
            self.config.rustc_link();
        }
        if self.config.update_crate_modified() {
            println!("cargo:warning=The crate files has changed, it is recommended to re-execute 'cargo build' to ensure the correctness of '{}'", go_clib_filename);
        }
    }
    fn build_code_by_idl(&self) {
        let include_dirs = vec![self.config.idl_include_dir.clone()];
        match self.config.idl_type {
            IdlType::Proto | IdlType::ProtoNoCodec => {
                pilota_build::Builder::protobuf_with_backend(self.clone())
                    .doc_header("// Code generated by fcplug. DO NOT EDIT.".to_string())
                    .include_dirs(include_dirs)
                    .plugin(AutoDerivePlugin::new(
                        Arc::new(["#[derive(::serde::Serialize, ::serde::Deserialize)]".into()]),
                        |_| PredicateResult::GoOn,
                    ))
                    .ignore_unused(true)
                    .compile(
                        [&self.config.idl_file],
                        Output::File(self.config.rust_mod_gen_file.clone()),
                    )
            }
            IdlType::Thrift | IdlType::ThriftNoCodec => {
                pilota_build::Builder::thrift_with_backend(self.clone())
                    .doc_header("// Code generated by fcplug. DO NOT EDIT.".to_string())
                    .include_dirs(include_dirs)
                    .plugin(AutoDerivePlugin::new(
                        Arc::new(["#[derive(::serde::Serialize, ::serde::Deserialize)]".into()]),
                        |_| PredicateResult::GoOn,
                    ))
                    .ignore_unused(true)
                    .compile(
                        [&self.config.idl_file],
                        Output::File(self.config.rust_mod_gen_file.clone()),
                    )
            }
        }
    }

    fn rust_gen_more_code(&self) {
        let mut rust_code = std::fs::read_to_string(&self.config.rust_mod_gen_file).unwrap();
        if !self.config.has_rustffi {
            rust_code.push_str(&format!("pub(super) trait RustFfi {{}}"));
        }
        if !self.config.has_goffi {
            rust_code.push_str(&format!("pub(super) trait GoFfi {{}}"));
            rust_code.push_str(&format!("pub trait GoFfiCall {{}}"));
        }
        let rust_impl_name = &self.config.rust_mod_impl_name;
        rust_code.push_str(&format!(
            r###"trait Ffi: RustFfi + GoFfi + GoFfiCall {{}}

        pub struct {rust_impl_name};

        impl GoFfiCall for {rust_impl_name} {{}}
        impl Ffi for {rust_impl_name} {{}}
        "###
        ));
        std::fs::write(&self.config.rust_mod_gen_file, rust_code).unwrap();
        fmt_file(&self.config.rust_mod_gen_file);

        if !self.config.rust_mod_impl_file.exists() {
            let rust_mod_impl_code = self.rust_mod_impl_code.borrow();
            let mod_gen_name = &self.config.rust_mod_gen_name;
            deal_result(
                CODE_IO,
                std::fs::write(
                    &self.config.rust_mod_impl_file,
                    &format!(
                        r###"#![allow(unused_variables)]

                pub use {mod_gen_name}::*;

                mod {mod_gen_name};

                {rust_mod_impl_code}
                "###
                    ),
                ),
            );
            fmt_file(&self.config.rust_mod_impl_file);
        }
    }
}

impl MakeBackend for Generator {
    type Target = GeneratorBackend;

    fn make_backend(self, context: Context) -> Self::Target {
        let thrift = ThriftBackend::new(context.clone());
        let protobuf = ProtobufBackend::new(context.clone());
        let context = Arc::new(context);
        GeneratorBackend {
            thrift,
            protobuf,
            rust: RustGeneratorBackend {
                config: self.config.clone(),
                context: Cx(context.clone()),
            },
            rust_mod_impl_code: self.rust_mod_impl_code.clone(),
            go: GoGeneratorBackend {
                config: self.config.clone(),
                context: Cx(context.clone()),
                go_lib_code: self.go_lib_code.clone(),
                go_main_code: self.go_main_code.clone(),
            },
            context: Cx(context),
            config: self.config,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct GeneratorBackend {
    pub(crate) config: WorkConfig,
    pub(crate) context: Cx,
    pub(crate) protobuf: ProtobufBackend,
    pub(crate) thrift: ThriftBackend,
    pub(crate) rust: RustGeneratorBackend,
    pub(crate) rust_mod_impl_code: Arc<RefCell<String>>,
    pub(crate) go: GoGeneratorBackend,
}

pub(crate) trait GoCodegenBackend {
    fn codegen_struct_type(&self, _def_id: DefId, _s: &Message) -> String {
        Default::default()
    }
    fn codegen_rustffi_iface_method(
        &self,
        service_def_id: DefId,
        method: &Arc<Method>,
    ) -> Option<(String, String)>;
    fn codegen_rustffi_service_impl(&self, service_def_id: DefId, s: &Service) -> String;
    fn codegen_goffi_iface_method(
        &self,
        service_def_id: DefId,
        method: &Arc<Method>,
    ) -> Option<String>;
    fn codegen_goffi_service_impl(&self, service_def_id: DefId, s: &Service) -> String;
}

pub(crate) trait RustCodegenBackend {
    fn codegen_rustffi_trait_method(
        &self,
        service_def_id: DefId,
        method: &Arc<Method>,
    ) -> Option<String>;
    fn codegen_rustffi_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service);
    fn codegen_goffi_trait_method(
        &self,
        service_def_id: DefId,
        method: &Arc<Method>,
    ) -> Option<String>;
    fn codegen_goffi_call_trait_method(
        &self,
        service_def_id: DefId,
        method: &Arc<Method>,
    ) -> Option<String>;
    fn codegen_goffi_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service);
}

#[derive(Clone)]
pub(crate) struct GoGeneratorBackend {
    pub(crate) config: WorkConfig,
    pub(crate) context: Cx,
    pub(crate) go_lib_code: Arc<RefCell<String>>,
    pub(crate) go_main_code: Arc<RefCell<String>>,
}

#[derive(Clone)]
pub(crate) struct RustGeneratorBackend {
    pub(crate) config: WorkConfig,
    pub(crate) context: Cx,
}

unsafe impl Send for GeneratorBackend {}

#[derive(Clone)]
pub(crate) struct Cx(pub(crate) Arc<Context>);

pub(crate) enum ServiceType {
    RustFfi,
    GoFfi,
}

impl Cx {
    pub(crate) fn service_type(&self, service_def_id: DefId) -> ServiceType {
        match self.rust_name(service_def_id).to_lowercase().as_str() {
            "rustffi" => ServiceType::RustFfi,
            "goffi" => ServiceType::GoFfi,
            _ => {
                unreachable!()
            }
        }
    }
    pub(crate) fn is_empty_ty(&self, kind: &TyKind) -> bool {
        match kind {
            TyKind::Path(path) => {
                if let Item::Message(m) = self.item(path.did).unwrap().as_ref() {
                    m.fields.is_empty()
                } else {
                    false
                }
            }
            TyKind::Void => true,
            _ => false,
        }
    }
}

impl RustGeneratorBackend {
    pub(crate) fn rust_codegen_item_ty(&self, ty: &TyKind) -> String {
        match &ty {
            TyKind::String => CodegenTy::String.to_string(),
            TyKind::Void => CodegenTy::Void.to_string(),
            TyKind::U8 => CodegenTy::U8.to_string(),
            TyKind::Bool => CodegenTy::Bool.to_string(),
            TyKind::Bytes => CodegenTy::Bytes.to_string(),
            TyKind::I8 => CodegenTy::I8.to_string(),
            TyKind::I16 => CodegenTy::I16.to_string(),
            TyKind::I32 => CodegenTy::I32.to_string(),
            TyKind::I64 => CodegenTy::I64.to_string(),
            TyKind::F64 => CodegenTy::F64.to_string(),
            TyKind::Vec(ty) => format!("::std::vec::Vec<{}>", self.rust_codegen_item_ty(&ty.kind)),
            TyKind::Set(ty) => format!(
                "::std::collections::HashSet<{}>",
                self.rust_codegen_item_ty(&ty.kind)
            ),
            TyKind::Map(key, value) => format!(
                "::std::collections::HashMap<{}, {}>",
                self.rust_codegen_item_ty(&key.kind),
                self.rust_codegen_item_ty(&value.kind)
            ),
            TyKind::Path(path) => self.context.rust_name(path.did).0.to_string(),
            TyKind::UInt32 => CodegenTy::UInt32.to_string(),
            TyKind::UInt64 => CodegenTy::UInt64.to_string(),
            TyKind::F32 => CodegenTy::F32.to_string(),
            TyKind::Arc(ty) => format!("::std::sync::Arc<{}>", self.rust_codegen_item_ty(&ty.kind)),
        }
    }
}

impl GoGeneratorBackend {
    pub(crate) fn go_codegen_item_ty(&self, ty: &TyKind, is_main: bool) -> String {
        match &ty {
            TyKind::String => "string".to_string(),
            TyKind::Void => "struct{}".to_string(),
            TyKind::U8 => "uint8".to_string(),
            TyKind::Bool => "bool".to_string(),
            TyKind::Bytes => "[]byte".to_string(),
            TyKind::I8 => "int8".to_string(),
            TyKind::I16 => "int16".to_string(),
            TyKind::I32 => "int32".to_string(),
            TyKind::I64 => "int64".to_string(),
            TyKind::F64 => "float64".to_string(),
            TyKind::Vec(ty) => format!("[]{}", self.go_codegen_item_ty(&ty.kind, is_main)),
            TyKind::Set(ty) => format!("[]{}", self.go_codegen_item_ty(&ty.kind, is_main)),
            TyKind::Map(key, value) => format!(
                "map[{}]{}",
                self.go_codegen_item_ty(&key.kind, is_main),
                self.go_codegen_item_ty(&value.kind, is_main)
            ),
            TyKind::Path(path) => {
                let mut pkg_pre = String::new();
                if is_main {
                    pkg_pre = self.config.gomod_name.clone() + ".";
                }
                format!("{pkg_pre}{}", self.struct_name(path.did))
            }
            TyKind::UInt32 => "uint32".to_string(),
            TyKind::UInt64 => "uint64".to_string(),
            TyKind::F32 => "float32".to_string(),
            TyKind::Arc(ty) => format!("*{}", self.go_codegen_item_ty(&ty.kind, is_main)),
        }
    }
    pub(crate) fn arg_name(&self, arg: &Arc<Arg>) -> String {
        arg.name.0.to_lowercase()
    }
    pub(crate) fn arg_type(&self, arg: &Arc<Arg>, is_main: bool) -> String {
        self.go_codegen_item_ty(&arg.ty.kind, is_main)
    }
    pub(crate) fn ret_type(&self, method: &Arc<Method>, is_main: bool) -> String {
        self.go_codegen_item_ty(&method.ret.kind, is_main)
    }
    pub(crate) fn field_name(&self, f: &Arc<Field>) -> String {
        self.context
            .rust_name(f.did)
            .0
            .upper_camel_ident()
            .to_string()
    }
    pub(crate) fn field_type(&self, f: &Arc<Field>) -> String {
        self.go_codegen_item_ty(&f.ty.kind, false)
    }
    pub(crate) fn field_tag(&self, f: &Arc<Field>) -> String {
        format!(
            r###"`json:"{}"`"###,
            self.context.rust_name(f.did).0.snake_ident()
        )
    }
    pub(crate) fn struct_name(&self, message_def_id: DefId) -> String {
        self.context
            .rust_name(message_def_id)
            .0
            .struct_ident()
            .to_string()
    }
    pub(crate) fn iface_method_name(&self, method: &Arc<Method>) -> String {
        method.name.0.upper_camel_ident().to_string()
    }
    pub(crate) fn ffi_func_name(&self, service_def_id: DefId, method: &Arc<Method>) -> String {
        let service_name_lower = self.context.rust_name(service_def_id).to_lowercase();
        let method_name_lower = (&**method.name).fn_ident();
        format!("{service_name_lower}_{method_name_lower}")
    }
}

impl Deref for Cx {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl GeneratorBackend {
    pub(crate) fn fix_empty_params(&self, method: &Method) -> Method {
        let mut method = method.clone();
        method.args = method
            .args
            .into_iter()
            .filter(|arg| !self.context.is_empty_ty(&arg.ty.kind))
            .collect::<Vec<Arc<Arg>>>();
        if self.context.is_empty_ty(&method.ret.kind) {
            method.ret.kind = TyKind::Void;
        }
        method
    }
}

impl CodegenBackend for GeneratorBackend {
    fn cx(&self) -> &Context {
        &self.context
    }
    fn codegen_struct_impl(&self, def_id: DefId, stream: &mut String, s: &Message) {
        // rust
        match self.config.idl_type {
            IdlType::Proto => self.protobuf.codegen_struct_impl(def_id, stream, s),
            IdlType::Thrift => self.thrift.codegen_struct_impl(def_id, stream, s),
            _ => {}
        }
        // go
        self.go
            .go_lib_code
            .borrow_mut()
            .push_str(&self.go.codegen_struct_type(def_id, &s));
    }
    fn codegen_service_impl(&self, service_def_id: DefId, stream: &mut String, s: &Service) {
        let mut s = s.clone();
        s.methods = s
            .methods
            .iter()
            .map(|method| Arc::new(self.fix_empty_params(method)))
            .collect::<Vec<Arc<Method>>>();
        // rust
        match self.config.idl_type {
            IdlType::Proto => self
                .protobuf
                .codegen_service_impl(service_def_id, stream, &s),
            IdlType::Thrift => self.thrift.codegen_service_impl(service_def_id, stream, &s),
            _ => {}
        }
        let service_type = self.context.service_type(service_def_id);
        let mut methods = Vec::new();
        let mut call_methods = String::new();
        for method in &s.methods {
            match service_type {
                ServiceType::RustFfi => {
                    if let Some(code) = self
                        .rust
                        .codegen_rustffi_trait_method(service_def_id, method)
                    {
                        methods.push(code);
                    }
                }
                ServiceType::GoFfi => {
                    if let Some(code) = self.rust.codegen_goffi_trait_method(service_def_id, method)
                    {
                        methods.push(code);
                    }
                    if let Some(code) = self
                        .rust
                        .codegen_goffi_call_trait_method(service_def_id, method)
                    {
                        call_methods.push_str(&(code + "\n"));
                    }
                }
            };
        }
        let mut methods = methods
            .into_iter()
            .filter(|m| !m.is_empty())
            .collect::<Vec<String>>();
        let name = self.context.rust_name(service_def_id);
        methods.push("".to_string());
        let trait_methods = methods.join(";\n");
        let impl_trait_methods = methods.join(" { todo!() }\n");
        stream.push_str(&format! {r#"
            pub(super) trait {name} {{
                {trait_methods}
            }}
            "#});
        let name = self.context.rust_name(service_def_id);
        let ust = &self.config.rust_mod_impl_name;
        self.rust_mod_impl_code.borrow_mut().push_str(&format!(
            r###"
            impl {name} for {ust} {{
                {impl_trait_methods}
            }}
            "###
        ));

        match service_type {
            ServiceType::RustFfi => {
                self.rust
                    .codegen_rustffi_service_impl(service_def_id, stream, &s);
            }
            ServiceType::GoFfi => {
                stream.push_str(&format! {r#"
                pub trait {name}Call {{
                    {call_methods}
                }}
                "#});
                self.rust
                    .codegen_goffi_service_impl(service_def_id, stream, &s);
            }
        }
        // go
        match service_type {
            ServiceType::RustFfi => {
                let mut iface_methods = String::new();
                let mut impl_methods = String::new();
                for method in &s.methods {
                    if let Some((iface_method, impl_method)) =
                        self.go.codegen_rustffi_iface_method(service_def_id, method)
                    {
                        iface_methods.push_str(&format!("{iface_method}\n"));
                        impl_methods.push_str(&format!(
                            r###"
                    //go:inline
                    func (RustFfiImpl) {iface_method} {{
                        {impl_method}
                    }}
                    "###
                        ));
                    }
                }
                self.go.go_lib_code.borrow_mut().push_str(&format!(
                    r###"
                        var GlobalRustFfi RustFfi = RustFfiImpl{{}}

                        type RustFfi interface {{
                            {iface_methods}
                        }}
                        type RustFfiImpl struct{{}}
                        {impl_methods}
                        "###
                ));
            }
            ServiceType::GoFfi => {
                let mut iface_body = String::new();
                let mut impl_methods = String::new();
                for method in &s.methods {
                    if let Some(m) = self.go.codegen_goffi_iface_method(service_def_id, method) {
                        iface_body.push_str(&format!("{m}\n"));
                        impl_methods.push_str(&format!(
                            r###"func (_UnimplementedGoFfi) {m} {{
                        panic("unimplemented")
                    }}
                    "###
                        ));
                    }
                }
                self.go.go_main_code.borrow_mut().push_str(&format!(
                    r###"

                        var GlobalGoFfi GoFfi = _UnimplementedGoFfi{{}}

                        type GoFfi interface {{
                            {iface_body}
                        }}
                        type _UnimplementedGoFfi struct{{}}
                        {impl_methods}

                        "###
                ))
            }
        }

        match service_type {
            ServiceType::RustFfi => self
                .go
                .go_lib_code
                .borrow_mut()
                .push_str(&self.go.codegen_rustffi_service_impl(service_def_id, &s)),
            ServiceType::GoFfi => self
                .go
                .go_main_code
                .borrow_mut()
                .push_str(&self.go.codegen_goffi_service_impl(service_def_id, &s)),
        }
    }
    fn codegen_enum_impl(&self, def_id: DefId, stream: &mut String, e: &Enum) {
        // rust codec
        match self.config.idl_type {
            IdlType::Proto => self.protobuf.codegen_enum_impl(def_id, stream, e),
            IdlType::Thrift => self.thrift.codegen_enum_impl(def_id, stream, e),
            _ => {}
        }
    }
    fn codegen_newtype_impl(&self, def_id: DefId, stream: &mut String, t: &NewType) {
        // rust codec
        match self.config.idl_type {
            IdlType::Proto => self.protobuf.codegen_newtype_impl(def_id, stream, t),
            IdlType::Thrift => self.thrift.codegen_newtype_impl(def_id, stream, t),
            _ => {}
        }
    }
}
