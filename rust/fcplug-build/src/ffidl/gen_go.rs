use std::sync::Arc;

use pilota_build::{codegen, DefId, IdentName, rir::Message, rir::Service};
use pilota_build::Context;
use pilota_build::rir::Field;
use pilota_build::ty::TyKind;

use crate::ffidl::Config;

#[derive(Clone)]
pub(crate) struct GoCodegenBackend {
    pub(crate) config: Arc<Config>,
    pub(crate) context: Arc<Context>,
}

impl GoCodegenBackend {
    fn cx(&self) -> &Context {
        self.context.as_ref()
    }
    #[inline]
    fn codegen_item_ty(&self, ty: &TyKind) -> String {
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
            TyKind::Vec(ty) => format!("[]{}", self.codegen_item_ty(&ty.kind)),
            TyKind::Set(ty) => format!("[]{}", self.codegen_item_ty(&ty.kind)),
            TyKind::Map(key, value) => format!("map[{}]{}", self.codegen_item_ty(&key.kind), self.codegen_item_ty(&value.kind)),
            TyKind::Path(path) => self.context.rust_name(path.did).0.to_string(),
            TyKind::UInt32 => "uint32".to_string(),
            TyKind::UInt64 => "uint64".to_string(),
            TyKind::F32 => "float32".to_string(),
            TyKind::Arc(ty) => format!("*{}", self.codegen_item_ty(&ty.kind)),
        }
    }
    fn field_name(&self, f: &Arc<Field>) -> String {
        self.context.rust_name(f.did).0.upper_camel_ident().into_string()
    }
    fn field_tag(&self, f: &Arc<Field>) -> String {
        format!(r###"`json:"{}"`"###, self.context.rust_name(f.did).0.snake_ident())
    }
    pub(crate) fn codegen_struct_impl(&self, def_id: DefId, stream: &mut String, s: &Message) {
        let fields = s.fields
            .iter()
            .map(|f| {
                let name = self.field_name(f);
                let tag = self.field_tag(f);
                self.context.with_adjust(f.did, |adjust| {
                    let mut ty = self.codegen_item_ty(&f.ty.kind);
                    if codegen::is_raw_ptr_field(f, adjust) {
                        match f.ty.kind {
                            TyKind::Vec(_) | TyKind::Set(_) | TyKind::Map(_, _) | TyKind::Arc(_) => {}
                            _ => ty = format!("*{ty}")
                        }
                    }
                    format!("{name} {ty} {tag}")
                })
            })
            .collect::<Vec<String>>()
            .join("\n");
        let name = self.context.rust_name(def_id);
        stream.push_str(&format!(r###"type {name} struct {{
            {fields}
        }}
        "###));

        // cgo types

    }
    pub(crate) fn codegen_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {

    }
}
