use fcplug::{ABIResult, GoFfiResult, RustFfiArg, TBytes, TryIntoTBytes};
use fcplug::protobuf::PbMessage;

use crate::echo_gen::{GoFfi, Ping, Pong, RustFfi};

mod echo_gen;


struct ImplFfi;

impl RustFfi for ImplFfi {
    fn echo_rs(mut req: RustFfiArg<Ping>) -> ABIResult<TBytes<Pong>> {
        let req = req.try_to_object::<PbMessage<_>>();
        println!("rust receive req: {:?}", req);
        Pong { msg: "this is pong from rust".to_string() }.try_into_tbytes::<PbMessage<_>>()
    }
}

impl GoFfi for ImplFfi {
    unsafe fn echo_go_set_result(mut go_ret: RustFfiArg<Pong>) -> GoFfiResult {
        GoFfiResult::from_ok(go_ret.try_to_object::<PbMessage<_>>()?)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_echo_go() {
        let req = Ping {
            msg: "this is ping from rust".to_string(),
        }.try_into_tbytes::<PbMessage<_>>().unwrap();
        let pong = unsafe { ImplFfi::echo_go::<Pong>(req) };
        println!("{:?}", pong);
    }
}
