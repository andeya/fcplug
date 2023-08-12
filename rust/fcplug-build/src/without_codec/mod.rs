use pilota_build::{CodegenBackend, Context, DefId};
use pilota_build::rir::{Enum, Message, Method, NewType, Service};

use crate::generator::{GeneraterBackend, Generator};

mod gen_go_no_codec;
mod gen_rust_no_codec;

impl Generator {
    pub(crate) fn _gen_code(self) {
        todo!()
    }
}

impl CodegenBackend for GeneraterBackend {
    fn cx(&self) -> &Context {
        self.context.0.as_ref()
    }

    fn codegen_struct_impl(&self, _def_id: DefId, _stream: &mut String, _s: &Message) {
        todo!()
    }

    fn codegen_service_impl(&self, _def_id: DefId, _stream: &mut String, _s: &Service) {
        todo!()
    }

    fn codegen_service_method(&self, _service_def_id: DefId, _method: &Method) -> String {
        todo!()
    }

    fn codegen_enum_impl(&self, _def_id: DefId, _stream: &mut String, _e: &Enum) {
        todo!()
    }

    fn codegen_newtype_impl(&self, _def_id: DefId, _stream: &mut String, _t: &NewType) {
        todo!()
    }
}
