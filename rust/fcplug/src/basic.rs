// ---------------------------------TryFromBytes implement-------------------------------

use crate::{TryFromBytes, TryIntoBytes};

impl TryFromBytes<'_> for Vec<u8> {
    fn try_from_bytes(buf: &'_ mut [u8]) -> anyhow::Result<Self>
        where
            Self: Sized,
    {
        Ok(buf.to_owned())
    }
}

impl<'a> TryFromBytes<'a> for &'a [u8] {
    fn try_from_bytes(buf: &'a mut [u8]) -> anyhow::Result<Self>
        where
            Self: Sized,
    {
        Ok(buf)
    }
}

impl<'a> TryFromBytes<'a> for &'a str {
    fn try_from_bytes(buf: &'a mut [u8]) -> anyhow::Result<Self>
        where
            Self: Sized,
    {
        Ok(unsafe { std::str::from_utf8_unchecked(buf) })
    }
}


impl<'a> TryFromBytes<'a> for &'a mut [u8] {
    fn try_from_bytes(buf: &'a mut [u8]) -> anyhow::Result<Self>
        where
            Self: Sized,
    {
        Ok(buf)
    }
}

impl<'a> TryFromBytes<'a> for &'a mut str {
    fn try_from_bytes(buf: &'a mut [u8]) -> anyhow::Result<Self>
        where
            Self: Sized,
    {
        Ok(unsafe { std::str::from_utf8_unchecked_mut(buf) })
    }
}


// ---------------------------------TryIntoBytes implement-------------------------------

impl TryIntoBytes for Vec<u8> {
    fn try_into_bytes(self) -> anyhow::Result<Vec<u8>> where Self: Sized {
        Ok(self)
    }
}

impl TryIntoBytes for String {
    fn try_into_bytes(self) -> anyhow::Result<Vec<u8>> where Self: Sized {
        Ok(self.into_bytes())
    }
}
