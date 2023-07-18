use std::sync::Arc;

use pilota_build::{codegen, DefId, IdentName, rir::Message, rir::Service};
use pilota_build::Context;
use pilota_build::rir::{Field, Method};
use pilota_build::ty::{TyKind};

use crate::ffidl::{Config, GoObjectPath};

#[derive(Clone)]
pub(crate) struct GoCodegenBackend {
    pub(crate) config: Arc<Config>,
    pub(crate) context: Arc<Context>,
}

impl GoCodegenBackend {
    #[inline]
    fn codegen_item_ty(&self, ty: &TyKind, is_main: bool) -> String {
        match &ty {
            TyKind::String => "string".to_string(),
            TyKind::Void => "".to_string(),
            TyKind::U8 => "uint8".to_string(),
            TyKind::Bool => "bool".to_string(),
            TyKind::Bytes => "[]byte".to_string(),
            TyKind::I8 => "int8".to_string(),
            TyKind::I16 => "int16".to_string(),
            TyKind::I32 => "int32".to_string(),
            TyKind::I64 => "int64".to_string(),
            TyKind::F64 => "float64".to_string(),
            TyKind::Vec(ty) => format!("[]{}", self.codegen_item_ty(&ty.kind, is_main)),
            TyKind::Set(ty) => format!("[]{}", self.codegen_item_ty(&ty.kind, is_main)),
            TyKind::Map(key, value) => format!("map[{}]{}", self.codegen_item_ty(&key.kind, is_main), self.codegen_item_ty(&value.kind, is_main)),
            TyKind::Path(path) => {
                let mut pkg_pre = String::new();
                if is_main {
                    pkg_pre = self.config.go_mod_name() + ".";
                }
                format!("{pkg_pre}{}", self.context.rust_name(path.did).0.to_string())
            }
            TyKind::UInt32 => "uint32".to_string(),
            TyKind::UInt64 => "uint64".to_string(),
            TyKind::F32 => "float32".to_string(),
            TyKind::Arc(ty) => format!("*{}", self.codegen_item_ty(&ty.kind, is_main)),
        }
    }
    #[inline]
    fn codegen_go_item_ty(&self, ty: &TyKind, is_main: bool) -> String {
        match &ty {
            TyKind::Void => "".to_string(),
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
                format!("ctypes.Slice[{},{}]", self.codegen_go_item_ty(&ty.kind, is_main), self.codegen_c_item_ty(&ty.kind, is_main))
            }
            TyKind::Map(key, value) => {
                format!("ctypes.Map[{},{},{},{}]",
                        self.codegen_go_item_ty(&key.kind, is_main), self.codegen_go_item_ty(&value.kind, is_main),
                        self.codegen_c_item_ty(&key.kind, is_main), self.codegen_c_item_ty(&value.kind, is_main),
                )
            }
            TyKind::Path(path) => {
                let mut pkg_pre = String::new();
                if is_main {
                    pkg_pre = self.config.go_mod_name() + ".";
                }
                format!("{pkg_pre}G_{}", self.context.rust_name(path.did).0.to_string())
            }
            TyKind::Arc(ty) => format!("*{}", self.codegen_go_item_ty(&ty.kind, is_main)),
        }
    }
    #[inline]
    fn codegen_c_item_ty(&self, ty: &TyKind, is_main: bool) -> String {
        match &ty {
            TyKind::Void => "".to_string(),
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
            TyKind::Vec(ty) | TyKind::Set(ty) => format!("ctypes.C_Slice[{},{}]", self.codegen_c_item_ty(&ty.kind, is_main), self.codegen_go_item_ty(&ty.kind, is_main)),
            TyKind::Map(key, value) => {
                format!("ctypes.C_Map[{},{},{},{}]",
                        self.codegen_c_item_ty(&key.kind, is_main), self.codegen_c_item_ty(&value.kind, is_main),
                        self.codegen_go_item_ty(&key.kind, is_main), self.codegen_go_item_ty(&value.kind, is_main),
                )
            }
            TyKind::Path(path) => {
                let mut pkg_pre = String::new();
                if is_main {
                    pkg_pre = self.config.go_mod_name() + ".";
                }
                format!("{pkg_pre}C_{}", self.context.rust_name(path.did).to_string())
            }
            TyKind::Arc(ty) => format!("*{}", self.codegen_c_item_ty(&ty.kind, is_main)),
        }
    }
    fn field_name(&self, f: &Arc<Field>) -> String {
        self.context.rust_name(f.did).0.upper_camel_ident().into_string()
    }
    fn field_tag(&self, f: &Arc<Field>) -> String {
        format!(r###"`json:"{}"`"###, self.context.rust_name(f.did).0.snake_ident())
    }
    fn struct_go_name(&self, message_def_id: DefId, s: &Message) -> String {
        let name = self.context.rust_name(message_def_id);
        if s.is_all_in_stack() {
            format!("C_{}", name)
        } else {
            format!("G_{}", name)
        }
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
                    let mut ty = self.codegen_item_ty(&f.ty.kind, false);
                    let mut ty2 = self.codegen_go_item_ty(&f.ty.kind, false);
                    let mut ty3 = self.codegen_c_item_ty(&f.ty.kind, false);
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
            func (p *{name}) ToObject() *G_{name} {{
                return (*G_{name})(unsafe.Pointer(p))
            }}
            //go:inline
            //go:nosplit
            func (p *G_{name}) ToScalar() *{name} {{
                return (*{name})(unsafe.Pointer(p))
            }}

            type C_{name} struct {{
                {cfields}
            }}

            func (p C_{name}) AsCgo () *C.struct_C_{name} {{
                return (*C.struct_C_{name})(unsafe.Pointer(&p))
            }}
        "###));
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
                return *(*C_{name})(unsafe.Pointer(&p))
            }}
            //go:inline
            //go:nosplit
            func (p C_{name}) ToReprGo() G_{name} {{
                return *(*G_{name})(unsafe.Pointer(&p))
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
                        let ty = self.codegen_go_item_ty(&f.ty.kind, false);
                        let cty = self.codegen_c_item_ty(&f.ty.kind, false);
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
    fn codegen_method_args(&self, service_def_id: DefId, method: &Method, is_c: bool, is_main: bool) -> String {
        match self.context.rust_name(service_def_id).to_lowercase().as_str() {
            "goffi" => method.args
                .iter()
                .map(|arg| format!("{} {}{}",
                                   (&**arg.name).snake_ident(),
                                   if arg.ty.is_in_stack() { "" } else { "*" },
                                   if is_c {
                                       self.codegen_c_item_ty(&arg.ty.kind, is_main)
                                   } else {
                                       self.codegen_item_ty(&arg.ty.kind, is_main)
                                   }
                ))
                .collect::<Vec<String>>()
                .join(", "),
            "rustffi" => method.args
                .iter()
                .map(|arg| format!("{} {}",
                                   (&**arg.name).snake_ident(),
                                   if is_c { self.codegen_c_item_ty(&arg.ty.kind, is_main) } else { self.codegen_item_ty(&arg.ty.kind, is_main) }
                ))
                .collect::<Vec<String>>()
                .join(", "),
            _ => { String::new() }
        }
    }
    fn codegen_method_ret(&self, service_def_id: DefId, method: &Arc<Method>, is_c: bool, is_main: bool) -> String {
        let t = if is_c { format!("{}.{}", self.config.go_mod_name(), self.codegen_c_item_ty(&method.ret.kind, is_main)) } else { self.codegen_item_ty(&method.ret.kind, is_main) };
        match self.context.rust_name(service_def_id).to_lowercase().as_str() {
            "goffi" => if method.ret.is_in_stack() {
                t
            } else {
                format!("*{t}")
            },
            "rustffi" => if is_c {
                t
            } else {
                self.rustffi_go_ret_ty(method, is_main)
            },
            _ => { String::new() }
        }
    }
    fn rustffi_go_ret_ty(&self, method: &Arc<Method>, is_main: bool) -> String {
        if method.ret.is_in_stack() {
            self.codegen_item_ty(&method.ret.kind, is_main)
        } else {
            format!("RustFfi_{}", self.go_method_name(method))
        }
    }
    fn go_method_name(&self, method: &Arc<Method>) -> String {
        (&**method.name).struct_ident().into_string()
    }
    fn c_method_name(&self, method: &Arc<Method>) -> String {
        (&**method.name).fn_ident().into_string()
    }
    pub(crate) fn codegen_service_interface(&self, service_def_id: DefId, stream: &mut String, s: &Service) {
        let name = self.context.rust_name(service_def_id);
        let name_lower = name.to_lowercase();

        let methods = s.methods.iter().map(|method| {
            let fn_name = self.go_method_name(method);
            let args = self.codegen_method_args(service_def_id, method, false, false);
            let ret = self.codegen_method_ret(service_def_id, method, false, false);
            format!(r###"{fn_name}({args}){ret}"###)
        }).collect::<Vec<String>>().join("\n");
        stream.push_str(&format!(r###"
        type {name} interface {{
            {methods}
        }}
        "###));

        match s.name.to_string().to_lowercase().as_str() {
            "rustffi" => {
                let methods = s.methods.iter().map(|method| {
                    let fn_name = self.go_method_name(method);
                    let c_fn_name = self.c_method_name(method);
                    let args = self.codegen_method_args(service_def_id, method, false, false);
                    let ret = self.codegen_method_ret(service_def_id, method, false, false);
                    let cty = self.codegen_c_item_ty(&method.ret.kind, false);
                    if !method.ret.is_in_stack() {
                        let cty = cty.trim_start_matches("*").to_string();
                        let go_ret_ty = self.rustffi_go_ret_ty(method, false);
                        let ty = self.codegen_item_ty(&method.ret.kind, false);
                        let args_to_c = method.args.iter().map(|arg| {
                            let ident = (&**arg.name).snake_ident();
                            if arg.ty.is_in_stack() {
                                if arg.ty.is_bool() {
                                    format!("*(*C._Bool)(unsafe.Pointer(&{ident}))")
                                } else {
                                    let cty = self.codegen_c_item_ty(&arg.ty.kind, false);
                                    format!("{cty}({ident})")
                                }
                            } else {
                                format!("*{ident}.ToObject().ToReprC(_keepAliveRow_{name_lower}).AsCgo()")
                            }
                        }).collect::<Vec<String>>().join(",");
                        let keep_alive = if args_to_c.is_empty() {
                            String::new()
                        } else {
                            format!("var _keepAliveRow_{name_lower} = keepAliveTable_{name_lower}_{c_fn_name}.NewRow()")
                        };
                        format!(r###"
                        var keepAliveTable_{name_lower}_{c_fn_name} = ctypes.NewKeepAliveTable("{name_lower}_{c_fn_name}")

                        type {go_ret_ty} struct {{
                            Ret *{ty}
                            cRetPtr *C.struct_C_{ty}
                        }}
                        func (p *{go_ret_ty}) Free() {{
                            if p!=nil && p.cRetPtr!=nil {{
                                C.{name_lower}_{c_fn_name}_free_ret(p.cRetPtr)
                            }}
                        }}
                        func (Impl{name}) {fn_name} ({args}) {ret} {{
	                        {keep_alive}
                            var _cRetPtr_{name_lower} = C.{name_lower}_{c_fn_name}({args_to_c})
                            return {go_ret_ty} {{
                                Ret: (*{ty})(unsafe.Pointer(ctypes.Ref((*{cty})(unsafe.Pointer(_cRetPtr_{name_lower})).ToReprGo()))),
                                cRetPtr: _cRetPtr_{name_lower},
                            }}
                        }}"###)
                    } else {
                        let args_to_c = method.args.iter().map(|arg| {
                            let ident = (&**arg.name).snake_ident();
                            if arg.ty.is_in_stack() {
                                if arg.ty.is_bool() {
                                    format!("*(*C._Bool)(unsafe.Pointer(&{ident}))")
                                } else {
                                    let cty = self.codegen_c_item_ty(&arg.ty.kind, false);
                                    format!("{cty}({ident})")
                                }
                            } else {
                                format!("*{ident}.ToObject().ToReprC(_keepAliveRow_{name_lower}).AsCgo()")
                            }
                        }).collect::<Vec<String>>().join(",");
                        let ty = self.codegen_item_ty(&method.ret.kind, false);
                        if method.ret.is_scalar() {
                            format!(r###"func (Impl{name}) {fn_name} ({args}) {ret} {{
                                return {ty}(({cty})(C.{name_lower}_{c_fn_name}({args_to_c})))
                            }}
                            "###)
                        } else {
                            format!(r###"func (Impl{name}) {fn_name} ({args}) {ret} {{
                            return *(*{ty})(unsafe.Pointer(ctypes.Ref(C.{name_lower}_{c_fn_name}({args_to_c}))))
                        }}
                        "###)
                        }
                    }
                }).collect::<Vec<String>>().join("\n");
                stream.push_str(&format!(r###"
                type Impl{name} struct {{}}
                    var _ {name} = Impl{name}{{}}
                    {methods}
                "###));
            }
            _ => {}
        };
    }
    pub(crate) fn codegen_service_export(&self, service_def_id: DefId, stream: &mut String, s: &Service) {
        let name = self.context.rust_name(service_def_id);
        let name_lower = name.to_lowercase();


        let interface_name = self.context.rust_name(service_def_id);
        let pkg_name = self.config.dir_name();
        let GoObjectPath { import, mut object_ident } = self.config.goffi_impl_of_object.clone().unwrap_or(GoObjectPath {
            import: String::new(),
            object_ident: format!("Unimplemented{interface_name}"),
        });

        let mut import_list = format!(r###""{}"
        "###, self.config.go_mod_path);

        let mut impl_object = String::new();

        if !import.is_empty() {
            object_ident = format!("{pkg_name}.{object_ident}");
            import_list.push_str(&format!(r###""{}"
            "###, import));
        } else {
            impl_object.push_str(&format!("type {object_ident} struct{{}}\n", ));
            impl_object.push_str(&s.methods.iter().map(|method| {
                let iface_fn_name = self.go_method_name(method);
                let args = self.codegen_method_args(service_def_id, method, false, true);
                let ret = self.codegen_method_ret(service_def_id, method, false, true);
                format!(r###"//go:inline
                //go:nosplit
                func ({object_ident}) {iface_fn_name}({args}){ret} {{ panic("unimplemented!") }}"###)
            }).collect::<Vec<String>>().join("\n"));
            object_ident.push_str("{}");
        }

        let func_list = s.methods.iter().map(|method| {
            let iface_fn_name = self.go_method_name(method);
            let c_fn_name = self.c_method_name(method);
            let args = self.codegen_method_args(service_def_id, method, true, true);
            let ret_c_ty = self.codegen_c_item_ty(&method.ret.kind, true);

            let args_to_go = method.args
                .iter()
                .map(|arg| {
                    if arg.ty.is_in_stack() {
                        format!("{}({})", self.codegen_item_ty(&arg.ty.kind, true), (&**arg.name).snake_ident().into_string())
                    } else {
                        let ty = self.codegen_item_ty(&arg.ty.kind, true);
                        format!("(*{ty})(unsafe.Pointer(ctypes.Ref({}.ToReprGo())))", (&**arg.name).snake_ident())
                    }
                })
                .collect::<Vec<String>>()
                .join(", ");

            let gty = self.codegen_go_item_ty(&method.ret.kind, true);
            if method.ret.is_in_stack() {
                format!(r###"//go:inline
                //go:nosplit
                //export {name_lower}_{c_fn_name}
                func {name_lower}_{c_fn_name}({args}) {ret_c_ty} {{
                    return *(*{gty})(unsafe.Pointer(ctypes.Ref({object_ident}.{iface_fn_name}({args_to_go}))))
                }}
                "###)
            } else {
                format!(r###"
                var keepAliveTable_{name_lower}_{c_fn_name} = ctypes.NewKeepAliveTable("{name_lower}_{c_fn_name}")
                //go:inline
                //go:nosplit
                //export {name_lower}_{c_fn_name}
                func {name_lower}_{c_fn_name}({args}) *{ret_c_ty} {{
	                var _keepAliveRow_{name_lower} = keepAliveTable_{name_lower}_{c_fn_name}.NewRow()
                    var _c_ret = (*{gty})(unsafe.Pointer({object_ident}.{iface_fn_name}({args_to_go}))).ToReprC(_keepAliveRow_{name_lower})
                    keepAliveTable_{name_lower}_{c_fn_name}.KeepRowWithPtr(_keepAliveRow_{name_lower},unsafe.Pointer(&_c_ret))
                    return &_c_ret
                }}
                "###)
            }
        })
            .collect::<Vec<String>>()
            .join("\n");

        stream.push_str(&format!(
            r###"package main
                import "C"

                import (
                    "unsafe"

                    "github.com/andeya/fcplug/go/ctypes"
                    {import_list}
                )

	            var (
                    _ unsafe.Pointer
                    _ ctypes.C_DynArray[any]
                )

                func main() {{}}

                var _ {pkg_name}.{interface_name} = {object_ident}

                {impl_object}

                {func_list}
                "###,
        ));
    }
}
