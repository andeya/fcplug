use protobuf::MessageFull;

use crate::{ABIResult, Buffer, FFIResult, ResultCode, TryFromBytes, TryIntoBytes};
use crate::protobuf::PbMessage;

#[test]
fn it_works() {
    assert_eq!(4, 2 + 2);
}


pub fn callback<'a, Request, Response, F>(_ffi_method_name: &'static str, method_fn: F, args: &'a mut Buffer) -> FFIResult
    where Request: TryFromBytes<'a>,
          Response: TryIntoBytes,
          F: Fn(Request) -> ABIResult<Response> {
    let args_obj = match Request::try_from_bytes(args.read_mut().unwrap_or_default()) {
        Ok(args_obj) => args_obj,
        Err(err) => return FFIResult::err(ResultCode::Decode, Some(err)),
    };

    #[cfg(debug_assertions)]
        let txt = format!("invoking method={}, args_obj={:?}", _ffi_method_name, args_obj);

    let res_obj: ABIResult<Response> = method_fn(args_obj);

    #[cfg(debug_assertions)]
        let txt = format!("{}, abi_result={:?}", txt, res_obj);

    let res = FFIResult::from(res_obj);

    #[cfg(debug_assertions)]
    println!("{}, ffi_result={:?}", txt, res);

    res
}

#[inline]
pub fn callback_pb<'a, A, R, F>(_ffi_method_name: &'static str, f: F, args: &'a mut Buffer) -> FFIResult
    where A: MessageFull,
          R: MessageFull,
          F: Fn(A) -> ABIResult<R> {
    callback::<'a, PbMessage<A>, PbMessage<R>, _>(_ffi_method_name, |req| {
        f(req.0).map(PbMessage)
    }, args)
}
