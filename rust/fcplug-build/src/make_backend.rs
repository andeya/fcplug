use std::ops::Deref;
use std::sync::Arc;

use pilota_build::{
    CodegenBackend, Context, DefId, MakeBackend, ProtobufBackend, rir::Enum, rir::Message,
    rir::Method, rir::NewType, rir::Service, ThriftBackend,
};
use pilota_build::db::RirDatabase;
use pilota_build::rir::{Arg, Item};
use pilota_build::ty::TyKind;

use crate::config::{IdlType, WorkConfig};
use crate::gen_go::GoCodegenBackend;
use crate::gen_go_no_codec::GoCodegenBackendNoCodec;
use crate::gen_rust::RustCodegenBackend;
use crate::gen_rust_no_codec::RustCodegenBackendNoCodec;
use crate::Generator;

impl MakeBackend for Generator {
    type Target = GeneraterBackend;

    fn make_backend(self, context: Context) -> Self::Target {
        let thrift = ThriftBackend::new(context.clone());
        let protobuf = ProtobufBackend::new(context.clone());
        let context = Arc::new(context);
        GeneraterBackend {
            thrift,
            protobuf,
            rust: RustCodegenBackend {
                config: self.config.clone(),
                context: Cx(context.clone()),
                rust_impl_rustffi_code: self.rust_impl_rustffi_code.clone(),
                rust_impl_goffi_code: self.rust_impl_goffi_code.clone(),
            },
            go: GoCodegenBackend {
                config: self.config.clone(),
                context: Cx(context.clone()),
                go_pkg_code: self.go_pkg_code.clone(),
                go_main_code: self.go_main_code.clone(),
            },
            rust_no_codec: RustCodegenBackendNoCodec {
                config: self.config.clone(),
                context: Cx(context.clone()),
                rust_impl_rustffi_code: self.rust_impl_rustffi_code,
                rust_impl_goffi_code: self.rust_impl_goffi_code,
            },
            go_no_codec: GoCodegenBackendNoCodec {
                config: self.config.clone(),
                context: Cx(context.clone()),
                go_pkg_code: self.go_pkg_code.clone(),
                go_main_code: self.go_main_code.clone(),
            },
            context: Cx(context),
            config: self.config,
        }
    }
}


#[allow(dead_code)]
#[derive(Clone)]
pub struct GeneraterBackend {
    config: WorkConfig,
    context: Cx,
    protobuf: ProtobufBackend,
    thrift: ThriftBackend,
    rust: RustCodegenBackend,
    go: GoCodegenBackend,
    rust_no_codec: RustCodegenBackendNoCodec,
    go_no_codec: GoCodegenBackendNoCodec,
}

unsafe impl Send for GeneraterBackend {}

#[derive(Clone)]
pub(crate) struct Cx(Arc<Context>);

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

impl Deref for Cx {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl GeneraterBackend {
    fn fix_empty_params(&self, method: &Method) -> Method {
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

impl CodegenBackend for GeneraterBackend {
    fn cx(&self) -> &Context {
        self.context.0.as_ref()
    }
    fn codegen_struct_impl(&self, def_id: DefId, stream: &mut String, s: &Message) {
        match self.config.idl_type() {
            IdlType::Proto | IdlType::ProtoNoCodec => self.protobuf.codegen_struct_impl(def_id, stream, s),
            IdlType::Thrift | IdlType::ThriftNoCodec => self.thrift.codegen_struct_impl(def_id, stream, s),
        }
    }
    fn codegen_service_impl(&self, service_def_id: DefId, stream: &mut String, s: &Service) {
        let mut s = s.clone();
        s.methods = s
            .methods
            .iter()
            .map(|method| Arc::new(self.fix_empty_params(method)))
            .collect::<Vec<Arc<Method>>>();
        self.protobuf
            .codegen_service_impl(service_def_id, stream, &s);
        self.rust.codegen_service_impl(service_def_id, stream, &s);
        self.go.codegen(service_def_id, &s)
    }
    fn codegen_service_method(&self, service_def_id: DefId, method: &Method) -> String {
        let method = self.fix_empty_params(method);
        self.protobuf
            .codegen_service_method(service_def_id, &method);
        String::new()
    }
    fn codegen_enum_impl(&self, def_id: DefId, stream: &mut String, e: &Enum) {
        self.protobuf.codegen_enum_impl(def_id, stream, e);
    }
    fn codegen_newtype_impl(&self, def_id: DefId, stream: &mut String, t: &NewType) {
        self.protobuf.codegen_newtype_impl(def_id, stream, t);
    }
}
