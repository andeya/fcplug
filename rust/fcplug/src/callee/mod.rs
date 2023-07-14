pub use abi::{ABIMessage, ABIRequest, ABIResponse, ABIResult, Buffer, callback, FFIResult, ResultCode};
pub use fcplug_macros::{ffi_fb_callee, ffi_pb_callee, ffi_raw_callee};

pub mod protobuf;
pub mod flatbuf;
mod abi;
mod log;
mod basic;


#[test]
fn it_works() {
    assert_eq!(4, 2 + 2);
}
