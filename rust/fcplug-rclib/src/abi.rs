use std::fmt::{Display, Formatter};
use crate::fcplug::FFIResult;
use protobuf::{Message, well_known_types::Any};
use crate::ABIMessage;

pub type ABIResult<T> = Result<T, AbiError>;

impl<T: ABIMessage> From<FFIResult> for ABIResult<T> {
    fn from(value: FFIResult) -> Self {
        if value.get_code() == 0 {
            Ok(value.get_data().unpack::<T>().unwrap_or_default().unwrap_or_default())
        } else {
            Err(AbiError { code: value.get_code(), msg: value.get_msg().to_string() })
        }
    }
}

#[derive(Debug, Default)]
pub struct AbiError {
    pub code: i32,
    pub msg: String,
}

impl<T: ABIMessage> From<ABIResult<T>> for FFIResult {
    fn from(value: ABIResult<T>) -> Self {
        let mut r = FFIResult::new();
        match value {
            Ok(t) => {
                r.set_data(Any::pack(&t).unwrap_or_default());
            }
            Err(e) => {
                r.set_code(if e.code == 0 { -1 } else { e.code });
                r.set_msg(e.msg)
            }
        }
        r
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Buffer {
    ptr: *mut u8,
    len: usize,
    cap: usize,
}

impl Display for Buffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.read())
    }
}

impl<T: ABIMessage> From<T> for Buffer {
    fn from(value: T) -> Self {
        value
            .write_to_bytes()
            .map_or(Buffer::null(), Buffer::from_vec)
    }
}


#[no_mangle]
pub extern "C" fn free_buffer(buf: Buffer) {
    unsafe {
        let _ = buf.consume();
    }
}

impl Buffer {
    pub(crate) fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }

    /// read provides a reference to the included data to be parsed or copied elsewhere
    /// data is only guaranteed to live as long as the Buffer
    /// (or the scope of the extern "C" call it came from)
    pub(crate) fn read(&self) -> Option<&[u8]> {
        if self.is_empty() {
            None
        } else {
            unsafe { Some(std::slice::from_raw_parts(self.ptr, self.len)) }
        }
    }

    /// consume must only be used on memory previously released by from_vec
    /// when the Vec is out of scope, it will deallocate the memory previously referenced by Buffer
    pub(crate) unsafe fn consume(self) -> Vec<u8> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut v = Vec::from_raw_parts(self.ptr, self.len, self.cap);
        v.shrink_to_fit();
        v
    }

    /// this releases our memory to the caller
    pub(crate) fn from_vec(mut v: Vec<u8>) -> Self {
        let buf = Buffer {
            ptr: v.as_mut_ptr(),
            len: v.len(),
            cap: v.capacity(),
        };
        std::mem::forget(v);
        buf
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.ptr.is_null() || self.len == 0 || self.cap == 0
    }

    pub(crate) fn decode<T: ABIMessage>(&self) -> ABIResult<T> {
        self.read()
            .map_or_else(|| Ok(T::default()), |b| T::parse_from_bytes(b).map_err(codec_map_err))
    }
    pub(crate) fn from_abi_result<T: ABIMessage>(value: ABIResult<T>) -> Self {
        Buffer::from(FFIResult::from(value))
    }
    pub fn to_abi_result<T: ABIMessage>(&self) -> ABIResult<T> {
        self.read()
            .map_or_else(
                || Ok(T::default()),
                |b| FFIResult::parse_from_bytes(b)
                    .map_or_else(|e| Err(codec_map_err(e)), ABIResult::<T>::from))
    }
}

fn codec_map_err<E: ToString>(e: E) -> AbiError {
    AbiError { code: -1, msg: e.to_string() }
}
