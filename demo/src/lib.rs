use fcplug_rclib::{ABIResult};
use fcplug_rclib::flatbuf::{FbRequest, FbResponseWriter};
use crate::idl::Echo;
use crate::idl_generated::{EchoRequest, EchoResponse, EchoResponseArgs};

mod idl;
#[allow(unused_imports, dead_code)]
mod idl_generated;

#[fcplug_rclib::ffi_pb_method]
fn echo(args: Echo) -> ABIResult<Echo> {
    let mut r = Echo::new();
    r.set_msg("input is: ".to_string() + args.get_msg());
    Ok(r)
}

#[fcplug_rclib::ffi_fb_method]
fn echo<'a>(
    req: FbRequest<'a, EchoRequest<'a>>,
) -> (EchoResponseArgs<'a>, FbResponseWriter<EchoResponse<'a>>) {
    let data = req.data().unwrap();
    let mut w = req.new_response_writer();
    (
        EchoResponseArgs {
            data: Some(
                w.create_string(("input is: ".to_string() + data).as_str()),
            ),
        },
        w,
    )
}

#[inline]
#[no_mangle]
pub extern "C" fn ffi_fb_echo_raw(req: ::fcplug_rclib::Buffer) -> ::fcplug_rclib::FFIResult {
    fn echo<'a>(
        req: FbRequest<'a, EchoRequest<'a>>,
    ) -> (EchoResponseArgs<'a>, FbResponseWriter<EchoResponse<'a>>) {
        let data = req.data().unwrap();
        let mut w = req.new_response_writer();
        (
            EchoResponseArgs {
                data: Some(
                    w.create_string(("input is: ".to_string() + data).as_str()),
                ),
            },
            w,
        )
    }
    let (_resp_, mut _w_) = echo(::fcplug_rclib::flatbuf::FbRequest::try_from_buffer(&req)?);
    let _resp_ = EchoResponse::create(&mut _w_, &_resp_);
    _w_.finish_minimal(_resp_);
    ::fcplug_rclib::FFIResult::ok(::fcplug_rclib::ABIResponse::try_into_buffer(_w_).unwrap())
}


#[test]
fn test_echo() {
    use fcplug_rclib::*;
    let mut req = Echo::new();
    req.set_msg("andeya".to_string());
    let buf = protobuf::PbMessage::new(req).try_into_buffer().expect("xxxxxxxxxxxxx");
    let r: FFIResult = ffi_pb_echo(buf.buffer());
    unsafe { buf.mem_free() };
    println!("FFIResult={:?}", r);

    println!("ABIResult={:?}", if let ResultCode::NoError = r.code {
        protobuf::PbMessage::try_from_bytes(r.data.read().unwrap_or_default()).map_err(|_e| ResultCode::Decode)
    } else {
        Err::<protobuf::PbMessage<Echo>, ResultCode>(r.code)
    });
}
