use fcplug::callee::ABIResult;
use fcplug::callee::flatbuf::{FbRequest, FbResponseWriter};
use goffi_gen::idl::Echo;
use goffi_gen::idl_generated::{EchoRequest, EchoResponse, EchoResponseArgs};

#[fcplug::callee::ffi_raw_callee]
fn echo(args: &str) -> ABIResult<String> {
    Ok("input is: ".to_string() + args)
}


#[fcplug::callee::ffi_pb_callee]
fn echo(args: Echo) -> ABIResult<Echo> {
    let mut r = Echo::new();
    r.set_msg("input is: ".to_string() + args.get_msg());
    Ok(r)
}

#[fcplug::callee::ffi_fb_callee]
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

#[cfg(test)]
mod tests {
    use goffi_gen::go_ffi::{Buffer, helloString};

    use crate::ffi_raw_echo;

    #[test]
    fn test_echo() {
        use fcplug::callee::*;
        let req = "andeya".to_string().try_into_buffer().unwrap();
        let mut r: FFIResult = ffi_raw_echo(req.buffer());
        unsafe { req.mem_free() };
        println!("FFIResult={:?}", r);

        println!("ABIResult={:?}", if let ResultCode::NoError = r.code {
            <&str>::try_from_bytes(r.data.read_mut().unwrap_or_default()).map_err(|_e| ResultCode::Decode)
        } else {
            Err::<&str, ResultCode>(r.code)
        });
    }

    #[test]
    #[inline]
    fn test_call_go() {
        let mut s = String::from("hello world");
        unsafe {
            helloString(Buffer {
                ptr: s.as_mut_ptr(),
                len: s.len(),
                cap: s.len(),
            })
        }
    }
}
