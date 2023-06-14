#![feature(try_trait_v2)]

pub use fcplug_macros::{ffi_fb_method, ffi_pb_method, ffi_raw_method};

pub use crate::abi::{ABIMessage, ABIRequest, ABIResponse, ABIResult, Buffer, callback, FFIResult, ResultCode};

pub mod protobuf;
pub mod flatbuf;
mod abi;
mod log;
mod basic;


#[test]
fn it_works() {
    assert_eq!(4, 2 + 2);
}
