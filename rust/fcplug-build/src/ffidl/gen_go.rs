use std::sync::Arc;

use pilota_build::{DefId, rir::Message, rir::Service};
use pilota_build::Context;

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
    pub(crate) fn codegen_struct_impl(&self, def_id: DefId, stream: &mut String, s: &Message) {}
    pub(crate) fn codegen_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {}
}
