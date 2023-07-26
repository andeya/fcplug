#![feature(downcast_unchecked)]
#![feature(try_trait_v2)]
#![feature(new_uninit)]
#![feature(const_trait_impl)]

use std::convert::Infallible;
use std::fmt::{Debug, Display, Formatter};
use std::ops::FromResidual;

#[cfg(debug_assertions)]
use tracing::error;

mod basic;
pub mod protobuf;
pub mod serde;


#[inline]
#[no_mangle]
pub extern "C" fn no_mangle_types(_: Buffer, _: RustFfiResult, _: GoFfiResult) {
    unimplemented!()
}

#[inline]
#[no_mangle]
pub extern "C" fn free_buffer(buf: Buffer) {
    unsafe { buf.mem_free() }
}

#[inline]
#[no_mangle]
pub extern "C" fn leak_buffer(buf: Buffer) -> usize {
    if let Some(v) = buf.read() {
        Box::leak(Box::new(v.to_vec())) as *mut Vec<u8> as usize
    } else {
        0
    }
}

pub trait FromMessage<M> {
    fn from_message(value: M) -> Self;
    fn try_from_bytes(buf: &mut [u8]) -> ABIResult<Self>
        where Self: Sized,
              M: for<'a> TryFromBytes<'a>
    {
        Ok(Self::from_message(M::try_from_bytes(buf)?))
    }
}

pub trait IntoMessage<M> {
    fn into_message(self) -> M;
    fn try_into_bytes(self) -> ABIResult<Vec<u8>> where
        Self: Sized,
        M: TryIntoBytes,
    {
        self.into_message().try_into_bytes()
    }
}

pub trait ABIMessage<'a>: TryFromBytes<'a> + TryIntoBytes {}

pub trait TryFromBytes<'b>: Debug {
    fn try_from_bytes(buf: &'b mut [u8]) -> ABIResult<Self> where Self: Sized;
}

pub trait TryIntoBytes: Debug {
    fn try_into_bytes(self) -> ABIResult<Vec<u8>> where Self: Sized;
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Buffer {
    pub ptr: *mut u8,
    pub len: usize,
    pub cap: usize,
}

impl Default for Buffer {
    #[inline]
    fn default() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }
}


impl Buffer {
    #[inline]
    pub fn null() -> Self {
        Self::default()
    }

    /// read provides a reference to the included data to be parsed or copied elsewhere
    /// data is only guaranteed to live as long as the Buffer
    /// (or the scope of the extern "C" call it came from)
    #[inline]
    pub fn read(&self) -> Option<&[u8]> {
        if self.is_empty() {
            None
        } else {
            unsafe { Some(std::slice::from_raw_parts(self.ptr, self.len)) }
        }
    }

    /// read_mut provides a reference to the included data to be parsed or copied elsewhere
    /// data is only guaranteed to live as long as the Buffer
    /// (or the scope of the extern "C" call it came from)
    #[inline]
    pub fn read_mut(&mut self) -> Option<&mut [u8]> {
        if self.is_empty() {
            None
        } else {
            unsafe { Some(std::slice::from_raw_parts_mut(self.ptr, self.len)) }
        }
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.ptr.is_null() || self.len == 0 || self.cap == 0
    }


    #[inline]
    pub(crate) unsafe fn mem_free(self) {
        if !self.ptr.is_null() {
            let _ = unsafe { Vec::from_raw_parts(self.ptr, self.len, self.cap) };
        }
    }

    /// this releases our memory to the caller
    #[inline]
    pub fn from_vec(mut v: Vec<u8>) -> Self {
        if v.is_empty() {
            Self::null()
        } else {
            v.shrink_to_fit();
            Self {
                len: v.len(),
                cap: v.capacity(),
                ptr: v.leak().as_mut_ptr(),
            }
        }
    }

    // pub(crate) fn consume_vec(self) -> Vec<u8> {
    //     if !self.ptr.is_null() {
    //         unsafe { Vec::from_raw_parts(self.ptr, self.len, self.cap) }
    //     } else {
    //         Vec::new()
    //     }
    // }
}

impl Display for Buffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.read())
    }
}


pub type ResultCode = i8;

#[derive(Debug)]
pub struct ResultMsg {
    pub code: ResultCode,
    pub msg: String,
}

const RC_NO_ERROR: ResultCode = 0;
const RC_DECODE: ResultCode = -1;
const RC_ENCODE: ResultCode = -2;
const RC_UNKNOWN: ResultCode = -128;

pub type ABIResult<T> = Result<T, ResultMsg>;

#[derive(Debug, Clone)]
pub struct TBytes<T> {
    pub bytes: Vec<u8>,
    _p: std::marker::PhantomData<T>,
}

impl<T> TBytes<T> {
    #[inline(always)]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes, _p: Default::default() }
    }
}

impl<T> TBytes<T> {
    #[inline]
    pub fn try_from<M>(value: T) -> ABIResult<Self> where
        T: IntoMessage<M>,
        M: TryIntoBytes,
    {
        Ok(TBytes::<T>::new(T::into_message(value).try_into_bytes()?))
    }
}

pub trait TryIntoTBytes {
    #[inline]
    fn try_into_tbytes<M>(self) -> ABIResult<TBytes<Self>>
        where
            Self: IntoMessage<M> + Sized,
            for<'a> M: TryIntoBytes,
    {
        Ok(TBytes::<Self>::new(self.into_message().try_into_bytes()?))
    }
}

impl<T> TryIntoTBytes for T {}

impl<T: Debug> TryIntoBytes for TBytes<T> {
    fn try_into_bytes(self) -> ABIResult<Vec<u8>> where Self: Sized {
        Ok(self.bytes)
    }
}

#[derive(Debug)]
pub struct RustFfiArg<T> {
    buf: Buffer,
    _p: std::marker::PhantomData<T>,
}

impl<T> RustFfiArg<T> {
    #[inline(always)]
    pub fn from(buf: Buffer) -> Self {
        Self { buf, _p: Default::default() }
    }
    pub fn bytes(&self) -> &[u8] { self.buf.read().unwrap_or_default() }
    pub fn bytes_mut(&mut self) -> &mut [u8] { self.buf.read_mut().unwrap_or_default() }
    pub fn try_to_object<U>(&mut self) -> ABIResult<T> where
        Self: Sized,
        for<'a> U: TryFromBytes<'a>,
        T: FromMessage<U>,
    { Ok(T::from_message(U::try_from_bytes(self.bytes_mut())?)) }
}

#[derive(Debug)]
#[repr(C)]
pub struct GoFfiResult {
    pub code: ResultCode,
    pub data_ptr: usize,
}

impl GoFfiResult {
    #[inline]
    pub fn from_ok<T>(data: T) -> Self {
        Self { code: RC_NO_ERROR, data_ptr: Box::leak(Box::new(data)) as *mut T as usize }
    }
    #[inline]
    pub(crate) fn from_err(mut ret_msg: ResultMsg) -> Self {
        let err = ret_msg.msg.to_string();
        #[cfg(debug_assertions)]{
            error!("{}", err);
        }
        if ret_msg.code == 0 {
            ret_msg.code = RC_UNKNOWN
        }
        Self { code: ret_msg.code, data_ptr: Box::leak(Box::new(err)) as *mut String as usize }
    }
    #[inline]
    pub unsafe fn consume_data<T>(&mut self) -> Option<T> {
        let data_ptr = self.data_ptr as *mut T;
        if data_ptr.is_null() {
            None
        } else {
            self.data_ptr = std::ptr::null_mut::<u8>() as usize;
            Some(*Box::from_raw(data_ptr))
        }
    }
}


impl FromResidual<Result<Infallible, ResultMsg>> for GoFfiResult {
    #[inline]
    fn from_residual(residual: Result<Infallible, ResultMsg>) -> Self {
        let ResultMsg { code, msg } = residual.unwrap_err();
        Self { code, data_ptr: Box::leak(Box::new(msg)) as *mut String as usize }
    }
}

impl<T: TryIntoBytes> From<ABIResult<T>> for GoFfiResult {
    #[inline]
    fn from(value: ABIResult<T>) -> Self {
        match value {
            Ok(v) => match v.try_into_bytes() {
                Ok(v) => Self::from_ok(Buffer::from_vec(v)),
                Err(e) => {
                    Self::from_err(e)
                }
            },
            Err(e) => Self::from_err(e)
        }
    }
}

impl<T: Default> From<GoFfiResult> for ABIResult<T> {
    #[inline]
    fn from(mut value: GoFfiResult) -> Self {
        match value.code {
            RC_NO_ERROR => {
                if let Some(v) = unsafe { value.consume_data::<T>() } {
                    Ok(v)
                } else {
                    Ok(T::default())
                }
            }
            code => {
                let msg = if let Some(v) = unsafe { value.consume_data::<String>() } {
                    v
                } else {
                    String::default()
                };
                Err(ResultMsg { code, msg })
            }
        }
    }
}


#[derive(Debug)]
#[repr(C)]
pub struct RustFfiResult {
    pub code: ResultCode,
    pub data: Buffer,
}

impl RustFfiResult {
    #[inline]
    pub fn from_ok(data: Buffer) -> Self {
        Self { code: RC_NO_ERROR, data }
    }
    #[inline]
    pub(crate) fn from_err(mut ret_msg: ResultMsg) -> Self {
        #[cfg(debug_assertions)]{
            let err = ret_msg.msg.to_string();
            error!("{}", err);
        }
        if ret_msg.code == 0 {
            ret_msg.code = RC_UNKNOWN
        }
        Self { code: ret_msg.code, data: Buffer::from_vec(ret_msg.msg.into_bytes()) }
    }
}

impl FromResidual<Result<Infallible, ResultCode>> for RustFfiResult {
    fn from_residual(residual: Result<Infallible, ResultCode>) -> Self {
        Self { code: residual.unwrap_err(), data: Buffer::null() }
    }
}

impl<T: TryIntoBytes> From<ABIResult<T>> for RustFfiResult {
    #[inline]
    fn from(value: ABIResult<T>) -> Self {
        match value {
            Ok(v) => match v.try_into_bytes() {
                Ok(v) => Self::from_ok(Buffer::from_vec(v)),
                Err(e) => Self::from_err(e),
            },
            Err(e) => Self::from_err(e)
        }
    }
}

impl<'a, T: TryFromBytes<'a>> From<&'a mut RustFfiResult> for ABIResult<T> {
    #[inline]
    fn from(value: &'a mut RustFfiResult) -> Self {
        match value.code {
            RC_NO_ERROR => {
                TryFromBytes::try_from_bytes(value.data.read_mut().unwrap_or_default())
                    .map_err(|err| {
                        let msg = format!("{:?}", err);
                        #[cfg(debug_assertions)]{
                            error!("{}", msg);
                        }
                        ResultMsg { code: RC_DECODE, msg }
                    })
            }
            code => {
                Err(ResultMsg { code, msg: value.data.to_string() })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Buffer;

    #[test]
    fn it_works() {
        assert_eq!(4, 2 + 2);
    }

    #[test]
    fn test_free() {
        let buf = Buffer::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
        println!("{:?}", buf.read());
        {
            println!("{:?}", unsafe { Vec::from_raw_parts(buf.ptr, buf.len, buf.cap) });
        }
        println!("{:?}", buf.read());
    }
}
