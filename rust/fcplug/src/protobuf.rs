use std::fmt::Debug;

pub use pilota::prost::Message;

use crate::{ABIResult, FromMessage, RC_DECODE, RC_ENCODE, ResultMsg, TryFromBytes, TryIntoBytes};

#[derive(Debug)]
pub struct PbMessage<T: Message>(pub T);

impl<T: Message + Default> FromMessage<PbMessage<T>> for T {
    fn from_message(value: PbMessage<T>) -> Self {
        value.0
    }
}

impl<T: Message + Default> TryFromBytes<'_> for PbMessage<T> {
    fn try_from_bytes(buf: &mut [u8]) -> ABIResult<Self> where Self: Sized {
        Ok(T::decode(buf as &[u8]).map(PbMessage).map_err(decode_map_err)?)
    }
}

impl<T: Message> TryIntoBytes for PbMessage<T> {
    fn try_into_bytes(self) -> ABIResult<Vec<u8>> {
        Ok(self.0.encode_to_vec())
    }
}

fn decode_map_err(e: pilota::prost::DecodeError) -> ResultMsg {
    ResultMsg { code: RC_DECODE, msg: e.to_string() }
}

fn encode_map_err(e: pilota::prost::EncodeError) -> ResultMsg {
    ResultMsg { code: RC_ENCODE, msg: e.to_string() }
}
