use std::sync::Arc;

use pilota_build::{DefId, IdentName};
use pilota_build::rir::{Method, Service};

use crate::generator::{RustCodegenBackend, RustGeneratorBackend, ServiceType};

impl RustCodegenBackend for RustGeneratorBackend {
    fn codegen_rustffi_trait_method(&self, service_def_id: DefId, method: &Arc<Method>) -> Option<String> {
        let method_name = (&**method.name).fn_ident();
        let args = self.codegen_method_args(service_def_id, method);
        let ret = self.codegen_method_ret(service_def_id, method);
        Some(format!("fn {method_name}({args}) -> {ret}"))
    }
    fn codegen_rustffi_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        let name = self.context.rust_name(def_id);
        let name_lower = name.to_lowercase();
        let ust = self.config.rust_mod_impl_name();
        stream.push_str(
            &s.methods
                .iter()
                .map(|method| {
                    let fn_name = (&**method.name).fn_ident();
                    let args = self.codegen_ffi_args_param(def_id, method);
                    let args_ident = self.codegen_ffi_args_ident(def_id, method);
                    let ret = self.codegen_ffi_ret(def_id, method);
                    format!(
                        r###"#[no_mangle]
                #[inline]
                pub extern "C" fn {name_lower}_{fn_name}({args}) -> {ret} {{
                    {ret}::from(<{ust} as {name}>::{fn_name}({args_ident}))
                }}
                "###
                    )
                })
                .collect::<Vec<String>>()
                .join("\n"),
        );
    }
    fn codegen_goffi_trait_method(&self, service_def_id: DefId, method: &Arc<Method>) -> Option<String> {
        if self.context.is_empty_ty(&method.ret.kind) && !method.ret.is_scalar() {
            return None;
        }
        let method_name = (&**method.name).fn_ident();
        let ffi_ret = self.codegen_ffi_ret(service_def_id, method);
        let ret_ty_name = self.rust_codegen_item_ty(&method.ret.kind);
        Some(format!("unsafe fn {method_name}_set_result(go_ret: ::fcplug::RustFfiArg<{ret_ty_name}>) -> {ffi_ret}"))
    }
    fn codegen_goffi_call_trait_method(&self, service_def_id: DefId, method: &Arc<Method>) -> Option<String> {
        let name = self.context.rust_name(service_def_id);
        let name_lower = name.to_lowercase();
        let method_name = (&**method.name).fn_ident();
        let args = self.codegen_method_args(service_def_id, method);
        let ret = self.codegen_method_ret(service_def_id, method);
        let generic_signature = if self.context.is_empty_ty(&method.ret.kind) {
            String::new()
        } else {
            "<T: Default>".to_string()
        };
        let args_ident = self.codegen_ffi_args_ident(service_def_id, method);
        Some(format!(
            r###"unsafe fn {method_name}{generic_signature}({args}) -> {ret} {{
                ::fcplug::ABIResult::from({name_lower}_{method_name}({args_ident}))
            }}
            "###
        ))
    }
    fn codegen_goffi_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        let name = self.context.rust_name(def_id);
        let name_lower = name.to_lowercase();
        let ust = self.config.rust_mod_impl_name();
        let ffi_fns = s
            .methods
            .iter()
            .map(|method| {
                let fn_name = (&**method.name).fn_ident();
                let args = self.codegen_ffi_args_param(def_id, method);
                let ret = self.codegen_ffi_ret(def_id, method);
                format!("fn {name_lower}_{fn_name}({args}) -> {ret};")
            })
            .collect::<Vec<String>>()
            .join("\n");
        stream.push_str(&format!(
            r###"
            #[link(name = "{}", kind = "{}")]
            extern "C" {{
            {ffi_fns}
        }}
        "###,
            self.config.go_c_header_name_base,
            self.config.rustc_link_kind_goffi(),
        ));

        let store_to_rust_fns = s
            .methods
            .iter()
            .filter(|method| !method.ret.is_scalar())
            .map(|method| {
                let fn_name = (&**method.name).fn_ident().to_string() + "_set_result";
                let ret = self.codegen_ffi_ret(def_id, method);
                format!(
                    r###"#[no_mangle]
                #[inline]
                pub extern "C" fn {name_lower}_{fn_name}(buf: ::fcplug::Buffer) -> {ret} {{
                    unsafe{{<{ust} as {name}>::{fn_name}(::fcplug::RustFfiArg::from(buf))}}
                }}
                "###
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        stream.push_str(&store_to_rust_fns);
    }
}

impl RustGeneratorBackend {
    fn codegen_ffi_args_param(&self, service_def_id: DefId, method: &Method) -> String {
        match self.context.service_type(service_def_id) {
            ServiceType::RustFfi => method
                .args
                .iter()
                .map(|arg| {
                    let ident = (&**arg.name).snake_ident();
                    let ty_name = self.rust_codegen_item_ty(&arg.ty.kind);
                    if arg.ty.is_scalar() {
                        format!("{ident}: {ty_name}")
                    } else {
                        format!("{ident}: ::fcplug::Buffer")
                    }
                })
                .collect::<Vec<String>>()
                .join(", "),
            ServiceType::GoFfi => method
                .args
                .iter()
                .map(|arg| {
                    let ident = (&**arg.name).snake_ident();
                    let ty_name = self.rust_codegen_item_ty(&arg.ty.kind);
                    if arg.ty.is_scalar() {
                        format!("{ident}: {ty_name}")
                    } else {
                        format!("{ident}: ::fcplug::Buffer")
                    }
                })
                .collect::<Vec<String>>()
                .join(", "),
        }
    }
    fn codegen_ffi_args_ident(&self, service_def_id: DefId, method: &Method) -> String {
        match self.context.service_type(service_def_id) {
            ServiceType::RustFfi => method
                .args
                .iter()
                .map(|arg| {
                    let ident = (&**arg.name).snake_ident();
                    if arg.ty.is_scalar() {
                        format!("{ident}")
                    } else {
                        format!("::fcplug::RustFfiArg::from({ident})")
                    }
                })
                .collect::<Vec<String>>()
                .join(", "),
            ServiceType::GoFfi => method
                .args
                .iter()
                .map(|arg| {
                    let ident = (&**arg.name).snake_ident();
                    if arg.ty.is_scalar() {
                        format!("{ident}")
                    } else {
                        format!("::fcplug::Buffer::from_vec({ident}.bytes)")
                    }
                })
                .collect::<Vec<String>>()
                .join(", "),
        }
    }
    fn codegen_method_args(&self, service_def_id: DefId, method: &Method) -> String {
        match self.context.service_type(service_def_id) {
            ServiceType::RustFfi => method
                .args
                .iter()
                .map(|arg| {
                    let ident = (&**arg.name).snake_ident();
                    let ty_name = self.rust_codegen_item_ty(&arg.ty.kind);
                    if arg.ty.is_scalar() {
                        format!("{ident}: {ty_name}")
                    } else {
                        format!("{ident}: ::fcplug::RustFfiArg<{ty_name}>")
                    }
                })
                .collect::<Vec<String>>()
                .join(", "),
            ServiceType::GoFfi => method
                .args
                .iter()
                .map(|arg| {
                    let ident = (&**arg.name).snake_ident();
                    let ty_name = self.rust_codegen_item_ty(&arg.ty.kind);
                    if arg.ty.is_scalar() {
                        format!("{ident}: {ty_name}")
                    } else {
                        format!("{ident}: ::fcplug::TBytes<{ty_name}>")
                    }
                })
                .collect::<Vec<String>>()
                .join(", "),
        }
    }
    fn codegen_method_ret(&self, service_def_id: DefId, method: &Method) -> String {
        let ty_name = self.rust_codegen_item_ty(&method.ret.kind);
        match self.context.service_type(service_def_id) {
            ServiceType::RustFfi => {
                if self.context.is_empty_ty(&method.ret.kind) {
                    format!("::fcplug::ABIResult<()>")
                } else if method.ret.is_scalar() {
                    format!("{ty_name}")
                } else {
                    format!("::fcplug::ABIResult<::fcplug::TBytes<{ty_name}>>")
                }
            }
            ServiceType::GoFfi => {
                if self.context.is_empty_ty(&method.ret.kind) {
                    format!("::fcplug::ABIResult<()>")
                } else if method.ret.is_scalar() {
                    format!("{ty_name}")
                } else {
                    format!("::fcplug::ABIResult<T>")
                }
            }
        }
    }
    fn codegen_ffi_ret(&self, service_def_id: DefId, method: &Method) -> String {
        let ty_name = self.rust_codegen_item_ty(&method.ret.kind);
        match self.context.service_type(service_def_id) {
            ServiceType::RustFfi => {
                if self.context.is_empty_ty(&method.ret.kind) {
                    format!("::fcplug::RustFfiResult")
                } else if method.ret.is_scalar() {
                    format!("{ty_name}")
                } else {
                    format!("::fcplug::RustFfiResult")
                }
            }
            ServiceType::GoFfi => {
                if self.context.is_empty_ty(&method.ret.kind) {
                    format!("::fcplug::GoFfiResult")
                } else if method.ret.is_scalar() {
                    format!("{ty_name}")
                } else {
                    format!("::fcplug::GoFfiResult")
                }
            }
        }
    }
}
