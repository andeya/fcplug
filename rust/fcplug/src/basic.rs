// ---------------------------------TryFromBytes implement-------------------------------

use crate::{ABIResult, TryFromBytes, TryIntoBytes};

impl TryFromBytes<'_> for Vec<u8> {
    #[inline(always)]
    fn try_from_bytes(buf: &mut [u8]) -> ABIResult<Self>
    where
        Self: Sized,
    {
        Ok(buf.to_owned())
    }
}

impl<'a> TryFromBytes<'a> for &'a [u8] {
    #[inline(always)]
    fn try_from_bytes(buf: &'a mut [u8]) -> ABIResult<Self>
    where
        Self: Sized,
    {
        Ok(buf)
    }
}

impl<'a> TryFromBytes<'a> for &'a str {
    #[inline(always)]
    fn try_from_bytes(buf: &'a mut [u8]) -> ABIResult<Self>
    where
        Self: Sized,
    {
        Ok(unsafe { std::str::from_utf8_unchecked(buf) })
    }
}

impl<'a> TryFromBytes<'a> for &'a mut [u8] {
    #[inline(always)]
    fn try_from_bytes(buf: &'a mut [u8]) -> ABIResult<Self>
    where
        Self: Sized,
    {
        Ok(buf)
    }
}

impl<'a> TryFromBytes<'a> for &'a mut str {
    #[inline(always)]
    fn try_from_bytes(buf: &'a mut [u8]) -> ABIResult<Self>
    where
        Self: Sized,
    {
        Ok(unsafe { std::str::from_utf8_unchecked_mut(buf) })
    }
}

impl TryFromBytes<'_> for () {
    #[inline(always)]
    fn try_from_bytes(_: &mut [u8]) -> ABIResult<Self>
    where
        Self: Sized,
    {
        Ok(())
    }
}

// ---------------------------------TryIntoBytes implement-------------------------------

impl TryIntoBytes for Vec<u8> {
    #[inline(always)]
    fn try_into_bytes(self) -> ABIResult<Vec<u8>>
    where
        Self: Sized,
    {
        Ok(self)
    }
}

impl TryIntoBytes for String {
    #[inline(always)]
    fn try_into_bytes(self) -> ABIResult<Vec<u8>>
    where
        Self: Sized,
    {
        Ok(self.into_bytes())
    }
}

const EMPTY_BYTES: Vec<u8> = Vec::new();

impl TryIntoBytes for () {
    #[inline(always)]
    fn try_into_bytes(self) -> ABIResult<Vec<u8>>
    where
        Self: Sized,
    {
        Ok(EMPTY_BYTES)
    }
}
