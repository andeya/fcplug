#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::Arc;

use pilota_build::DefId;
use pilota_build::rir::{Message, Method, Service};

use crate::generator::{GoCodegenBackend, GoGeneratorBackend};

impl GoCodegenBackend for GoGeneratorBackend {
    fn codegen_struct_type(&self, def_id: DefId, s: &Message) -> String {
        let files = s
            .fields
            .iter()
            .map(|field| {
                let field_name = self.field_name(field);
                let field_type = self.field_type(field);
                let field_tag = self.field_tag(field);
                format!("{field_name}    {field_type}    {field_tag}")
            })
            .collect::<Vec<String>>()
            .join("    \n");
        let struct_name = self.struct_name(def_id);
        format!(r###"
type {struct_name} struct {{
    {files}
}}
        "###)
    }

    fn codegen_rustffi_iface_method(&self, service_def_id: DefId, method: &Arc<Method>) -> Option<(String, String)> {
        // TODO
        None
    }

    fn codegen_rustffi_service_impl(&self, service_def_id: DefId, s: &Service) -> String {
        // TODO
        String::new()
    }

    fn codegen_goffi_iface_method(&self, service_def_id: DefId, method: &Arc<Method>) -> Option<String> {
        // TODO
        None
    }

    fn codegen_goffi_service_impl(&self, service_def_id: DefId, s: &Service) -> String {
        // TODO
        String::new()
    }
}
