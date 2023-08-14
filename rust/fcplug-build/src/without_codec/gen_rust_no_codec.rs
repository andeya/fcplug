#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::Arc;

use pilota_build::DefId;
use pilota_build::rir::{Method, Service};

use crate::generator::{RustCodegenBackend, RustGeneratorBackend};

impl RustCodegenBackend for RustGeneratorBackend {
    fn codegen_rustffi_trait_method(&self, service_def_id: DefId, method: &Arc<Method>) -> Option<String> {
        // TODO
        None
    }


    fn codegen_rustffi_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        // TODO
    }

    fn codegen_goffi_trait_method(&self, service_def_id: DefId, method: &Arc<Method>) -> Option<String> {
        // TODO
        None
    }

    fn codegen_goffi_call_trait_method(&self, service_def_id: DefId, method: &Arc<Method>) -> Option<String> {
        // TODO
        None
    }


    fn codegen_goffi_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        // TODO
    }
}
