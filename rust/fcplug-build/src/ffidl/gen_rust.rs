use std::cell::RefCell;
use std::sync::Arc;

use pilota_build::{DefId, IdentName};
use pilota_build::rir::{Method, Service};
use pilota_build::ty::{CodegenTy, TyKind};

use crate::ffidl::Config;
use crate::ffidl::make_backend::{Cx, ServiceType};

#[derive(Clone)]
pub(crate) struct RustCodegenBackend {
    pub(crate) config: Arc<Config>,
    pub(crate) context: Cx,
    pub(crate) rust_impl_rustffi_code: Arc<RefCell<String>>,
    pub(crate) rust_impl_goffi_code: Arc<RefCell<String>>,
}

impl RustCodegenBackend {
    pub(crate) fn codegen_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        match self.context.service_type(def_id) {
            ServiceType::RustFfi => self.codegen_rustffi_service_impl(def_id, stream, s),
            ServiceType::GoFfi => self.codegen_goffi_service_impl(def_id, stream, s),
        }
    }
    fn codegen_rustffi_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        let name = self.context.rust_name(def_id);
        let name_lower = name.to_lowercase();
        let ust = self.config.rust_mod_impl_name();

        let mut trait_methods = String::new();
        let mut impl_trait_methods = String::new();
        s.methods
            .iter()
            .for_each(|method| {
                let method_name = (&**method.name).fn_ident();
                let args = self.codegen_method_args(def_id, method);
                let ret = self.codegen_method_ret(def_id, method);
                trait_methods.push_str(&format!("fn {method_name}({args}) -> {ret};\n"));
                impl_trait_methods.push_str(&format!("fn {method_name}({args}) -> {ret} {{ todo!() }}\n"));
            });
        stream.push_str(&format! {r#"
        pub(super) trait {name} {{
            {trait_methods}
        }}
        "#});
        self.rust_impl_rustffi_code.borrow_mut().push_str(&format!(
            r###"
            impl {name} for {ust} {{
                {impl_trait_methods}
            }}
            "###
        ));

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
    fn codegen_goffi_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        let name = self.context.rust_name(def_id);
        let name_lower = name.to_lowercase();
        let ust = self.config.rust_mod_impl_name();

        let methods = s.methods
            .iter()
            .map(|method| {
                let method_name = (&**method.name).fn_ident();
                let args = self.codegen_method_args(def_id, method);
                let ret = self.codegen_method_ret(def_id, method);
                let generic_signature = if self.context.is_empty_ty(&method.ret.kind) {
                    String::new()
                } else {
                    "<T: Default>".to_string()
                };
                let args_ident = self.codegen_ffi_args_ident(def_id, method);
                format!(
                    r###"unsafe fn {method_name}{generic_signature}({args}) -> {ret} {{
                    ::fcplug::ABIResult::from({name_lower}_{method_name}({args_ident}))
                }}
                "###
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        stream.push_str(&format! {r#"
        pub trait {name}Call {{
            {methods}
        }}
        "#});

        let mut trait_methods = String::new();
        let mut impl_trait_methods = String::new();
        s.methods
            .iter()
            .filter(|method| { !self.context.is_empty_ty(&method.ret.kind) && !method.ret.is_scalar() })
            .for_each(|method| {
                let method_name = (&**method.name).fn_ident();
                let ffi_ret = self.codegen_ffi_ret(def_id, method);
                let ret_ty_name = self.codegen_item_ty(&method.ret.kind);
                trait_methods.push_str(&format!("unsafe fn {method_name}_set_result(go_ret: ::fcplug::RustFfiArg<{ret_ty_name}>) -> {ffi_ret};\n"));
                impl_trait_methods.push_str(&format!("unsafe fn {method_name}_set_result(go_ret: ::fcplug::RustFfiArg<{ret_ty_name}>) -> {ffi_ret} {{ todo!() }}\n"));
            });
        stream.push_str(&format! {r#"
        pub(super) trait {name} {{
            {trait_methods}
        }}
        "#});
        self.rust_impl_goffi_code.borrow_mut().push_str(&format!(
            r###"
            impl {name} for {ust} {{
                {impl_trait_methods}
            }}
            "###
        ));

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
            r###"extern "C" {{
            {ffi_fns}
        }}
        "###
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

impl RustCodegenBackend {
    fn codegen_ffi_args_param(&self, service_def_id: DefId, method: &Method) -> String {
        match self.context.service_type(service_def_id) {
            ServiceType::RustFfi => method
                .args
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
            ServiceType::GoFfi => method
                .args
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
                    let ty_name = self.codegen_item_ty(&arg.ty.kind);
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
            TyKind::Set(ty) => format!(
                "::std::collections::HashSet<{}>",
                self.codegen_item_ty(&ty.kind)
            ),
            TyKind::Map(key, value) => format!(
                "::std::collections::HashMap<{}, {}>",
                self.codegen_item_ty(&key.kind),
                self.codegen_item_ty(&value.kind)
            ),
            TyKind::Path(path) => self.context.rust_name(path.did).0.to_string(),
            TyKind::UInt32 => CodegenTy::UInt32.to_string(),
            TyKind::UInt64 => CodegenTy::UInt64.to_string(),
            TyKind::F32 => CodegenTy::F32.to_string(),
            TyKind::Arc(ty) => format!("::std::sync::Arc<{}>", self.codegen_item_ty(&ty.kind)),
        }
    }
}
