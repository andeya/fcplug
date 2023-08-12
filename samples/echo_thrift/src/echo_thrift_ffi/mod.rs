#![allow(unused_variables)]

pub use echo_thrift_gen::*;

mod echo_thrift_gen;

impl RustFfi for FfiImpl {
    fn echo_rs(req: ::fcplug::RustFfiArg<Ping>) -> ::fcplug::ABIResult<::fcplug::TBytes<Pong>> {
        todo!()
    }
}

impl GoFfi for FfiImpl {
    unsafe fn echo_go_set_result(go_ret: ::fcplug::RustFfiArg<Pong>) -> ::fcplug::GoFfiResult {
        todo!()
    }
}
