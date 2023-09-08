#![feature(test)]

extern crate test;

pub mod echo_pb_ffi;


#[cfg(test)]
mod tests {
    use test::Bencher;

    use fcplug::protobuf::PbMessage;
    use fcplug::TryIntoTBytes;

    use crate::echo_pb_ffi::{FfiImpl, GoFfiCall, Ping, Pong};

    #[test]
    fn test_call_echo_go() {
        let pong = unsafe {
            FfiImpl::echo_go::<Pong>(Ping {
                msg: "this is ping from rust".to_string(),
            }.try_into_tbytes::<PbMessage<_>>().unwrap())
        };
        println!("{:?}", pong);
    }

    #[bench]
    fn bench_call_echo_go(b: &mut Bencher) {
        let req = Ping {
            msg: "this is ping from rust".to_string(),
        }
            .try_into_tbytes::<PbMessage<_>>()
            .unwrap();
        b.iter(|| {
            let pong = unsafe { FfiImpl::echo_go::<Vec<u8>>(req.clone()) };
            let _ = test::black_box(pong);
        });
    }
}
