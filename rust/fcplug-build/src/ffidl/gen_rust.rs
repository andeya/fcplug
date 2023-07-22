use std::sync::Arc;

use pilota_build::{DefId, IdentName};
use pilota_build::rir::{Method, Service};
use pilota_build::ty::{CodegenTy, TyKind};

use crate::ffidl::{Config, Cx, ServiceType};

#[derive(Clone)]
pub(crate) struct RustCodegenBackend {
    pub(crate) config: Arc<Config>,
    pub(crate) context: Cx,
}

impl RustCodegenBackend {
    pub(crate) fn codegen_service_method(&self, service_def_id: DefId, method: &Method) -> String {
        let service_name_lower = self.context.rust_name(service_def_id).to_lowercase();
        let method_name = (&**method.name).fn_ident();
        let args = self.codegen_method_args(service_def_id, method);
        let ret = self.codegen_method_ret(service_def_id, method);
        match self.context.service_type(service_def_id) {
            ServiceType::RustFfi => format!("fn {method_name}({args}) -> {ret};"),
            ServiceType::GoFfi => {
                let is_empty_ret = self.context.is_empty_ty(&method.ret.kind);
                let set_ret_fn = if is_empty_ret || method.ret.is_scalar() {
                    String::new()
                } else {
                    let ffi_ret = self.codegen_ffi_ret(service_def_id, method);
                    let ret_ty_name = self.codegen_item_ty(&method.ret.kind);
                    format!("unsafe fn {method_name}_set_result(go_ret: ::fcplug::RustFfiArg<{ret_ty_name}>) -> {ffi_ret};")
                };

                let generic_signature = if self.context.is_empty_ty(&method.ret.kind) {
                    String::new()
                } else {
                    "<T: Default>".to_string()
                };
                let args_ident = self.codegen_ffi_args_ident(service_def_id, method);
                format!(r###"unsafe fn {method_name}{generic_signature}({args}) -> {ret} {{
                    ::fcplug::ABIResult::from({service_name_lower}_{method_name}({args_ident}))
                }}
                {set_ret_fn}
                "###)
            }
        }
    }
    pub(crate) fn codegen_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        match self.context.service_type(def_id) {
            ServiceType::RustFfi => self.codegen_rustffi_service_impl(def_id, stream, s),
            ServiceType::GoFfi => self.codegen_goffi_service_impl(def_id, stream, s),
        }
    }
    fn codegen_rustffi_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        let name = self.context.rust_name(def_id);
        let name_lower = name.to_lowercase();
        let (ust, setted) = self.rustffi_impl_name(def_id);
        if !setted {
            let methods = s.methods.iter().map(|method| {
                let name = (&**method.name).fn_ident();
                let args = self.codegen_method_args(def_id, method);
                let ret = self.codegen_method_ret(def_id, method);
                format!("fn {name}({args}) -> {ret} {{ unimplemented!() }}")
            }).collect::<Vec<String>>()
                .join("\n");

            stream.push_str(&format!(r###"struct {ust};
            impl {name} for {ust} {{
                {methods}
            }}"###));
        };

        stream.push_str(&s.methods.iter().map(|method| {
            let fn_name = (&**method.name).fn_ident();
            let args = self.codegen_ffi_args_param(def_id, method);
            let args_ident = self.codegen_ffi_args_ident(def_id, method);
            let ret = self.codegen_ffi_ret(def_id, method);
            format!(r###"#[no_mangle]
                #[inline]
                pub extern "C" fn {name_lower}_{fn_name}({args}) -> {ret} {{
                    {ret}::from(<{ust} as {name}>::{fn_name}({args_ident}))
                }}
                "###)
        })
            .collect::<Vec<String>>()
            .join("\n"));
    }
    fn codegen_goffi_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        let name = self.context.rust_name(def_id);
        let name_lower = name.to_lowercase();
        let ffi_fns = s.methods.iter().map(|method| {
            let fn_name = (&**method.name).fn_ident();
            let args = self.codegen_ffi_args_param(def_id, method);
            let ret = self.codegen_ffi_ret(def_id, method);
            format!("fn {name_lower}_{fn_name}({args}) -> {ret};")
        })
            .collect::<Vec<String>>()
            .join("\n");
        stream.push_str(&format!(r###"extern "C" {{
            {ffi_fns}
        }}
        "###));


        let (ust, _) = self.rustffi_impl_name(def_id);
        let store_to_rust_fns = s.methods.iter()
            .filter(|method| !method.ret.is_scalar())
            .map(|method| {
                let fn_name = (&**method.name).fn_ident().to_string() + "_set_result";
                let ret = self.codegen_ffi_ret(def_id, method);
                format!(r###"#[no_mangle]
                #[inline]
                pub extern "C" fn {name_lower}_{fn_name}(buf: ::fcplug::Buffer) -> {ret} {{
                    unsafe{{<{ust} as {name}>::{fn_name}(::fcplug::RustFfiArg::from(buf))}}
                }}
                "###)
            })
            .collect::<Vec<String>>()
            .join("\n");
        stream.push_str(&store_to_rust_fns);
    }
}

impl RustCodegenBackend {
    fn rustffi_impl_name(&self, service_def_id: DefId) -> (String, bool) {
        if let Some(ust) = &self.config.impl_ffi_for_unitstruct {
            (ust.0.to_string(), true)
        } else {
            let name = self.context.rust_name(service_def_id);
            (format!("Unimplemented{name}"), false)
        }
    }
    fn codegen_ffi_args_param(&self, service_def_id: DefId, method: &Method) -> String {
        match self.context.service_type(service_def_id) {
            ServiceType::RustFfi => method.args
                .iter()
                .map(|arg| {
                    let ident = (&**arg.name).snake_ident();
                    let ty_name = self.codegen_item_ty(&arg.ty.kind);
                    if arg.ty.is_scalar() {
                        format!("{ident}: {ty_name}")
                    } else {
                        format!("{ident}: ::fcplug::Buffer")
                    }
                })
                .collect::<Vec<String>>()
                .join(", "),
            ServiceType::GoFfi => method.args
                .iter()
                .map(|arg| {
                    let ident = (&**arg.name).snake_ident();
                    let ty_name = self.codegen_item_ty(&arg.ty.kind);
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
            ServiceType::RustFfi => method.args
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
            ServiceType::GoFfi => method.args
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
            ServiceType::RustFfi => method.args
                .iter()
                .map(|arg| {
                    let ident = (&**arg.name).snake_ident();
                    let ty_name = self.codegen_item_ty(&arg.ty.kind);
                    if arg.ty.is_scalar() {
                        format!("{ident}: {ty_name}")
                    } else {
                        format!("{ident}: ::fcplug::RustFfiArg<{ty_name}>")
                    }
                })
                .collect::<Vec<String>>()
                .join(", "),
            ServiceType::GoFfi => method.args
                .iter()
                .map(|arg| {
                    let ident = (&**arg.name).snake_ident();
                    let ty_name = self.codegen_item_ty(&arg.ty.kind);
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
        let ty_name = self.codegen_item_ty(&method.ret.kind);
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
        let ty_name = self.codegen_item_ty(&method.ret.kind);
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
    #[inline]
    fn codegen_item_ty(&self, ty: &TyKind) -> String {
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
            TyKind::Vec(ty) => format!("::std::vec::Vec<{}>", self.codegen_item_ty(&ty.kind)),
            TyKind::Set(ty) => format!("::std::collections::HashSet<{}>", self.codegen_item_ty(&ty.kind)),
            TyKind::Map(key, value) => format!("::std::collections::HashMap<{}, {}>", self.codegen_item_ty(&key.kind), self.codegen_item_ty(&value.kind)),
            TyKind::Path(path) => self.context.rust_name(path.did).0.to_string(),
            TyKind::UInt32 => CodegenTy::UInt32.to_string(),
            TyKind::UInt64 => CodegenTy::UInt64.to_string(),
            TyKind::F32 => CodegenTy::F32.to_string(),
            TyKind::Arc(ty) => format!("::std::sync::Arc<{}>", self.codegen_item_ty(&ty.kind)),
        }
    }
}
