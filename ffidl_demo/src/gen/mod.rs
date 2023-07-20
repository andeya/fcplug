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
pub struct SearchRequest {
    pub query: ::std::string::String,

    pub page_number: i32,

    pub result_per_page: i32,
}
impl ::pilota::prost::Message for SearchRequest {
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + ::pilota::prost::encoding::string::encoded_len(1, &self.query)
            + ::pilota::prost::encoding::int32::encoded_len(2, &self.page_number)
            + ::pilota::prost::encoding::int32::encoded_len(3, &self.result_per_page)
    }

    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::pilota::prost::bytes::BufMut,
    {
        ::pilota::prost::encoding::string::encode(1, &self.query, buf);
        ::pilota::prost::encoding::int32::encode(2, &self.page_number, buf);
        ::pilota::prost::encoding::int32::encode(3, &self.result_per_page, buf);
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
        const STRUCT_NAME: &'static str = stringify!(SearchRequest);
        match tag {
            1 => {
                let mut _inner_pilota_value = &mut self.query;
                ::pilota::prost::encoding::string::merge(wire_type, _inner_pilota_value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(query));
                        error
                    })
            }
            2 => {
                let mut _inner_pilota_value = &mut self.page_number;
                ::pilota::prost::encoding::int32::merge(wire_type, _inner_pilota_value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(page_number));
                        error
                    })
            }
            3 => {
                let mut _inner_pilota_value = &mut self.result_per_page;
                ::pilota::prost::encoding::int32::merge(wire_type, _inner_pilota_value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(result_per_page));
                        error
                    })
            }
            _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
}

pub trait GoFfi {
    unsafe fn search_client<T: Default>(
        req: ::fcplug::TBytes<SearchRequest>,
    ) -> ::fcplug::ABIResult<T> {
        ::fcplug::ABIResult::from(goffi_search_client(::fcplug::Buffer::from_vec(req.bytes)))
    }
    unsafe fn search_client_set_result(
        go_ret: ::fcplug::RustFfiArg<Client>,
    ) -> ::fcplug::GoFfiResult;
}
extern "C" {
    fn goffi_search_client(req: ::fcplug::Buffer) -> ::fcplug::GoFfiResult;
}
#[no_mangle]
#[inline]
pub extern "C" fn goffi_search_client_set_result(buf: ::fcplug::Buffer) -> ::fcplug::GoFfiResult {
    unsafe { <crate::Test as GoFfi>::search_client_set_result(::fcplug::RustFfiArg::from(buf)) }
}
#[derive(Debug, Default, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct WebSite {
    pub name: ::std::string::String,

    pub link: ::std::string::String,

    pub age: i32,

    pub server: ::std::collections::HashMap<::std::string::String, Server>,

    pub a: ::std::vec::Vec<::std::string::String>,

    pub b: ::std::vec::Vec<u8>,
}
impl ::pilota::prost::Message for WebSite {
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + ::pilota::prost::encoding::string::encoded_len(1, &self.name)
            + ::pilota::prost::encoding::string::encoded_len(2, &self.link)
            + ::pilota::prost::encoding::int32::encoded_len(3, &self.age)
            + ::pilota::prost::encoding::hash_map::encoded_len(
                ::pilota::prost::encoding::string::encoded_len,
                ::pilota::prost::encoding::message::encoded_len,
                4,
                &self.server,
            )
            + ::pilota::prost::encoding::string::encoded_len_repeated(5, &self.a)
            + ::pilota::prost::encoding::bytes::encoded_len(6, &self.b)
    }

    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::pilota::prost::bytes::BufMut,
    {
        ::pilota::prost::encoding::string::encode(1, &self.name, buf);
        ::pilota::prost::encoding::string::encode(2, &self.link, buf);
        ::pilota::prost::encoding::int32::encode(3, &self.age, buf);
        ::pilota::prost::encoding::hash_map::encode(
            ::pilota::prost::encoding::string::encode,
            ::pilota::prost::encoding::string::encoded_len,
            ::pilota::prost::encoding::message::encode,
            ::pilota::prost::encoding::message::encoded_len,
            4,
            &self.server,
            buf,
        );
        ::pilota::prost::encoding::string::encode_repeated(5, &self.a, buf);
        ::pilota::prost::encoding::bytes::encode(6, &self.b, buf);
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
        const STRUCT_NAME: &'static str = stringify!(WebSite);
        match tag {
            1 => {
                let mut _inner_pilota_value = &mut self.name;
                ::pilota::prost::encoding::string::merge(wire_type, _inner_pilota_value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(name));
                        error
                    })
            }
            2 => {
                let mut _inner_pilota_value = &mut self.link;
                ::pilota::prost::encoding::string::merge(wire_type, _inner_pilota_value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(link));
                        error
                    })
            }
            3 => {
                let mut _inner_pilota_value = &mut self.age;
                ::pilota::prost::encoding::int32::merge(wire_type, _inner_pilota_value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(age));
                        error
                    })
            }
            4 => {
                let mut _inner_pilota_value = &mut self.server;
                ::pilota::prost::encoding::hash_map::merge(
                    ::pilota::prost::encoding::string::merge,
                    ::pilota::prost::encoding::message::merge,
                    &mut _inner_pilota_value,
                    buf,
                    ctx,
                )
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, stringify!(server));
                    error
                })
            }
            5 => {
                let mut _inner_pilota_value = &mut self.a;
                ::pilota::prost::encoding::string::merge_repeated(
                    wire_type,
                    _inner_pilota_value,
                    buf,
                    ctx,
                )
                .map_err(|mut error| {
                    error.push(STRUCT_NAME, stringify!(a));
                    error
                })
            }
            6 => {
                let mut _inner_pilota_value = &mut self.b;
                ::pilota::prost::encoding::bytes::merge(wire_type, _inner_pilota_value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(b));
                        error
                    })
            }
            _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
}

pub trait RustFfi {
    fn search_web_site(
        req: ::fcplug::RustFfiArg<SearchRequest>,
    ) -> ::fcplug::ABIResult<::fcplug::TBytes<WebSite>>;
}
#[no_mangle]
#[inline]
pub extern "C" fn rustffi_search_web_site(req: ::fcplug::Buffer) -> ::fcplug::RustFfiResult {
    ::fcplug::RustFfiResult::from(<crate::Test as RustFfi>::search_web_site(
        ::fcplug::RustFfiArg::from(req),
    ))
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
pub struct Server {
    pub hostname: ::std::string::String,

    pub port: i32,
}
impl ::pilota::prost::Message for Server {
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + ::pilota::prost::encoding::string::encoded_len(1, &self.hostname)
            + ::pilota::prost::encoding::int32::encoded_len(2, &self.port)
    }

    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::pilota::prost::bytes::BufMut,
    {
        ::pilota::prost::encoding::string::encode(1, &self.hostname, buf);
        ::pilota::prost::encoding::int32::encode(2, &self.port, buf);
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
        const STRUCT_NAME: &'static str = stringify!(Server);
        match tag {
            1 => {
                let mut _inner_pilota_value = &mut self.hostname;
                ::pilota::prost::encoding::string::merge(wire_type, _inner_pilota_value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(hostname));
                        error
                    })
            }
            2 => {
                let mut _inner_pilota_value = &mut self.port;
                ::pilota::prost::encoding::int32::merge(wire_type, _inner_pilota_value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(port));
                        error
                    })
            }
            _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
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
pub struct Client {
    pub ip: ::std::string::String,

    pub city: ::std::string::String,
}
impl ::pilota::prost::Message for Client {
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + ::pilota::prost::encoding::string::encoded_len(1, &self.ip)
            + ::pilota::prost::encoding::string::encoded_len(2, &self.city)
    }

    #[allow(unused_variables)]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: ::pilota::prost::bytes::BufMut,
    {
        ::pilota::prost::encoding::string::encode(1, &self.ip, buf);
        ::pilota::prost::encoding::string::encode(2, &self.city, buf);
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
        const STRUCT_NAME: &'static str = stringify!(Client);
        match tag {
            1 => {
                let mut _inner_pilota_value = &mut self.ip;
                ::pilota::prost::encoding::string::merge(wire_type, _inner_pilota_value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(ip));
                        error
                    })
            }
            2 => {
                let mut _inner_pilota_value = &mut self.city;
                ::pilota::prost::encoding::string::merge(wire_type, _inner_pilota_value, buf, ctx)
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(city));
                        error
                    })
            }
            _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
}
