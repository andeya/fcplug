use std::convert::Infallible;
use std::fmt::{Debug, Display, Formatter};
use std::ops::FromResidual;

use flatbuffers::FlatBufferBuilder;
#[cfg(debug_assertions)]
use tracing::error;

#[no_mangle]
pub extern "C" fn no_mangle_types(_: Buffer, _: FFIResult) {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn free_buffer(free_type: OriginType, free_ptr: usize) {
    unsafe {
        match free_type {
            OriginType::Vec => {
                #[cfg(debug_assertions)]   println!("OriginType::Vec, free_ptr={}", free_ptr);
                let ptr = free_ptr as *mut Vec<u8>;
                if ptr.is_null() {
                    return;
                }
                let _ = Box::from_raw(ptr);
            }
            OriginType::FlatBuffer => {
                #[cfg(debug_assertions)] println!("OriginType::FlatBuffer, free_ptr={}", free_ptr);
                let ptr = free_ptr as *mut FlatBufferBuilder;
                if ptr.is_null() {
                    return;
                }
                let _ = Box::from_raw(ptr);
            }
        };
    }
}

pub fn callback<'a, Request, Response, F>(_ffi_method_name: &'static str, method_fn: F, args: &'a Buffer) -> FFIResult
    where Request: ABIRequest<'a>,
          Response: ABIResponse,
          F: Fn(Request) -> ABIResult<Response> {
    let args_obj = match Request::try_from_bytes(args.read().unwrap_or_default()) {
        Ok(args_obj) => args_obj,
        Err(err) => return FFIResult::err(ResultCode::Decode, Some(err)),
    };

    #[cfg(debug_assertions)]
        let txt = format!("invoking method={}, args_bytes={:?}, args_obj={:?}", _ffi_method_name, args.read(), args_obj);

    let res_obj: ABIResult<Response> = method_fn(args_obj);

    #[cfg(debug_assertions)]
        let txt = format!("{}, abi_result={:?}", txt, res_obj);

    let res = FFIResult::from(res_obj);

    #[cfg(debug_assertions)]
    println!("{}, ffi_result={:?}", txt, res);

    res
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Buffer {
    pub ptr: *mut u8,
    pub len: usize,
    pub cap: usize,
}

impl Display for Buffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.read())
    }
}

impl Buffer {
    pub const fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            len: 0,
            cap: 0,
        }
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


    pub(crate) fn is_empty(&self) -> bool {
        self.ptr.is_null() || self.len == 0 || self.cap == 0
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LeakBuffer {
    pub(crate) free_type: OriginType,
    pub(crate) free_ptr: usize,
    pub(crate) buffer: Buffer,
}


impl LeakBuffer {
    pub unsafe fn mem_free(self) {
        free_buffer(self.free_type, self.free_ptr)
    }

    pub const fn null() -> Self {
        Self {
            free_type: OriginType::Vec,
            free_ptr: 0,
            buffer: Buffer::null(),
        }
    }

    /// this releases our memory to the caller
    pub(crate) fn from_vec(mut v: Vec<u8>) -> Self {
        let mut buf = Self {
            free_type: OriginType::Vec,
            free_ptr: 0,
            buffer: Buffer {
                ptr: v.as_mut_ptr(),
                len: v.len(),
                cap: v.capacity(),
            },
        };
        buf.free_ptr = Box::into_raw(Box::new(v)) as usize;
        // mem::forget(v);
        buf
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.free_ptr == 0 || self.buffer.is_empty()
    }
    pub fn read(&self) -> Option<&[u8]> {
        if self.is_empty() {
            None
        } else {
            unsafe { Some(std::slice::from_raw_parts(self.buffer.ptr, self.buffer.len)) }
        }
    }
    // pub(crate) fn consume_vec(self) -> Vec<u8> {
    //     match self.free_type {
    //         OriginType::Vec => {
    //             unsafe { *Box::from_raw(self.free_ptr as *mut Vec<u8>) }
    //         }
    //         OriginType::FlatBuffer => {
    //             let v = self.read().map_or_else(Vec::new, |v| v.to_owned());
    //             unsafe { self.mem_free(); }
    //             v
    //         }
    //     }
    // }
    pub fn buffer(&self) -> Buffer {
        self.buffer
    }
}


#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum ResultCode {
    NoError = 0,
    Decode = 1,
    Encode = 2,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum OriginType {
    Vec = 0,
    FlatBuffer = 1,
}

#[derive(Debug)]
#[repr(C)]
pub struct FFIResult {
    pub code: ResultCode,
    pub data: LeakBuffer,
}

impl FFIResult {
    pub fn ok(data: LeakBuffer) -> Self {
        Self { code: ResultCode::NoError, data }
    }
    pub(crate) fn err<E: Debug>(code: ResultCode, _err: Option<E>) -> Self {
        #[cfg(debug_assertions)]{
            if let Some(err) = _err {
                error!("{:?}", err);
            }
        }
        Self { code, data: LeakBuffer::null() }
    }
}

pub type ABIResult<T> = Result<T, ResultCode>;

impl<T: ABIResponse> From<ABIResult<T>> for FFIResult {
    fn from(value: ABIResult<T>) -> Self {
        match value {
            Ok(v) => match v.try_into_buffer() {
                Ok(v) => Self::ok(v),
                Err(e) => Self::err(ResultCode::Encode, Some(e)),
            },
            Err(e) => Self::err(ResultCode::Encode, Some(e))
        }
    }
}

impl<'a, T: ABIRequest<'a>> From<&'a FFIResult> for ABIResult<T> {
    fn from(value: &'a FFIResult) -> Self {
        match value.code {
            ResultCode::NoError => {
                ABIRequest::try_from_bytes(value.data.read().unwrap_or_default())
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

pub trait ABIMessage<'a>: ABIRequest<'a> + ABIResponse {}

pub trait ABIRequest<'a>: Debug {
    type DecodeError: Debug;
    fn try_from_bytes(buf: &'a [u8]) -> Result<Self, Self::DecodeError> where Self: Sized;
}

pub trait ABIResponse: Debug {
    type EncodeError: Debug;
    const ORIGIN_TYPE_FOR_FREE: OriginType;
    fn try_into_buffer(self) -> Result<LeakBuffer, Self::EncodeError>;
}

impl FromResidual<Result<Infallible, ResultCode>> for FFIResult {
    fn from_residual(residual: Result<Infallible, ResultCode>) -> Self {
        Self { code: residual.unwrap_err(), data: LeakBuffer::null() }
    }
}

#[test]
fn test_free() {
    let buf = LeakBuffer::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    println!("{:?}", buf.read());
    {
        println!("{:?}", unsafe { Vec::from_raw_parts(buf.buffer.ptr, buf.buffer.len, buf.buffer.cap) });
    }
    println!("{:?}", buf.read());
}
