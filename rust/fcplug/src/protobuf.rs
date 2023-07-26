use std::fmt::Debug;

pub use pilota::prost::Message;

use crate::{
    ABIResult, FromMessage, IntoMessage, ResultMsg, TryFromBytes, TryIntoBytes, RC_DECODE,
    RC_ENCODE,
};

#[derive(Debug)]
pub struct PbMessage<T: Message>(pub T);

impl<T: Message + Default> FromMessage<PbMessage<T>> for T {
    #[inline]
    fn from_message(value: PbMessage<T>) -> Self {
        value.0
    }
}

impl<T: Message + Default> IntoMessage<PbMessage<T>> for T {
    #[inline]
    fn into_message(self) -> PbMessage<T> {
        PbMessage(self)
    }
}

impl<T: Message + Default> TryFromBytes<'_> for PbMessage<T> {
    #[inline]
    fn try_from_bytes(buf: &mut [u8]) -> ABIResult<Self>
    where
        Self: Sized,
    {
        Ok(T::decode(buf as &[u8])
            .map(PbMessage)
            .map_err(decode_map_err)?)
    }
}

impl<T: Message> TryIntoBytes for PbMessage<T> {
    #[inline]
    fn try_into_bytes(self) -> ABIResult<Vec<u8>> {
        Ok(self.0.encode_to_vec())
    }
}

#[inline]
fn decode_map_err(e: pilota::prost::DecodeError) -> ResultMsg {
    ResultMsg {
        code: RC_DECODE,
        msg: e.to_string(),
    }
}

#[inline]
#[allow(dead_code)]
fn encode_map_err(e: pilota::prost::EncodeError) -> ResultMsg {
    ResultMsg {
        code: RC_ENCODE,
        msg: e.to_string(),
    }
}
