#![allow(warnings, clippy::all)]
#[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
pub struct SearchRequest {
    pub query: ::std::string::String,

    pub page_number: i32,

    pub result_per_page: i32,
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct WebSite {
    pub name: ::std::string::String,

    pub link: ::std::string::String,

    pub age: i32,

    pub server: ::std::collections::HashMap<::std::string::String, Server>,
}
#[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
pub struct Server {
    pub hostname: ::std::string::String,

    pub port: i32,
}
pub trait RustFfi {
    fn search(req: &SearchRequest) -> ::fcplug::ABIResult<WebSite>;
}
#[no_mangle]
#[inline]
pub extern "C" fn rustffi_search(mut req: ::fcplug::Buffer) -> ::fcplug::callee::FFIResult {
    ::fcplug::callee::protobuf::callback(
        "rustffi_search",
        <crate::Test as RustFfi>::search,
        &mut req,
    )
}
