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
    #[inline]
    fn codegen_go_item_ty(&self, ty: &TyKind) -> String {
        match &ty {
            TyKind::Void => "ctypes.Void".to_string(),
            TyKind::U8 => "ctypes.Uint8".to_string(),
            TyKind::Bool => "ctypes.Bool".to_string(),
            TyKind::I8 => "ctypes.Int8".to_string(),
            TyKind::I16 => "ctypes.Int16".to_string(),
            TyKind::I32 => "ctypes.Int32".to_string(),
            TyKind::I64 => "ctypes.Int64".to_string(),
            TyKind::UInt32 => "ctypes.Uint32".to_string(),
            TyKind::UInt64 => "ctypes.Uint64".to_string(),
            TyKind::F32 => "ctypes.Float32".to_string(),
            TyKind::F64 => "ctypes.Float64".to_string(),
            TyKind::String => "ctypes.String".to_string(),
            TyKind::Bytes => "ctypes.Bytes".to_string(),
            TyKind::Vec(ty) | TyKind::Set(ty) => {
                format!("ctypes.Slice[{},{}]", self.codegen_go_item_ty(&ty.kind), self.codegen_c_item_ty(&ty.kind))
            }
            TyKind::Map(key, value) => {
                format!("ctypes.Map[{},{},{},{}]",
                        self.codegen_go_item_ty(&key.kind), self.codegen_go_item_ty(&value.kind),
                        self.codegen_c_item_ty(&key.kind), self.codegen_c_item_ty(&value.kind),
                )
            }
            TyKind::Path(path) => format!("G_{}", self.context.rust_name(path.did).0.to_string()),
            TyKind::Arc(ty) => format!("*{}", self.codegen_go_item_ty(&ty.kind)),
        }
    }
    #[inline]
    fn codegen_c_item_ty(&self, ty: &TyKind) -> String {
        match &ty {
            TyKind::Void => "ctypes.Void".to_string(),
            TyKind::U8 => "ctypes.Uint8".to_string(),
            TyKind::Bool => "ctypes.Bool".to_string(),
            TyKind::I8 => "ctypes.Int8".to_string(),
            TyKind::I16 => "ctypes.Int16".to_string(),
            TyKind::I32 => "ctypes.Int32".to_string(),
            TyKind::I64 => "ctypes.Int64".to_string(),
            TyKind::UInt32 => "ctypes.Uint32".to_string(),
            TyKind::UInt64 => "ctypes.Uint64".to_string(),
            TyKind::F32 => "ctypes.Float32".to_string(),
            TyKind::F64 => "ctypes.Float64".to_string(),
            TyKind::Bytes => "ctypes.C_Bytes".to_string(),
            TyKind::String => "ctypes.C_String".to_string(),
            TyKind::Vec(ty) | TyKind::Set(ty) => format!("ctypes.C_Slice[{},{}]", self.codegen_c_item_ty(&ty.kind), self.codegen_go_item_ty(&ty.kind)),
            TyKind::Map(key, value) => {
                format!("ctypes.C_Map[{},{},{},{}]",
                        self.codegen_c_item_ty(&key.kind), self.codegen_c_item_ty(&value.kind),
                        self.codegen_go_item_ty(&key.kind), self.codegen_go_item_ty(&value.kind),
                )
            }
            TyKind::Path(path) => format!("C_{}", self.context.rust_name(path.did).to_string()),
            TyKind::Arc(ty) => format!("*{}", self.codegen_c_item_ty(&ty.kind)),
        }
    }
    fn field_name(&self, f: &Arc<Field>) -> String {
        self.context.rust_name(f.did).0.upper_camel_ident().into_string()
    }
    fn field_tag(&self, f: &Arc<Field>) -> String {
        format!(r###"`json:"{}"`"###, self.context.rust_name(f.did).0.snake_ident())
    }
    pub(crate) fn codegen_struct_impl(&self, def_id: DefId, stream: &mut String, s: &Message) {
        let mut fields = String::new();
        let mut gfields = String::new();
        let mut cfields = String::new();
        s.fields
            .iter()
            .for_each(|f| {
                let name = self.field_name(f);
                let tag = self.field_tag(f);
                self.context.with_adjust(f.did, |adjust| {
                    let mut ty = self.codegen_item_ty(&f.ty.kind);
                    let mut ty2 = self.codegen_go_item_ty(&f.ty.kind);
                    let mut ty3 = self.codegen_c_item_ty(&f.ty.kind);
                    if codegen::is_raw_ptr_field(f, adjust) {
                        match f.ty.kind {
                            TyKind::Vec(_) | TyKind::Set(_) | TyKind::Map(_, _) | TyKind::Arc(_) => {}
                            _ => {
                                ty = format!("*{ty}");
                                ty2 = format!("*{ty2}");
                                ty3 = format!("*{ty3}");
                            }
                        }
                    }
                    fields.push_str(&format!("{name} {ty} {tag}\n"));
                    gfields.push_str(&format!("{name} {ty2} {tag}\n"));
                    cfields.push_str(&format!("{name} {ty3}\n"));
                })
            });
        let name = self.context.rust_name(def_id);
        stream.push_str(&format!(r###"type {name} struct {{
                {fields}
            }}
            type G_{name} struct {{
                {gfields}
            }}
            //go:inline
            //go:nosplit
            func (p *{name}) toObject() *G_{name} {{
                return (*G_{name})(unsafe.Pointer(p))
            }}
            //go:inline
            //go:nosplit
            func (p *G_{name}) toScalar() *{name} {{
                return (*{name})(unsafe.Pointer(p))
            }}
        "###));
        if s.is_all_in_stack() {
            stream.push_str(&format! {
                r#"type C_{name}=G_{name};
                "#
            });
        } else {
            stream.push_str(&format!(r###"type C_{name} struct {{
                {cfields}
            }}
            "###));
        }
        self.codegen_conv_repr_c_impl(def_id, stream, s);
    }
    fn codegen_conv_repr_c_impl(&self, def_id: DefId, stream: &mut String, s: &Message) {
        let name = self.context.rust_name(def_id);
        stream.push_str(&format!(r###"
        var (
            _ ctypes.ReprGoToC[C_{name}] = G_{name}{{}}
            _ ctypes.ReprCToGo[G_{name}]   = C_{name}{{}}
        )
        "###));
        if s.is_all_in_stack() {
            stream.push_str(&format! {r#"
            //go:inline
            //go:nosplit
            func (p G_{name}) ToReprC(_ *ctypes.KeepAliveRow) C_{name} {{
                return p
            }}
            //go:inline
            //go:nosplit
            func (p C_{name}) ToReprGo() G_{name} {{
                return p
            }}
            "#});
            return;
        }
        let mut repr_go_to_c_body = String::new();
        let mut repr_c_to_go_body = String::new();
        s.fields
            .iter()
            .for_each(|f| {
                let name = self.field_name(f);
                self.context.with_adjust(f.did, |adjust| {
                    if f.is_in_stack() {
                        repr_go_to_c_body.push_str(&format!("{name}: p.{name},\n"));
                        repr_c_to_go_body.push_str(&format!("{name}: p.{name},\n"));
                    } else {
                        let ty = self.codegen_go_item_ty(&f.ty.kind);
                        let cty = self.codegen_c_item_ty(&f.ty.kind);
                        if codegen::is_raw_ptr_field(f, adjust) {
                            match f.ty.kind {
                                TyKind::Vec(_) | TyKind::Set(_) | TyKind::Map(_, _) | TyKind::Arc(_) => {
                                    repr_go_to_c_body.push_str(&format!("{name}: p.{name}.ToReprC(keepAliveRow),\n"));
                                    repr_c_to_go_body.push_str(&format!("{name}: p.{name}.ToReprGo(),\n"));
                                }
                                _ => {
                                    repr_go_to_c_body.push_str(&format!(r###"{name}: func(_{name} *{ty}) *{cty} {{
                                        if _{name} != nil {{
                                            return nil
                                        }} else {{
                                            _C{name} := _{name}.ToReprC(keepAliveRow)
                                            return &_C{name}
                                        }}
                                    }}(p.{name}),
                                    "###));
                                    repr_c_to_go_body.push_str(&format!(r###"{name}: func(_{name} *{cty}) *{ty} {{
                                        if _{name} != nil {{
                                            return nil
                                        }} else {{
                                            _G{name} := _{name}.ToReprGo()
                                            return &_G{name}
                                        }}
                                    }}(p.{name}),
                                    "###));
                                }
                            }
                        } else {
                            repr_go_to_c_body.push_str(&format!("{name}: p.{name}.ToReprC(keepAliveRow),\n"));
                            repr_c_to_go_body.push_str(&format!("{name}: p.{name}.ToReprGo(),\n"));
                        }
                    }
                })
            });
        stream.push_str(&format! {r#"
            //go:inline
            //go:nosplit
            func (p G_{name}) ToReprC(keepAliveRow *ctypes.KeepAliveRow) C_{name} {{
                return C_{name}{{
                    {repr_go_to_c_body}
                }}
            }}
            //go:inline
            //go:nosplit
            func (p C_{name}) ToReprGo() G_{name} {{
                return G_{name}{{
                    {repr_c_to_go_body}
                }}
            }}
            "#});
    }
    pub(crate) fn codegen_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {}
}
