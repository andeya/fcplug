// Code generated by fcplug. DO NOT EDIT.
#![allow(warnings, clippy::all)]
#[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
#[repr(C)]
#[derive(Clone, PartialEq)]
pub struct A {
    pub number: i32,
}
#[::async_trait::async_trait]
impl ::pilota::thrift::Message for A {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };

        protocol.write_struct_begin(&struct_ident)?;
        protocol.write_i32_field(1, *&self.number)?;
        protocol.write_field_stop()?;
        protocol.write_struct_end()?;
        Ok(())
    }

    fn decode<T: ::pilota::thrift::TInputProtocol>(
        protocol: &mut T,
    ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
        let mut number = None;

        let mut __pilota_decoding_field_id = None;

        protocol.read_struct_begin()?;
        if let Err(err) = (|| {
            loop {
                let field_ident = protocol.read_field_begin()?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                    break;
                }
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        number = Some(protocol.read_i32()?);
                    }
                    _ => {
                        protocol.skip(field_ident.field_type)?;
                    }
                }

                protocol.read_field_end()?;
            }
            Ok::<_, ::pilota::thrift::DecodeError>(())
        })() {
            if let Some(field_id) = __pilota_decoding_field_id {
                return Err(::pilota::thrift::DecodeError::new(
                    ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                    format!("decode struct `A` field(#{}) failed", field_id),
                ));
            } else {
                return Err(err);
            }
        };
        protocol.read_struct_end()?;

        let Some(number) = number else {
            return Err(::pilota::thrift::DecodeError::new(
                ::pilota::thrift::DecodeErrorKind::InvalidData,
                "field number is required".to_string(),
            ));
        };

        let data = Self { number };
        Ok(data)
    }

    async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
        protocol: &mut T,
    ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
        let mut number = None;

        let mut __pilota_decoding_field_id = None;

        protocol.read_struct_begin().await?;
        if let Err(err) = async {
            loop {
                let field_ident = protocol.read_field_begin().await?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                    break;
                }
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        number = Some(protocol.read_i32().await?);
                    }
                    _ => {
                        protocol.skip(field_ident.field_type).await?;
                    }
                }

                protocol.read_field_end().await?;
            }
            Ok::<_, ::pilota::thrift::DecodeError>(())
        }
        .await
        {
            if let Some(field_id) = __pilota_decoding_field_id {
                return Err(::pilota::thrift::DecodeError::new(
                    ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                    format!("decode struct `A` field(#{}) failed", field_id),
                ));
            } else {
                return Err(err);
            }
        };
        protocol.read_struct_end().await?;

        let Some(number) = number else {
            return Err(::pilota::thrift::DecodeError::new(
                ::pilota::thrift::DecodeErrorKind::InvalidData,
                "field number is required".to_string(),
            ));
        };

        let data = Self { number };
        Ok(data)
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
            + protocol.write_i32_field_len(Some(1), *&self.number)
            + protocol.write_field_stop_len()
            + protocol.write_struct_end_len()
    }
}
#[derive(Debug, Default, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct Pong {
    pub msg: ::std::string::String,

    pub number_map: ::std::collections::HashMap<i16, A>,
}
#[::async_trait::async_trait]
impl ::pilota::thrift::Message for Pong {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Pong" };

        protocol.write_struct_begin(&struct_ident)?;
        protocol.write_string_field(1, &&self.msg)?;
        protocol.write_map_field(
            2,
            ::pilota::thrift::TType::I16,
            ::pilota::thrift::TType::Struct,
            &&self.number_map,
            |protocol, key| {
                protocol.write_i16(*key)?;
                Ok(())
            },
            |protocol, val| {
                protocol.write_struct(val)?;
                Ok(())
            },
        )?;
        protocol.write_field_stop()?;
        protocol.write_struct_end()?;
        Ok(())
    }

    fn decode<T: ::pilota::thrift::TInputProtocol>(
        protocol: &mut T,
    ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
        let mut msg = None;
        let mut number_map = None;

        let mut __pilota_decoding_field_id = None;

        protocol.read_struct_begin()?;
        if let Err(err) = (|| {
            loop {
                let field_ident = protocol.read_field_begin()?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                    break;
                }
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        msg = Some(protocol.read_string()?);
                    }
                    Some(2) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                        number_map = Some({
                            let map_ident = protocol.read_map_begin()?;
                            let mut val =
                                ::std::collections::HashMap::with_capacity(map_ident.size);
                            for _ in 0..map_ident.size {
                                val.insert(
                                    protocol.read_i16()?,
                                    ::pilota::thrift::Message::decode(protocol)?,
                                );
                            }
                            protocol.read_map_end()?;
                            val
                        });
                    }
                    _ => {
                        protocol.skip(field_ident.field_type)?;
                    }
                }

                protocol.read_field_end()?;
            }
            Ok::<_, ::pilota::thrift::DecodeError>(())
        })() {
            if let Some(field_id) = __pilota_decoding_field_id {
                return Err(::pilota::thrift::DecodeError::new(
                    ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                    format!("decode struct `Pong` field(#{}) failed", field_id),
                ));
            } else {
                return Err(err);
            }
        };
        protocol.read_struct_end()?;

        let Some(msg) = msg else {
            return Err(::pilota::thrift::DecodeError::new(
                ::pilota::thrift::DecodeErrorKind::InvalidData,
                "field msg is required".to_string(),
            ));
        };
        let Some(number_map) = number_map else {
            return Err(::pilota::thrift::DecodeError::new(
                ::pilota::thrift::DecodeErrorKind::InvalidData,
                "field number_map is required".to_string(),
            ));
        };

        let data = Self { msg, number_map };
        Ok(data)
    }

    async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
        protocol: &mut T,
    ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
        let mut msg = None;
        let mut number_map = None;

        let mut __pilota_decoding_field_id = None;

        protocol.read_struct_begin().await?;
        if let Err(err) = async {
            loop {
                let field_ident = protocol.read_field_begin().await?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                    break;
                }
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        msg = Some(protocol.read_string().await?);
                    }
                    Some(2) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                        number_map = Some({
                            let map_ident = protocol.read_map_begin().await?;
                            let mut val =
                                ::std::collections::HashMap::with_capacity(map_ident.size);
                            for _ in 0..map_ident.size {
                                val.insert(
                                    protocol.read_i16().await?,
                                    ::pilota::thrift::Message::decode_async(protocol).await?,
                                );
                            }
                            protocol.read_map_end().await?;
                            val
                        });
                    }
                    _ => {
                        protocol.skip(field_ident.field_type).await?;
                    }
                }

                protocol.read_field_end().await?;
            }
            Ok::<_, ::pilota::thrift::DecodeError>(())
        }
        .await
        {
            if let Some(field_id) = __pilota_decoding_field_id {
                return Err(::pilota::thrift::DecodeError::new(
                    ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                    format!("decode struct `Pong` field(#{}) failed", field_id),
                ));
            } else {
                return Err(err);
            }
        };
        protocol.read_struct_end().await?;

        let Some(msg) = msg else {
            return Err(::pilota::thrift::DecodeError::new(
                ::pilota::thrift::DecodeErrorKind::InvalidData,
                "field msg is required".to_string(),
            ));
        };
        let Some(number_map) = number_map else {
            return Err(::pilota::thrift::DecodeError::new(
                ::pilota::thrift::DecodeErrorKind::InvalidData,
                "field number_map is required".to_string(),
            ));
        };

        let data = Self { msg, number_map };
        Ok(data)
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Pong" })
            + protocol.write_string_field_len(Some(1), &&self.msg)
            + protocol.write_map_field_len(
                Some(2),
                ::pilota::thrift::TType::I16,
                ::pilota::thrift::TType::Struct,
                &self.number_map,
                |protocol, key| protocol.write_i16_len(*key),
                |protocol, val| protocol.write_struct_len(val),
            )
            + protocol.write_field_stop_len()
            + protocol.write_struct_end_len()
    }
}
pub trait GoFfiCall {
    unsafe fn echo_go<T: Default>(req: ::fcplug::TBytes<Ping>) -> ::fcplug::ABIResult<T> {
        ::fcplug::ABIResult::from(goffi_echo_go(::fcplug::Buffer::from_vec(req.bytes)))
    }
}

pub(super) trait GoFfi {
    unsafe fn echo_go_set_result(go_ret: ::fcplug::RustFfiArg<Pong>) -> ::fcplug::GoFfiResult;
}
extern "C" {
    fn goffi_echo_go(req: ::fcplug::Buffer) -> ::fcplug::GoFfiResult;
}
#[no_mangle]
#[inline]
pub extern "C" fn goffi_echo_go_set_result(buf: ::fcplug::Buffer) -> ::fcplug::GoFfiResult {
    unsafe { <FfiImpl as GoFfi>::echo_go_set_result(::fcplug::RustFfiArg::from(buf)) }
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
pub struct Ping {
    pub msg: ::std::string::String,

    pub number_list: ::std::vec::Vec<A>,

    pub number_set: ::std::vec::Vec<A>,
}
#[::async_trait::async_trait]
impl ::pilota::thrift::Message for Ping {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Ping" };

        protocol.write_struct_begin(&struct_ident)?;
        protocol.write_string_field(1, &&self.msg)?;
        protocol.write_list_field(
            2,
            ::pilota::thrift::TType::Struct,
            &&self.number_list,
            |protocol, val| {
                protocol.write_struct(val)?;
                Ok(())
            },
        )?;
        protocol.write_list_field(
            3,
            ::pilota::thrift::TType::Struct,
            &&self.number_set,
            |protocol, val| {
                protocol.write_struct(val)?;
                Ok(())
            },
        )?;
        protocol.write_field_stop()?;
        protocol.write_struct_end()?;
        Ok(())
    }

    fn decode<T: ::pilota::thrift::TInputProtocol>(
        protocol: &mut T,
    ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
        let mut msg = None;
        let mut number_list = None;
        let mut number_set = None;

        let mut __pilota_decoding_field_id = None;

        protocol.read_struct_begin()?;
        if let Err(err) = (|| {
            loop {
                let field_ident = protocol.read_field_begin()?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                    break;
                }
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        msg = Some(protocol.read_string()?);
                    }
                    Some(2) if field_ident.field_type == ::pilota::thrift::TType::List => {
                        number_list = Some(unsafe {
                            let list_ident = protocol.read_list_begin()?;
                            let mut val: Vec<A> = Vec::with_capacity(list_ident.size);
                            for i in 0..list_ident.size {
                                val.as_mut_ptr()
                                    .offset(i as isize)
                                    .write(::pilota::thrift::Message::decode(protocol)?);
                            }
                            val.set_len(list_ident.size);
                            protocol.read_list_end()?;
                            val
                        });
                    }
                    Some(3) if field_ident.field_type == ::pilota::thrift::TType::List => {
                        number_set = Some(unsafe {
                            let list_ident = protocol.read_list_begin()?;
                            let mut val: Vec<A> = Vec::with_capacity(list_ident.size);
                            for i in 0..list_ident.size {
                                val.as_mut_ptr()
                                    .offset(i as isize)
                                    .write(::pilota::thrift::Message::decode(protocol)?);
                            }
                            val.set_len(list_ident.size);
                            protocol.read_list_end()?;
                            val
                        });
                    }
                    _ => {
                        protocol.skip(field_ident.field_type)?;
                    }
                }

                protocol.read_field_end()?;
            }
            Ok::<_, ::pilota::thrift::DecodeError>(())
        })() {
            if let Some(field_id) = __pilota_decoding_field_id {
                return Err(::pilota::thrift::DecodeError::new(
                    ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                    format!("decode struct `Ping` field(#{}) failed", field_id),
                ));
            } else {
                return Err(err);
            }
        };
        protocol.read_struct_end()?;

        let Some(msg) = msg else {
            return Err(::pilota::thrift::DecodeError::new(
                ::pilota::thrift::DecodeErrorKind::InvalidData,
                "field msg is required".to_string(),
            ));
        };
        let Some(number_list) = number_list else {
            return Err(::pilota::thrift::DecodeError::new(
                ::pilota::thrift::DecodeErrorKind::InvalidData,
                "field number_list is required".to_string(),
            ));
        };
        let Some(number_set) = number_set else {
            return Err(::pilota::thrift::DecodeError::new(
                ::pilota::thrift::DecodeErrorKind::InvalidData,
                "field number_set is required".to_string(),
            ));
        };

        let data = Self {
            msg,
            number_list,
            number_set,
        };
        Ok(data)
    }

    async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
        protocol: &mut T,
    ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
        let mut msg = None;
        let mut number_list = None;
        let mut number_set = None;

        let mut __pilota_decoding_field_id = None;

        protocol.read_struct_begin().await?;
        if let Err(err) = async {
            loop {
                let field_ident = protocol.read_field_begin().await?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                    break;
                }
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        msg = Some(protocol.read_string().await?);
                    }
                    Some(2) if field_ident.field_type == ::pilota::thrift::TType::List => {
                        number_list = Some({
                            let list_ident = protocol.read_list_begin().await?;
                            let mut val = Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(::pilota::thrift::Message::decode_async(protocol).await?);
                            }
                            protocol.read_list_end().await?;
                            val
                        });
                    }
                    Some(3) if field_ident.field_type == ::pilota::thrift::TType::List => {
                        number_set = Some({
                            let list_ident = protocol.read_list_begin().await?;
                            let mut val = Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(::pilota::thrift::Message::decode_async(protocol).await?);
                            }
                            protocol.read_list_end().await?;
                            val
                        });
                    }
                    _ => {
                        protocol.skip(field_ident.field_type).await?;
                    }
                }

                protocol.read_field_end().await?;
            }
            Ok::<_, ::pilota::thrift::DecodeError>(())
        }
        .await
        {
            if let Some(field_id) = __pilota_decoding_field_id {
                return Err(::pilota::thrift::DecodeError::new(
                    ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                    format!("decode struct `Ping` field(#{}) failed", field_id),
                ));
            } else {
                return Err(err);
            }
        };
        protocol.read_struct_end().await?;

        let Some(msg) = msg else {
            return Err(::pilota::thrift::DecodeError::new(
                ::pilota::thrift::DecodeErrorKind::InvalidData,
                "field msg is required".to_string(),
            ));
        };
        let Some(number_list) = number_list else {
            return Err(::pilota::thrift::DecodeError::new(
                ::pilota::thrift::DecodeErrorKind::InvalidData,
                "field number_list is required".to_string(),
            ));
        };
        let Some(number_set) = number_set else {
            return Err(::pilota::thrift::DecodeError::new(
                ::pilota::thrift::DecodeErrorKind::InvalidData,
                "field number_set is required".to_string(),
            ));
        };

        let data = Self {
            msg,
            number_list,
            number_set,
        };
        Ok(data)
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Ping" })
            + protocol.write_string_field_len(Some(1), &&self.msg)
            + protocol.write_list_field_len(
                Some(2),
                ::pilota::thrift::TType::Struct,
                &self.number_list,
                |protocol, el| protocol.write_struct_len(el),
            )
            + protocol.write_list_field_len(
                Some(3),
                ::pilota::thrift::TType::Struct,
                &self.number_set,
                |protocol, el| protocol.write_struct_len(el),
            )
            + protocol.write_field_stop_len()
            + protocol.write_struct_end_len()
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
trait Ffi: RustFfi + GoFfi + GoFfiCall {}

pub struct FfiImpl;

impl GoFfiCall for FfiImpl {}
impl Ffi for FfiImpl {}