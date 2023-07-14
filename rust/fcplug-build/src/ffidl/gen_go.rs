use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::anyhow;
use itertools::Itertools;
use pilota_build::{Adjust, CodegenBackend, Context, DefId, IdentName, MakeBackend, Output};
use pilota_build::rir::{Field, Message, Method};
use pilota_build::ty::{CodegenTy, TyKind};
use pilota_thrift_parser::{File, Item, parser::Parser, Service, Struct};


struct GoMakeBackend;

impl MakeBackend for GoMakeBackend {
    type Target = GoCodegenBackend;

    fn make_backend(self, context: Context) -> Self::Target {
        GoCodegenBackend { context }
    }
}

#[derive(Clone)]
struct GoCodegenBackend {
    context: Context,
}

impl CodegenBackend for GoCodegenBackend {
    fn cx(&self) -> &Context {
        &self.context
    }
    fn codegen_service_method(&self, _service_def_id: DefId, method: &Method) -> String {
        format!("fn {}({}) -> {};",
                (&**method.name).fn_ident(),
                method.args
                    .iter()
                    .map(|arg| format!("{}: {}", (&**arg.name).snake_ident(), self.codegen_item_ty(&arg.ty.kind)))
                    .collect::<Vec<String>>()
                    .join(", "),
                self.codegen_item_ty(&method.ret.kind))
    }
    fn codegen_struct_impl(&self, def_id: DefId, stream: &mut String, s: &Message) {
        self.codegen_c_struct(def_id, stream, s);
        self.codegen_conv_repr_c_impl(def_id, stream, s);
    }
}

impl GoCodegenBackend {
    fn codegen_c_struct(&self, def_id: DefId, stream: &mut String, s: &Message) {
        let name = self.context.rust_name(def_id);
        if Self::has_all_scalar_fields(s) {
            stream.push_str(&format! {
                r#"pub type C_{name}={name};"#
            });
            return;
        }
        let fields = s
            .fields
            .iter()
            .map(|f| {
                let name = self.context.rust_name(f.did);
                self.context.with_adjust(f.did, |adjust| {
                    let mut ty = self.codegen_c_struct_item_ty(&f.ty.kind);

                    if Self::is_raw_ptr_field(f, adjust) {
                        ty = format!("*mut {ty}")
                    }

                    let attrs = adjust.iter().flat_map(|a| a.attrs()).join("");

                    format! {
                        r#"{attrs}
                        pub {name}: {ty},"#
                    }
                })
            })
            .join("\n");

        stream.push_str(&format! {
            r#"#[derive(Clone, PartialEq)]
            #[repr(C)]
            pub struct C_{name} {{
                {fields}
            }}"#
        });
    }
    fn codegen_conv_repr_c_impl(&self, def_id: DefId, stream: &mut String, s: &Message) {
        let name = self.context.rust_name(def_id);
        if Self::has_all_scalar_fields(s) {
            stream.push_str(&format! {r#"
                impl ::fcplug::ctypes::ConvReprC for {name} {{
                    type ReprC = C_{name};
                    #[inline(always)]
                    fn into_repr_c(self) -> Self::ReprC {{
                        self
                    }}
                    #[inline(always)]
                    fn from_repr_c(c: Self::ReprC) -> Self {{
                        c
                    }}
                }}
            "#});
            return;
        }
        let field_names = s.fields.iter().map(|f| self.context.rust_name(f.did).to_string()).collect::<Vec<String>>().join(",");
        let mut into_repr_c_body = format!(
            "let {name}{{ {field_names} }} = self;",
        );
        let mut from_repr_c_body = format!(
            "let C_{name}{{ {field_names} }} = c;",
        );

        into_repr_c_body.push_str(&s
            .fields
            .iter()
            .map(|f| {
                let name = self.context.rust_name(f.did);
                self.context.with_adjust(f.did, |adjust| {
                    if f.is_optional() && !adjust.map_or(false, |adjust| adjust.boxed()) {
                        format!(r###"let {name} = if let Some({name}) = {name} {{
                            ::std::boxed::Box::into_raw(::std::boxed::Box::new({name}.into_repr_c()))
                        }} else {{
                            ::std::ptr::null_mut()
                        }};"###)
                    } else if Self::is_scalar_type(&f.ty.kind) {
                        String::new()
                    } else {
                        format!(r###"let {name} = {name}.into_repr_c();"###)
                    }
                })
            })
            .join("\n"));

        from_repr_c_body.push_str(&s
            .fields
            .iter()
            .map(|f| {
                let name = self.context.rust_name(f.did);
                self.context.with_adjust(f.did, |adjust| {
                    let ty = self.codegen_item_ty(&f.ty.kind);
                    if f.is_optional() && !adjust.map_or(false, |adjust| adjust.boxed()) {
                        format!(r###"let {name} = if {name}.is_null() {{
                            ::std::option::Option::None
                        }} else {{
                            ::std::option::Option::Some(::fcplug::ctypes::ConvReprC::from_repr_c(unsafe {{ *::std::boxed::Box::from_raw({name}) }}))
                        }};"###)
                    } else if Self::is_scalar_type(&f.ty.kind) {
                        String::new()
                    } else {
                        format!(r###"let {name} = ::fcplug::ctypes::ConvReprC::from_repr_c({name});"###)
                    }
                })
            })
            .join("\n"));

        into_repr_c_body.push_str(&format!(r#"
        C_{name}{{{field_names}}}
        "#));
        from_repr_c_body.push_str(&format!(r#"
        {name}{{{field_names}}}
        "#));

        stream.push_str(&format! {
            r#"impl ::fcplug::ctypes::ConvReprC for {name} {{
                type ReprC = C_{name};
                #[inline(always)]
                fn into_repr_c(self) -> Self::ReprC {{
                    {into_repr_c_body}
                }}
                #[inline(always)]
                fn from_repr_c(c: Self::ReprC) -> Self {{
                    {from_repr_c_body}
                }}
            }}"#
        });
    }

    fn is_raw_ptr_field(f: &Arc<Field>, adjust: Option<&Adjust>) -> bool {
        f.is_optional() || adjust.map_or(false, |adjust| adjust.boxed())
    }
    fn has_all_scalar_fields(s: &Message) -> bool {
        s.fields.iter().all(|f| Self::is_scalar_type(&f.ty.kind))
    }
    fn is_scalar_type(ty: &TyKind) -> bool {
        match &ty {
            TyKind::Void | TyKind::U8 | TyKind::Bool | TyKind::I8 | TyKind::I16 | TyKind::I32 | TyKind::I64 | TyKind::F64 | TyKind::UInt32 | TyKind::UInt64 | TyKind::F32 => true,
            _ => false,
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
    #[inline]
    fn codegen_c_struct_item_ty(&self, ty: &TyKind) -> String {
        match &ty {
            TyKind::String => "::fcplug::ctypes::C_String".to_string(),
            TyKind::Void => CodegenTy::Void.to_string(),
            TyKind::U8 => CodegenTy::U8.to_string(),
            TyKind::Bool => CodegenTy::Bool.to_string(),
            TyKind::Bytes => "::fcplug::ctypes::C_Bytes".to_string(),
            TyKind::I8 => CodegenTy::I8.to_string(),
            TyKind::I16 => CodegenTy::I16.to_string(),
            TyKind::I32 => CodegenTy::I32.to_string(),
            TyKind::I64 => CodegenTy::I64.to_string(),
            TyKind::F64 => CodegenTy::F64.to_string(),
            TyKind::Vec(ty) => format!("::fcplug::ctypes::C_DynArray<{}>", self.codegen_c_struct_item_ty(&ty.kind)),
            TyKind::Set(ty) => format!("::fcplug::ctypes::C_DynArray<{}>", self.codegen_c_struct_item_ty(&ty.kind)),
            TyKind::Map(key, value) => format!("::fcplug::ctypes::C_Map<{}, {}>", self.codegen_c_struct_item_ty(&key.kind), self.codegen_c_struct_item_ty(&value.kind)),
            TyKind::Path(path) => format!("C_{}", self.context.rust_name(path.did).0.to_string()),
            TyKind::UInt32 => CodegenTy::UInt32.to_string(),
            TyKind::UInt64 => CodegenTy::UInt64.to_string(),
            TyKind::F32 => CodegenTy::F32.to_string(),
            TyKind::Arc(ty) => if Self::is_scalar_type(&ty.kind) {
                self.codegen_c_struct_item_ty(&ty.kind)
            } else {
                format!("*const {}", self.codegen_c_struct_item_ty(&ty.kind))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ffidl::{Config, FFIDL};

    #[test]
    fn test_thriftast() {
        FFIDL::generate(Config {
            file_path: "/Users/henrylee2cn/rust/fcplug/ffidl_demo/ffidl.thrift"
                .into(),
            rust_out_path:
            "/Users/henrylee2cn/rust/fcplug/ffidl_demo/src/gen/ffidl.rs".into(),
        })
            .unwrap();
    }

    #[test]
    fn test_gen_header() {
        cbindgen::Builder::new()
            .with_crate("/Users/henrylee2cn/rust/fcplug/ffidl_demo")
            .with_src("/Users/henrylee2cn/rust/fcplug/rust/fcplug/src/ctypes.rs")
            .with_language(cbindgen::Language::C)
            .generate()
            .unwrap()
            .write_to_file("/Users/henrylee2cn/rust/fcplug/ffidl_demo/src/gen/ffidl.h");
    }
}
