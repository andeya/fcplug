#![allow(dead_code)]
#![allow(unused_variables)]

use pilota_build::DefId;
use pilota_build::rir::{Message, Service};

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

    fn codegen_rustffi_iface_methods(&self, def_id: DefId, s: &Service) -> Vec<(String, String)> {
        // TODO:
        vec![]
    }

    #[allow(unused_variables)]
    fn codegen_rustffi_service_impl(&self, service_def_id: DefId, s: &Service) -> String {
        // TODO:
        String::new()
    }

    fn codegen_goffi_iface_methods(&self, def_id: DefId, s: &Service) -> Vec<String> {
        // TODO:
        vec![]
    }

    #[allow(unused_variables)]
    fn codegen_goffi_service_impl(&self, service_def_id: DefId, s: &Service) -> String {
        // TODO:
        String::new()
    }
}
