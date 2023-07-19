#![feature(downcast_unchecked)]
#![feature(try_trait_v2)]
#![feature(new_uninit)]
#![feature(const_trait_impl)]

use std::convert::Infallible;
use std::fmt::{Debug, Display, Formatter};
use std::ops::FromResidual;

#[cfg(debug_assertions)]
use tracing::error;

pub use fcplug_macros::{ffi_fb_callee, ffi_pb_callee, ffi_raw_callee};

pub mod callee;
pub mod caller;
mod basic;
mod protobuf;


#[no_mangle]
pub extern "C" fn no_mangle_types(_: Buffer, _: FFIResult) {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn free_buffer(buf: Buffer) {
    unsafe { buf.mem_free() }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Buffer {
    pub ptr: *mut u8,
    pub len: usize,
    pub cap: usize,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }
}


impl Buffer {
    pub fn null() -> Self {
        Self::default()
    }

    /// read provides a reference to the included data to be parsed or copied elsewhere
    /// data is only guaranteed to live as long as the Buffer
    /// (or the scope of the extern "C" call it came from)
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
    pub fn read_mut(&mut self) -> Option<&mut [u8]> {
        if self.is_empty() {
            None
        } else {
            unsafe { Some(std::slice::from_raw_parts_mut(self.ptr, self.len)) }
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.ptr.is_null() || self.len == 0 || self.cap == 0
    }


    pub(crate) unsafe fn mem_free(self) {
        if !self.ptr.is_null() {
            let _ = unsafe { Vec::from_raw_parts(self.ptr, self.len, self.cap) };
        }
    }

    /// this releases our memory to the caller
    pub(crate) fn from_vec(mut v: Vec<u8>) -> Self {
        let mut buf = Self {
            len: v.len(),
            cap: v.capacity(),
            ptr: v.leak().as_mut_ptr(),
        };
        buf
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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum ResultCode {
    NoError = 0,
    Decode = 1,
    Encode = 2,
}

pub type ABIResult<T> = Result<T, ResultCode>;

pub trait ABIMessage<'a>: TryFromBytes<'a> + TryIntoBytes {}

pub trait TryFromBytes<'a>: Debug {
    fn try_from_bytes(buf: &'a mut [u8]) -> anyhow::Result<Self> where Self: Sized;
}

pub trait TryIntoBytes: Debug {
    fn try_into_bytes(self) -> anyhow::Result<Vec<u8>> where Self: Sized;
}

#[derive(Debug)]
#[repr(C)]
pub struct FFIResult {
    pub code: ResultCode,
    pub data: Buffer,
}

impl FFIResult {
    pub fn ok(data: Buffer) -> Self {
        Self { code: ResultCode::NoError, data }
    }
    pub(crate) fn err<E: Debug>(code: ResultCode, _err: Option<E>) -> Self {
        #[cfg(debug_assertions)]{
            if let Some(err) = _err {
                error!("{:?}", err);
            }
        }
        Self { code, data: Buffer::null() }
    }
}

impl FromResidual<Result<Infallible, ResultCode>> for FFIResult {
    fn from_residual(residual: Result<Infallible, ResultCode>) -> Self {
        Self { code: residual.unwrap_err(), data: Buffer::null() }
    }
}

impl<T: TryIntoBytes> From<ABIResult<T>> for FFIResult {
    fn from(value: ABIResult<T>) -> Self {
        match value {
            Ok(v) => match v.try_into_bytes() {
                Ok(v) => Self::ok(Buffer::from_vec(v)),
                Err(e) => Self::err(ResultCode::Encode, Some(e)),
            },
            Err(e) => Self::err(ResultCode::Encode, Some(e))
        }
    }
}

impl<'a, T: TryFromBytes<'a>> From<&'a mut FFIResult> for ABIResult<T> {
    fn from(value: &'a mut FFIResult) -> Self {
        match value.code {
            ResultCode::NoError => {
                TryFromBytes::try_from_bytes(value.data.read_mut().unwrap_or_default())
                    .map_err(|_err| {
                        #[cfg(debug_assertions)]{
                            error!("{:?}", _err);
                        }
                        ResultCode::Decode
                    })
            }
            code => Err(code)
        }
    }
}


// #[macro_export]
// macro_rules! include_gen_file {
//     ($gen_file: tt) => {
//         include!(concat!(env!("OUT_DIR"), concat!("/", $gen_file)));
//     };
// }

// #[macro_export]
// macro_rules! include_goffi_gen {
//     () => {
//         include!(concat!(env!("OUT_DIR"), concat!("/", "goffi_gen.rs")));
//     };
// }

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
