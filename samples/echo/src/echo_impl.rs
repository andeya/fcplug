use fcplug::protobuf::PbMessage;
use fcplug::{ABIResult, GoFfiResult, RustFfiArg, TBytes, TryIntoTBytes};

use crate::echo_gen::{Ping, Pong};

impl crate::echo_gen::RustFfi for crate::echo_gen::ImplFfi {
    fn echo_rs(mut req: RustFfiArg<Ping>) -> ABIResult<TBytes<Pong>> {
        let _req = req.try_to_object::<PbMessage<_>>();
        #[cfg(debug_assertions)]
        println!("rust receive req: {:?}", _req);
        Pong {
            msg: "this is pong from rust".to_string(),
        }
        .try_into_tbytes::<PbMessage<_>>()
    }
}

impl crate::echo_gen::GoFfi for crate::echo_gen::ImplFfi {
    #[allow(unused_mut)]
    unsafe fn echo_go_set_result(mut go_ret: RustFfiArg<Pong>) -> GoFfiResult {
        #[cfg(debug_assertions)]
        return GoFfiResult::from_ok(go_ret.try_to_object::<PbMessage<_>>()?);
        #[cfg(not(debug_assertions))]
        return GoFfiResult::from_ok(go_ret.bytes().to_owned());
    }
}
