#![feature(test)]
extern crate test;

use fcplug::{ABIResult, GoFfiResult, RustFfiArg, TBytes, TryIntoTBytes};
use fcplug::protobuf::PbMessage;

use crate::echo_gen::{GoFfi, Ping, Pong, RustFfi};

mod echo_gen;


struct ImplFfi;

impl RustFfi for ImplFfi {
    fn echo_rs(mut req: RustFfiArg<Ping>) -> ABIResult<TBytes<Pong>> {
        let _req = req.try_to_object::<PbMessage<_>>();
        #[cfg(debug_assertions)]
        println!("rust receive req: {:?}", _req);
        Pong { msg: "this is pong from rust".to_string() }.try_into_tbytes::<PbMessage<_>>()
    }
}

impl GoFfi for ImplFfi {
    #[allow(unused_mut)]
    unsafe fn echo_go_set_result(mut go_ret: RustFfiArg<Pong>) -> GoFfiResult {
        #[cfg(debug_assertions)]
        return GoFfiResult::from_ok(go_ret.try_to_object::<PbMessage<_>>()?);
        #[cfg(not(debug_assertions))]
        return GoFfiResult::from_ok(go_ret.bytes().to_owned());
    }
}


#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_call_echo_go() {
        let req = Ping {
            msg: "this is ping from rust".to_string(),
        }.try_into_tbytes::<PbMessage<_>>().unwrap();
        let pong = unsafe { ImplFfi::echo_go::<Pong>(req) };
        println!("{:?}", pong);
    }

    #[bench]
    fn bench_call_echo_go(b: &mut Bencher) {
        let req = Ping {
            msg: "this is ping from rust".to_string(),
        }.try_into_tbytes::<PbMessage<_>>().unwrap();
        b.iter(|| {
            let pong = unsafe { ImplFfi::echo_go::<Vec<u8>>(req.clone()) };
            let _ = test::black_box(pong);
        });
    }
}
