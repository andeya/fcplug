#![allow(dead_code)]
#![allow(unused_variables)]

use pilota_build::DefId;
use pilota_build::rir::Service;

use crate::generator::{RustCodegenBackend, RustGeneratorBackend};

impl RustCodegenBackend for RustGeneratorBackend {
    fn codegen_rustffi_trait_methods(&self, def_id: DefId, s: &Service) -> Vec<String> {
        // TODO:
        vec![]
    }

    fn codegen_rustffi_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        // TODO
    }

    fn codegen_goffi_trait_methods(&self, def_id: DefId, s: &Service) -> Vec<String> {
        // TODO:
        vec![]
    }

    fn codegen_goffi_call_trait_methods(&self, def_id: DefId, s: &Service) -> Vec<String> {
        // TODO:
        vec![]
    }

    fn codegen_goffi_service_impl(&self, def_id: DefId, stream: &mut String, s: &Service) {
        // TODO
    }
}
