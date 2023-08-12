#![allow(unused_variables)]

pub use echo_pb_gen::*;
use fcplug::{GoFfiResult, TryIntoTBytes};
use fcplug::protobuf::PbMessage;

mod echo_pb_gen;

impl RustFfi for FfiImpl {
    fn echo_rs(mut req: ::fcplug::RustFfiArg<Ping>) -> ::fcplug::ABIResult<::fcplug::TBytes<Pong>> {
        let _req = req.try_to_object::<PbMessage<_>>();
        #[cfg(debug_assertions)]
        println!("rust receive req: {:?}", _req);
        Pong {
            msg: "this is pong from rust".to_string(),
        }
            .try_into_tbytes::<PbMessage<_>>()
    }
}

impl GoFfi for FfiImpl {
    #[allow(unused_mut)]
    unsafe fn echo_go_set_result(mut go_ret: ::fcplug::RustFfiArg<Pong>) -> ::fcplug::GoFfiResult {
        #[cfg(debug_assertions)]
        return GoFfiResult::from_ok(go_ret.try_to_object::<PbMessage<_>>()?);
        #[cfg(not(debug_assertions))]
        return GoFfiResult::from_ok(go_ret.bytes().to_owned());
    }
}
