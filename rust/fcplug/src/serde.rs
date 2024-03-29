use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::{
    ABIResult, FromMessage, IntoMessage, ResultMsg, TryFromBytes, TryIntoBytes, RC_DECODE,
};

#[derive(Debug)]
pub struct JsonMessage<T: for<'a> Deserialize<'a> + Serialize + Debug>(pub T);

impl<T> FromMessage<JsonMessage<T>> for T
where
    T: for<'a> Deserialize<'a> + Serialize + Debug,
{
    #[inline]
    fn from_message(value: JsonMessage<T>) -> Self {
        value.0
    }
}

impl<T> IntoMessage<JsonMessage<T>> for T
where
    T: for<'a> Deserialize<'a> + Serialize + Debug,
{
    #[inline]
    fn into_message(self) -> JsonMessage<T> {
        JsonMessage(self)
    }
}

impl<T> TryFromBytes<'_> for JsonMessage<T>
where
    T: for<'a> Deserialize<'a> + Serialize + Debug,
{
    #[inline]
    fn try_from_bytes(buf: &mut [u8]) -> ABIResult<Self>
    where
        Self: Sized,
    {
        Ok(serde_json::from_slice::<T>(buf as &[u8])
            .map(JsonMessage)
            .map_err(decode_map_err)?)
    }
}

impl<T> TryIntoBytes for JsonMessage<T>
where
    T: for<'a> Deserialize<'a> + Serialize + Debug,
{
    #[inline]
    fn try_into_bytes(self) -> ABIResult<Vec<u8>> {
        Ok(serde_json::to_vec(&self.0).map_err(encode_map_err)?)
    }
}

#[inline]
fn decode_map_err(e: serde_json::Error) -> ResultMsg {
    ResultMsg {
        code: RC_DECODE,
        msg: e.to_string(),
    }
}

#[inline]
fn encode_map_err(e: serde_json::Error) -> ResultMsg {
    ResultMsg {
        code: RC_DECODE,
        msg: e.to_string(),
    }
}
