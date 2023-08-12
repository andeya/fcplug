#![allow(dead_code)]

use std::cell::RefCell;
use std::sync::Arc;

use crate::config::WorkConfig;
use crate::make_backend::Cx;

#[derive(Clone)]
pub(crate) struct GoCodegenBackendNoCodec {
    pub(crate) config: WorkConfig,
    pub(crate) context: Cx,
    pub(crate) go_pkg_code: Arc<RefCell<String>>,
    pub(crate) go_main_code: Arc<RefCell<String>>,
}
