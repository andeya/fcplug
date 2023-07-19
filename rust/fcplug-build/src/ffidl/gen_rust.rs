use std::sync::Arc;

use itertools::Itertools;
use pilota_build::{Context, DefId, IdentName};
use pilota_build::rir::{Method, Service};
use pilota_build::ty::{CodegenTy, TyKind};

use crate::ffidl::Config;

#[derive(Clone)]
pub(crate) struct RustCodegenBackend {
    pub(crate) config: Arc<Config>,
    pub(crate) context: Arc<Context>,
}

impl RustCodegenBackend {
    pub(crate) fn codegen_service_method(&self, service_def_id: DefId, method: &Method) -> String {
        let name = (&**method.name).fn_ident();
        let args = self.codegen_method_args(service_def_id, method);
        let ret = self.codegen_method_ret(service_def_id, method);
        match self.context.rust_name(service_def_id).to_lowercase().as_str() {
            "rustffi" => format!("fn {name}({args}) -> {ret};"),
            "goffi" => format!("unsafe fn {name}({args}) -> {ret};"),
            _ => { String::new() }
        }
    }
    pub(crate) fn codegen_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        match s.name.to_string().to_lowercase().as_str() {
            "rustffi" => self.codegen_rustffi_service_impl(def_id, stream, s),
            "goffi" => self.codegen_goffi_service_impl(def_id, stream, s),
            _ => {}
        };
    }
    fn codegen_method_args(&self, service_def_id: DefId, method: &Method) -> String {
        match self.context.rust_name(service_def_id).to_lowercase().as_str() {
            "rustffi" => method.args
                .iter()
                .map(|arg| format!("{}: &{}", (&**arg.name).snake_ident(), self.codegen_item_ty(&arg.ty.kind)))
                .collect::<Vec<String>>()
                .join(", "),
            "goffi" => method.args
                .iter()
                .map(|arg| format!("{}: {}", (&**arg.name).snake_ident(), self.codegen_item_ty(&arg.ty.kind)))
                .collect::<Vec<String>>()
                .join(", "),
            _ => { String::new() }
        }
    }
    fn codegen_method_ret(&self, _service_def_id: DefId, method: &Method) -> String {
        format!("::fcplug::ABIResult<{}>",self.codegen_item_ty(&method.ret.kind))
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
    #[inline]
    fn codegen_rustffi_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        let name = self.context.rust_name(def_id);
        let name_lower = name.to_lowercase();
        let ust = if let Some(ust) = &self.config.rustffi_impl_of_unit_struct {
            ust.0.to_string()
        } else {
            let methods = s.methods.iter().map(|method| {
                let name = (&**method.name).fn_ident();
                let args = self.codegen_method_args(def_id, method);
                let ret = self.codegen_method_ret(def_id, method);
                format!("fn {name}({args}) -> {ret} {{ unimplemented!() }}")
            }).collect::<Vec<String>>()
                .join("\n");
            let ust = format!("Unimplemented{name}");
            stream.push_str(&format!(r###"struct {ust};
            impl {name} for {ust} {{
                {methods}
            }}"###));
            ust
        };

        stream.push_str(&s.methods.iter().map(|method| {
            let fn_name = (&**method.name).fn_ident();
            format!(r###"#[no_mangle]
                #[inline]
                pub extern "C" fn {name_lower}_{fn_name}(mut req: ::fcplug::Buffer) -> ::fcplug::callee::FFIResult {{
                    ::fcplug::callee::protobuf::callback("{name_lower}_{fn_name}", <{ust} as {name}>::{fn_name}, &mut req)
                }}
                "###)
        })
            .collect::<Vec<String>>()
            .join("\n"));
    }
    fn codegen_goffi_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        self.codegen_goffi_functions(def_id, stream, s);
        self.codegen_goffi_impl(def_id, stream, s);
    }
    fn codegen_goffi_functions(&self, def_id: DefId, stream: &mut String, s: &Service) {
        let name = self.context.rust_name(def_id);
        let name_lower = name.to_lowercase();
        let ffi_fns = s.methods.iter().map(|method| {
            let fn_name = (&**method.name).fn_ident();
            format!("fn {name_lower}_{fn_name}(mut req: ::fcplug::Buffer) -> ::fcplug::caller::FFIResult;")
        })
            .collect::<Vec<String>>()
            .join("\n");
        stream.push_str(&format!(r###"extern "C" {{
            {ffi_fns}
        }}
        "###));
    }
    fn codegen_goffi_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        let name = self.context.rust_name(def_id);
        let name_lower = name.to_lowercase();
        let impl_methods = s.methods.iter().map(|method| {
            let name = (&**method.name).fn_ident();
            let args = self.codegen_method_args(def_id, method);
            let ret = self.codegen_method_ret(def_id, method);
            let args_into_c_repr = method.args
                .iter()
                .filter(|arg| !arg.ty.is_in_stack())
                .map(|arg| {
                    let ident = (&**arg.name).snake_ident();
                    format!("let {ident} = ::std::boxed::Box::into_raw(::std::boxed::Box::new(::fcplug::ctypes::ConvRepr::into_c_repr({ident})));")
                })
                .collect::<Vec<String>>()
                .join("\n");

            let c_args = method.args
                .iter()
                .map(|arg| {
                    (&**arg.name).snake_ident().to_string()
                })
                .collect::<Vec<String>>()
                .join(", ");


            if method.ret.is_in_stack() {
                let free_args = method.args
                    .iter()
                    .filter(|arg| !arg.ty.is_in_stack())
                    .map(|arg| {
                        let ident = (&**arg.name).snake_ident();
                        let ty = self.codegen_item_ty(&arg.ty.kind);
                        format!("let _ = <{ty} as ::fcplug::ctypes::ConvRepr>::from_c_repr(*::std::boxed::Box::from_raw({ident}));")
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                format!(r###"unsafe fn {name}({args}) -> {ret} {{
                    {args_into_c_repr}
                    let ret__ = {name_lower}_{name}({c_args});
                    {free_args}
                    ret__
                }}"###)
            } else {
                let mut c_args_tuple = method.args
                    .iter()
                    .filter(|arg| !arg.ty.is_in_stack())
                    .map(|arg| (&**arg.name).snake_ident().to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                if c_args_tuple.contains(',') {
                    c_args_tuple = format!("({c_args_tuple})")
                } else if c_args_tuple.len() == 0 {
                    c_args_tuple = "::std::ptr::null_mut::<()>()".to_string()
                }
                let raw_ret_ty = self.codegen_item_ty(&method.ret.kind);
                let goffi_free_name = self.goffi_free_name(def_id, method);
                format!(r###"unsafe fn {name}({args}) -> {ret} {{
                    {args_into_c_repr}
                    let c_ret__ = {name_lower}_{name}({c_args});
                    let ret__ = <{raw_ret_ty} as ::fcplug::ctypes::ConvRepr>::from_c_repr(*::std::boxed::Box::from_raw(c_ret__));
                    ::fcplug::ctypes::GoFfiResult::new(
                        ret__,
                        {c_args_tuple},
                        c_ret__ as usize,
                        {goffi_free_name},
                    )
                }}"###)
            }
        }).collect::<Vec<String>>().join("\n");
        stream.push_str(&format!(r###"pub struct Impl{name};
        impl {name} for Impl{name}{{
            {impl_methods}
        }}
        "###))
    }
    fn goffi_free_name(&self, service_def_id: DefId, method: &Method) -> String {
        let name_lower = self.context.rust_name(service_def_id).to_lowercase();
        let fn_name = (&**method.name).fn_ident();
        format!("{name_lower}_{fn_name}_free_ret")
    }
}
