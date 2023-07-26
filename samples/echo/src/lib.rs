#![feature(test)]
extern crate test;

mod echo_gen;
mod echo_impl;

#[cfg(test)]
mod tests {
    use test::Bencher;

    use fcplug::protobuf::PbMessage;
    use fcplug::TryIntoTBytes;

    use crate::echo_gen::{GoFfi, ImplFfi, Ping, Pong};

    use super::*;

    #[test]
    fn test_call_echo_go() {
        let req = Ping {
            msg: "this is ping from rust".to_string(),
        }
        .try_into_tbytes::<PbMessage<_>>()
        .unwrap();
        let pong = unsafe { ImplFfi::echo_go::<Pong>(req) };
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
            let pong = unsafe { ImplFfi::echo_go::<Vec<u8>>(req.clone()) };
            let _ = test::black_box(pong);
        });
    }
}
