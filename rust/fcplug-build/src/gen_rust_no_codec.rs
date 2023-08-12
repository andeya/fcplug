#![allow(dead_code)]

use std::cell::RefCell;
use std::sync::Arc;

use crate::config::WorkConfig;
use crate::make_backend::Cx;

#[derive(Clone)]
pub(crate) struct RustCodegenBackendNoCodec {
    pub(crate) config: WorkConfig,
    pub(crate) context: Cx,
    pub(crate) rust_impl_rustffi_code: Arc<RefCell<String>>,
    pub(crate) rust_impl_goffi_code: Arc<RefCell<String>>,
}
