use std::cell::RefCell;

use itertools::Itertools;
use pilota_build::{codegen, CodegenBackend, Context, DefId, IdentName, MakeBackend};
use pilota_build::rir::{Message, Method, Service};
use pilota_build::ty::{CodegenTy, TyKind};

use crate::ffidl::Config;

pub(crate) struct RustMakeBackend {
    pub(crate) config: Config,
}

impl MakeBackend for RustMakeBackend {
    type Target = RustCodegenBackend;

    fn make_backend(self, context: Context) -> Self::Target {
        RustCodegenBackend { config: self.config, context, non_stack_messages: RefCell::new(vec![]) }
    }
}

#[derive(Clone)]
pub(crate) struct RustCodegenBackend {
    config: Config,
    context: Context,
    non_stack_messages: RefCell<Vec<DefId>>,
}

impl CodegenBackend for RustCodegenBackend {
    fn cx(&self) -> &Context {
        &self.context
    }
    fn codegen_service_method(&self, service_def_id: DefId, method: &Method) -> String {
        let name = (&**method.name).fn_ident();
        let args = self.codegen_method_args(service_def_id, method);
        let ret = self.codegen_method_ret(service_def_id, method);
        match self.context.rust_name(service_def_id).to_lowercase().as_str() {
            "rustffi" => format!("fn {name}({args}) -> {ret};"),
            "goffi" => format!("unsafe fn {name}({args}) -> {ret};"),
            _ => { String::new() }
        }
    }
    fn codegen_struct_impl(&self, def_id: DefId, stream: &mut String, s: &Message) {
        if !s.is_all_in_stack() {
            self.non_stack_messages.borrow_mut().push(def_id)
        }
        self.codegen_c_struct(def_id, stream, s);
        self.codegen_conv_repr_c_impl(def_id, stream, s);
    }
    fn codegen_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        match s.name.to_string().to_lowercase().as_str() {
            "rustffi" => self.codegen_rustffi_service_impl(def_id, stream, s),
            "goffi" => self.codegen_goffi_service_impl(def_id, stream, s),
            _ => {}
        };
    }
}

impl RustCodegenBackend {
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
    fn codegen_method_ret(&self, service_def_id: DefId, method: &Method) -> String {
        match self.context.rust_name(service_def_id).to_lowercase().as_str() {
            "rustffi" => self.codegen_item_ty(&method.ret.kind),
            "goffi" => if method.ret.is_in_stack() {
                self.codegen_item_ty(&method.ret.kind)
            } else {
                let ret_ty = self.codegen_item_ty(&method.ret.kind);
                let args_inner = method.args
                    .iter()
                    .filter(|arg| !arg.ty.is_in_stack())
                    .map(|arg| self.codegen_item_ty(&arg.ty.kind))
                    .collect::<Vec<String>>()
                    .join(", ");
                if args_inner.contains(',') {
                    format!("::fcplug::ctypes::GoFfiResult<{ret_ty}, ({args_inner})>")
                } else if args_inner.len() > 0 {
                    format!("::fcplug::ctypes::GoFfiResult<{ret_ty}, {args_inner}>")
                } else {
                    format!("::fcplug::ctypes::GoFfiResult<{ret_ty}, ()>")
                }
            },
            _ => { String::new() }
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
    fn codegen_c_item_ty(&self, ty: &TyKind) -> String {
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
            TyKind::Vec(ty) => format!("::fcplug::ctypes::C_DynArray<{}>", self.codegen_c_item_ty(&ty.kind)),
            TyKind::Set(ty) => format!("::fcplug::ctypes::C_DynArray<{}>", self.codegen_c_item_ty(&ty.kind)),
            TyKind::Map(key, value) => format!("::fcplug::ctypes::C_Map<{}, {}>", self.codegen_c_item_ty(&key.kind), self.codegen_c_item_ty(&value.kind)),
            TyKind::Path(path) => format!("C_{}", self.context.rust_name(path.did).to_string()),
            TyKind::UInt32 => CodegenTy::UInt32.to_string(),
            TyKind::UInt64 => CodegenTy::UInt64.to_string(),
            TyKind::F32 => CodegenTy::F32.to_string(),
            TyKind::Arc(ty) => if ty.is_in_stack() {
                self.codegen_c_item_ty(&ty.kind)
            } else {
                format!("*const {}", self.codegen_c_item_ty(&ty.kind))
            },
        }
    }
    fn codegen_c_struct(&self, def_id: DefId, stream: &mut String, s: &Message) {
        let name = self.context.rust_name(def_id);
        if s.is_all_in_stack() {
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
                    let mut ty = self.codegen_c_item_ty(&f.ty.kind);

                    if codegen::is_raw_ptr_field(f, adjust) {
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
        if s.is_all_in_stack() {
            // stream.push_str(&format! {r#"
            //     impl ::fcplug::ctypes::ConvReprC for {name} {{
            //         type ReprC = C_{name};
            //         #[inline(always)]
            //         fn into_repr_c(self) -> Self::ReprC {{
            //             self
            //         }}
            //         #[inline(always)]
            //         fn from_repr_c(c: Self::ReprC) -> Self {{
            //             c
            //         }}
            //     }}
            // "#});
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
                    } else if f.is_in_stack() {
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
                    if f.is_optional() && !adjust.map_or(false, |adjust| adjust.boxed()) {
                        format!(r###"let {name} = if {name}.is_null() {{
                            ::std::option::Option::None
                        }} else {{
                            ::std::option::Option::Some(::fcplug::ctypes::ConvReprC::from_repr_c(unsafe {{ *::std::boxed::Box::from_raw({name}) }}))
                        }};"###)
                    } else if f.is_in_stack() {
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
    fn codegen_rustffi_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        let name = self.context.rust_name(def_id);
        let name_lower = name.to_lowercase();
        let ust = if let Some(ust) = self.config.impl_rustffi_for_unit_struct {
            ust.to_string()
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
            let args = method.args
                .iter()
                .map(|arg| format!("{}: {}", (&**arg.name).snake_ident(), self.codegen_c_item_ty(&arg.ty.kind)))
                .collect::<Vec<String>>()
                .join(", ");
            let args_to_rust = method.args
                .iter()
                .map(|arg| format!("&<{} as ::fcplug::ctypes::ConvReprC>::from_repr_c({})",
                                   self.codegen_item_ty(&arg.ty.kind), (&**arg.name).snake_ident()))
                .collect::<Vec<String>>()
                .join(", ");
            let ret_rs_ty = self.codegen_item_ty(&method.ret.kind);
            let ret_c_ty = self.codegen_c_item_ty(&method.ret.kind);

            if method.ret.is_in_stack() {
                format!(r###"#[no_mangle]
                #[inline]
                pub extern "C" fn {name_lower}_{fn_name}({args}) -> {ret_c_ty} {{
                    <{ust} as {name}>::{fn_name}({args_to_rust})
                }}
                "###)
            } else {
                format!(r###"#[no_mangle]
                #[inline]
                pub extern "C" fn {name_lower}_{fn_name}({args}) -> *mut {ret_c_ty} {{
                    ::std::boxed::Box::into_raw(::std::boxed::Box::new(<{ret_rs_ty} as ::fcplug::ctypes::ConvReprC>::into_repr_c(<{ust} as {name}>::{fn_name}({args_to_rust}))))
                }}
                #[no_mangle]
                #[inline]
                pub extern "C" fn {name_lower}_{fn_name}_free_ret(ret_ptr: *mut {ret_c_ty}) {{
                    if !ret_ptr.is_null() {{
                        let _ = <{ret_rs_ty} as ::fcplug::ctypes::ConvReprC>::from_repr_c(unsafe {{ *::std::boxed::Box::from_raw(ret_ptr) }});
                    }}
                }}
                "###)
            }
        })
            .collect::<Vec<String>>()
            .join("\n"));
    }
    fn codegen_goffi_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        self.codegen_goffi_functions(def_id, stream, s);
        self.codegen_goffi_methods(def_id, stream, s);
    }
    fn codegen_goffi_functions(&self, def_id: DefId, stream: &mut String, s: &Service) {
        let name = self.context.rust_name(def_id);
        let name_lower = name.to_lowercase();

        let ffi_fns = s.methods.iter().map(|method| {
            let fn_name = (&**method.name).fn_ident();
            let args = method.args
                .iter()
                .map(|arg| format!(
                    "{}: {}{}",
                    (&**arg.name).snake_ident(),
                    if arg.ty.is_in_stack() { "" } else { "*mut " },
                    self.codegen_c_item_ty(&arg.ty.kind)
                ))
                .collect::<Vec<String>>()
                .join(", ");

            let ret = self.codegen_c_item_ty(&method.ret.kind);
            if method.ret.is_in_stack() {
                format!("fn {name_lower}_{fn_name}({args}) -> {ret};")
            } else {
                format!(r###"fn {name_lower}_{fn_name}({args}) -> *mut {ret};
            fn {name_lower}_{fn_name}_free_ret(ret_ptr: usize);
            "###)
            }
        })
            .collect::<Vec<String>>()
            .join("\n");
        stream.push_str(&format!(r###"extern "C" {{
            {ffi_fns}
        }}
        "###));
    }
    fn codegen_goffi_methods(&self, def_id: DefId, stream: &mut String, s: &Service) {
        let name = self.context.rust_name(def_id);
        let name_lower = name.to_lowercase();
        let impl_methods = s.methods.iter().map(|method| {
            let name = (&**method.name).fn_ident();
            let args = self.codegen_method_args(def_id, method);
            let ret = self.codegen_method_ret(def_id, method);
            let args_into_repr_c = method.args
                .iter()
                .filter(|arg| !arg.ty.is_in_stack())
                .map(|arg| {
                    let ident = (&**arg.name).snake_ident();
                    format!("let {ident} = ::std::boxed::Box::into_raw(::std::boxed::Box::new(::fcplug::ctypes::ConvReprC::into_repr_c({ident})));")
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
                        format!("let _ = <{ty} as ::fcplug::ctypes::ConvReprC>::from_repr_c(*::std::boxed::Box::from_raw({ident}));")
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                format!(r###"unsafe fn {name}({args}) -> {ret} {{
                    {args_into_repr_c}
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
                    {args_into_repr_c}
                    let c_ret__ = {name_lower}_{name}({c_args});
                    let ret__ = <{raw_ret_ty} as ::fcplug::ctypes::ConvReprC>::from_repr_c(*::std::boxed::Box::from_raw(c_ret__));
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