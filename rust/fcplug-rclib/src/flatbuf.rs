use std::convert::Infallible;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub use flatbuffers::{FlatBufferBuilder, WIPOffset};
use flatbuffers::{Follow, InvalidFlatbuffer, Verifiable};
#[cfg(debug_assertions)]
use tracing::error;

use crate::{ABIResult, Buffer, ResultCode};
use crate::abi::{ABIRequest, ABIResponse, LeakBuffer, OriginType};

pub struct FbRequest<'a, T> {
    request: T,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> FbRequest<'a, T> where T: 'a {
    pub fn new(req: T) -> Self {
        FbRequest { request: req, _phantom: Default::default() }
    }
    pub fn new_response_writer<U>(&self) -> FbResponseWriter<U> {
        FbResponseWriter { fbb: Box::new(FlatBufferBuilder::new()), _phantom: Default::default() }
    }
    pub fn try_from_buffer(req: &'a Buffer) -> ABIResult<Self>
        where T: 'a + Follow<'a, Inner=T> + Verifiable,
    {
        Self::try_from_bytes(req.read().unwrap_or_default())
            .map_err(|_err| {
                #[cfg(debug_assertions)]{
                    error!("{:?}", _err);
                }
                ResultCode::Decode
            })
    }
}

impl<'a, T> Deref for FbRequest<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl<'a, T> Debug for FbRequest<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::any::type_name::<T>())
    }
}

impl<'a, T> ABIRequest<'a> for FbRequest<'a, T> where
    T: 'a + Follow<'a, Inner=T> + Verifiable {
    type DecodeError = InvalidFlatbuffer;

    fn try_from_bytes(buf: &'a [u8]) -> Result<Self, Self::DecodeError> where Self: Sized {
        flatbuffers::root::<T>(buf).map(|v| Self::new(v))
    }
}

pub struct FbResponseWriter<T> {
    fbb: Box<FlatBufferBuilder<'static>>,
    _phantom: PhantomData<T>,
}

impl<T> FbResponseWriter<T> {
    pub fn mut_fbb(&mut self) -> &mut FlatBufferBuilder<'static> {
        &mut self.fbb
    }
    fn into_raw(self) -> *mut FlatBufferBuilder<'static> {
        Box::into_raw(self.fbb)
    }
}

impl<T> Deref for FbResponseWriter<T> {
    type Target = FlatBufferBuilder<'static>;

    fn deref(&self) -> &Self::Target {
        &self.fbb
    }
}

impl<T> DerefMut for FbResponseWriter<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fbb
    }
}

impl<T> Debug for FbResponseWriter<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::any::type_name::<T>())
    }
}

impl<T> ABIResponse for FbResponseWriter<T> {
    type EncodeError = Infallible;
    const ORIGIN_TYPE_FOR_FREE: OriginType = OriginType::FlatBuffer;

    fn try_into_buffer(mut self) -> Result<LeakBuffer, Self::EncodeError> {
        let b = self.mut_finished_buffer();
        let mut buf = LeakBuffer {
            free_type: OriginType::FlatBuffer,
            free_ptr: 0,
            buffer: Buffer {
                ptr: b.0[b.1..].as_mut_ptr(),
                len: b.0.len() - b.1,
                cap: b.0.len(),
            },
        };
        buf.free_ptr = self.into_raw() as usize;
        Ok(buf)
    }
}
