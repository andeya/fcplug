#![feature(try_trait_v2)]

pub use fcplug_macros::{ffi_pb_method, ffi_fb_method};
pub use crate::abi::{Buffer, ABIMessage, ABIResponse, ABIRequest, ABIResult, ResultCode, FFIResult};

pub mod protobuf;
pub mod flatbuf;
mod abi;
mod log;


#[test]
fn it_works() {
    assert_eq!(4, 2 + 2);
}
