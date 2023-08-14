#![allow(unused_variables)]

pub use echo_gen::*;

mod echo_gen;

impl GoFfi for FfiImpl {}

impl RustFfi for FfiImpl {}
