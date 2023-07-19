use std::fmt::Debug;

use protobuf::MessageFull;

use crate::{TryFromBytes, TryIntoBytes};

#[derive(Debug)]
pub struct PbMessage<T: MessageFull>(pub T);

impl<T: MessageFull> PbMessage<T> {
    pub fn new(v: T) -> Self {
        Self(v)
    }
}


impl<'a, T: MessageFull> TryFromBytes<'a> for PbMessage<T> {
    fn try_from_bytes(buf: &'a mut [u8]) -> anyhow::Result<Self> where Self: Sized {
        Ok(T::parse_from_bytes(buf).map(PbMessage)?)
    }
}

impl<T: MessageFull> TryIntoBytes for PbMessage<T> {
    fn try_into_bytes(self) -> anyhow::Result<Vec<u8>> {
        Ok(self.0.write_to_bytes()?)
    }
}
