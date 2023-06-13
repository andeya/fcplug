use std::fmt::{Debug};
use protobuf::{Message, ProtobufError};
use crate::abi::{ABIMessage, ABIRequest, ABIResponse, LeakBuffer};
use crate::{ABIResult, Buffer, FFIResult, ResultCode};

pub fn callback<A, R, F: Fn(A) -> ABIResult<R>>(_ffi_method_name: &str, f: F, args: &Buffer) -> FFIResult
    where A: Message,
          R: Message {
    let args_obj: PbMessage<A> = match ABIRequest::try_from_bytes(args.read().unwrap_or_default()) {
        Ok(args_obj) => args_obj,
        Err(err) => return FFIResult::err(ResultCode::Decode, Some(err)),
    };

    #[cfg(debug_assertions)]
        let txt = format!("invoking method={}, args_bytes={:?}, args_obj={:?}", _ffi_method_name, args.read(), args_obj);

    let res_obj: ABIResult<PbMessage<R>> = f(args_obj.0).map(PbMessage);

    #[cfg(debug_assertions)]
        let txt = format!("{}, abi_result={:?}", txt, res_obj);

    let res = FFIResult::from(res_obj);

    #[cfg(debug_assertions)]
    println!("{}, ffi_result={:?}", txt, res);

    res
}

#[derive(Debug)]
pub struct PbMessage<T: Message>(T);

impl<T: Message> PbMessage<T> {
    pub fn new(v: T) -> Self {
        Self(v)
    }
}

impl<'a, T: Message> ABIRequest<'a> for PbMessage<T> {
    type DecodeError = ProtobufError;

    fn try_from_bytes(buf: &'a [u8]) -> Result<Self, Self::DecodeError> where Self: Sized {
        T::parse_from_bytes(buf).map(PbMessage)
    }
}

impl<T: Message> ABIResponse for PbMessage<T> {
    type EncodeError = ProtobufError;

    fn try_into_buffer(self) -> Result<LeakBuffer, Self::EncodeError> {
        self.0.write_to_bytes().map(|v| LeakBuffer::from_vec(v))
    }
}

impl<'a, T: Message> ABIMessage<'a> for PbMessage<T> {}
