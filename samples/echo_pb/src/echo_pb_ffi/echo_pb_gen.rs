// Code generated by fcplug. DO NOT EDIT.
#![allow(warnings, clippy::all)]
#[derive(
    PartialOrd,
    Hash,
    Eq,
    Ord,
    Debug,
    Default,
    ::serde::Serialize,
    ::serde::Deserialize,
    Clone,
    PartialEq,
)]
pub struct Ping {
    pub msg: ::std::string::String,
}
impl ::pilota::prost::Message for Ping {
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + ::pilota::prost::encoding::string::encoded_len(1, &self.msg)
    }

    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::pilota::prost::bytes::BufMut,
    {
        ::pilota::prost::encoding::string::encode(1, &self.msg, buf);
    }

    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::pilota::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::pilota::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::pilota::prost::DecodeError>
    where
        B: ::pilota::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = stringify!(Ping);
        match tag {
            1 => {
                let mut _inner_pilota_value = &mut self.msg;
                ::pilota::prost::encoding::string::merge(wire_type, _inner_pilota_value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(msg));
                        error
                    })
            }
            _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
}

pub(super) trait RustFfi {
    fn echo_rs(req: ::fcplug::RustFfiArg<Ping>) -> ::fcplug::ABIResult<::fcplug::TBytes<Pong>>;
}
#[no_mangle]
#[inline]
pub extern "C" fn rustffi_echo_rs(req: ::fcplug::Buffer) -> ::fcplug::RustFfiResult {
    ::fcplug::RustFfiResult::from(<FfiImpl as RustFfi>::echo_rs(::fcplug::RustFfiArg::from(
        req,
    )))
}
#[derive(
    PartialOrd,
    Hash,
    Eq,
    Ord,
    Debug,
    Default,
    ::serde::Serialize,
    ::serde::Deserialize,
    Clone,
    PartialEq,
)]
pub struct Pong {
    pub msg: ::std::string::String,
}
impl ::pilota::prost::Message for Pong {
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + ::pilota::prost::encoding::string::encoded_len(1, &self.msg)
    }

    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::pilota::prost::bytes::BufMut,
    {
        ::pilota::prost::encoding::string::encode(1, &self.msg, buf);
    }

    #[allow(unused_variables)]
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: ::pilota::prost::encoding::WireType,
        buf: &mut B,
        ctx: ::pilota::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::pilota::prost::DecodeError>
    where
        B: ::pilota::prost::bytes::Buf,
    {
        const STRUCT_NAME: &'static str = stringify!(Pong);
        match tag {
            1 => {
                let mut _inner_pilota_value = &mut self.msg;
                ::pilota::prost::encoding::string::merge(wire_type, _inner_pilota_value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(msg));
                        error
                    })
            }
            _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
}

pub(super) trait GoFfi {
    unsafe fn echo_go_set_result(go_ret: ::fcplug::RustFfiArg<Pong>) -> ::fcplug::GoFfiResult;
}

pub trait GoFfiCall {
    unsafe fn echo_go<T: Default>(mut req: ::fcplug::TBytes<Ping>) -> ::fcplug::ABIResult<T> {
        ::fcplug::ABIResult::from(goffi_echo_go(::fcplug::Buffer::from_vec_mut(
            &mut req.bytes,
        )))
    }
}

#[link(name = "go_echo_pb", kind = "static")]
extern "C" {
    fn goffi_echo_go(req: ::fcplug::Buffer) -> ::fcplug::GoFfiResult;
}
#[no_mangle]
#[inline]
pub extern "C" fn goffi_echo_go_set_result(buf: ::fcplug::Buffer) -> ::fcplug::GoFfiResult {
    unsafe { <FfiImpl as GoFfi>::echo_go_set_result(::fcplug::RustFfiArg::from(buf)) }
}
trait Ffi: RustFfi + GoFfi + GoFfiCall {}

pub struct FfiImpl;

impl GoFfiCall for FfiImpl {}
impl Ffi for FfiImpl {}
