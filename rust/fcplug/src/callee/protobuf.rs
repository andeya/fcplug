use std::fmt::Debug;

use protobuf::{Error as ProtobufError, MessageFull};

use super::{abi, ABIResult, Buffer, FFIResult};
use super::abi::{ABIMessage, ABIRequest, ABIResponse, LeakBuffer, OriginType};

#[inline]
pub fn callback<'a, A, R, F>(_ffi_method_name: &'static str, f: F, args: &'a mut Buffer) -> FFIResult
    where A: MessageFull,
          R: MessageFull,
          F: Fn(A) -> ABIResult<R> {
    abi::callback::<'a, PbMessage<A>, PbMessage<R>, _>(_ffi_method_name, |req| {
        f(req.0).map(PbMessage)
    }, args)
}

#[derive(Debug)]
pub struct PbMessage<T: MessageFull>(T);

impl<T: MessageFull> PbMessage<T> {
    pub fn new(v: T) -> Self {
        Self(v)
    }
}

impl<'a, T: MessageFull> ABIRequest<'a> for PbMessage<T> {
    type DecodeError = ProtobufError;

    fn try_from_bytes(buf: &'a mut [u8]) -> Result<Self, Self::DecodeError> where Self: Sized {
        T::parse_from_bytes(buf).map(PbMessage)
    }
}

impl<T: MessageFull> ABIResponse for PbMessage<T> {
    type EncodeError = ProtobufError;
    const ORIGIN_TYPE_FOR_FREE: OriginType = OriginType::Vec;

    fn try_into_buffer(self) -> Result<LeakBuffer, Self::EncodeError> {
        self.0.write_to_bytes().map(|v| LeakBuffer::from_vec(v))
    }
}

impl<'a, T: MessageFull> ABIMessage<'a> for PbMessage<T> {}
