use echo_pb::echo_pb_ffi::{FfiImpl, GoFfiCall, Ping, Pong};
use fcplug::protobuf::PbMessage;
use fcplug::TryIntoTBytes;

fn main() {
    for i in 0..1000000 {
        println!("i={i}");
        let pong = unsafe {
            FfiImpl::echo_go::<Pong>(
                Ping {
                    msg: "this is ping from rust".to_string(),
                }
                .try_into_tbytes::<PbMessage<_>>()
                .unwrap(),
            )
        };
        let pong = pong.unwrap();
        if pong.msg != "this is pong from go" {
            panic!("pong==============:{pong:?}")
        }
    }
}
