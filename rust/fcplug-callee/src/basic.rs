use std::convert::Infallible;

use crate::{ABIRequest, ABIResponse};
use crate::abi::{LeakBuffer, OriginType};

// ---------------------------------ABIRequest implement-------------------------------

impl ABIRequest<'_> for Vec<u8> {
    type DecodeError = Infallible;

    fn try_from_bytes(buf: &'_ mut [u8]) -> Result<Self, Self::DecodeError>
        where
            Self: Sized,
    {
        Ok(buf.to_owned())
    }
}


impl<'a> ABIRequest<'a> for &'a [u8] {
    type DecodeError = Infallible;

    fn try_from_bytes(buf: &'a mut [u8]) -> Result<Self, Self::DecodeError>
        where
            Self: Sized,
    {
        Ok(buf)
    }
}

impl<'a> ABIRequest<'a> for &'a str {
    type DecodeError = Infallible;

    fn try_from_bytes(buf: &'a mut [u8]) -> Result<Self, Self::DecodeError>
        where
            Self: Sized,
    {
        Ok(unsafe { std::str::from_utf8_unchecked(buf) })
    }
}


impl<'a> ABIRequest<'a> for &'a mut [u8] {
    type DecodeError = Infallible;

    fn try_from_bytes(buf: &'a mut [u8]) -> Result<Self, Self::DecodeError>
        where
            Self: Sized,
    {
        Ok(buf)
    }
}

impl<'a> ABIRequest<'a> for &'a mut str {
    type DecodeError = Infallible;

    fn try_from_bytes(buf: &'a mut [u8]) -> Result<Self, Self::DecodeError>
        where
            Self: Sized,
    {
        Ok(unsafe { std::str::from_utf8_unchecked_mut(buf) })
    }
}


// ---------------------------------ABIResponse implement-------------------------------

impl ABIResponse for Vec<u8> {
    type EncodeError = Infallible;
    const ORIGIN_TYPE_FOR_FREE: OriginType = OriginType::Vec;

    fn try_into_buffer(self) -> Result<LeakBuffer, Self::EncodeError> {
        Ok(LeakBuffer::from_vec(self))
    }
}

impl ABIResponse for String {
    type EncodeError = Infallible;
    const ORIGIN_TYPE_FOR_FREE: OriginType = OriginType::Vec;

    fn try_into_buffer(self) -> Result<LeakBuffer, Self::EncodeError> {
        Ok(LeakBuffer::from_vec(self.into_bytes()))
    }
}
