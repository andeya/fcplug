use crate::generator::{Generator, MidOutput};

mod gen_go_no_codec;
mod gen_rust_no_codec;

impl Generator {
    pub(crate) fn _gen_code(self) -> MidOutput {
        MidOutput {
            rust_clib_includes: "".to_string(),
            mod_requires: vec![],
            imports: vec![],
        }
    }
}
