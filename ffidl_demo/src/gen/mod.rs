#![allow(warnings, clippy::all)]
#[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
pub struct SearchRequest {
    pub query: ::std::string::String,

    pub page_number: i32,

    pub result_per_page: i32,
}
#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct FfiSearchRequest {
    pub query: ::fcplug::ctypes::FfiArray<u8>,

    pub page_number: i32,

    pub result_per_page: i32,
}
impl ::fcplug::ctypes::ConvRepr for SearchRequest {
    type CRepr = FfiSearchRequest;
    #[inline(always)]
    fn into_c_repr(self) -> Self::CRepr {
        let SearchRequest {
            query,
            page_number,
            result_per_page,
        } = self;
        let query = query.into_c_repr();

        FfiSearchRequest {
            query,
            page_number,
            result_per_page,
        }
    }
    #[inline(always)]
    fn from_c_repr(c: Self::CRepr) -> Self {
        let FfiSearchRequest {
            query,
            page_number,
            result_per_page,
        } = c;
        let query = ::fcplug::ctypes::ConvRepr::from_c_repr(query);

        SearchRequest {
            query,
            page_number,
            result_per_page,
        }
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct WebSite {
    pub name: ::std::string::String,

    pub link: ::std::string::String,

    pub age: i32,

    pub server: ::std::collections::HashMap<::std::string::String, Server>,
}
#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct FfiWebSite {
    pub name: ::fcplug::ctypes::FfiArray<u8>,

    pub link: ::fcplug::ctypes::FfiArray<u8>,

    pub age: i32,

    pub server: ::fcplug::ctypes::FfiArray<
        ::fcplug::ctypes::MapEntry<::fcplug::ctypes::FfiArray<u8>, FfiServer>,
    >,
}
impl ::fcplug::ctypes::ConvRepr for WebSite {
    type CRepr = FfiWebSite;
    #[inline(always)]
    fn into_c_repr(self) -> Self::CRepr {
        let WebSite {
            name,
            link,
            age,
            server,
        } = self;
        let name = name.into_c_repr();
        let link = link.into_c_repr();

        let server = server.into_c_repr();
        FfiWebSite {
            name,
            link,
            age,
            server,
        }
    }
    #[inline(always)]
    fn from_c_repr(c: Self::CRepr) -> Self {
        let FfiWebSite {
            name,
            link,
            age,
            server,
        } = c;
        let name = ::fcplug::ctypes::ConvRepr::from_c_repr(name);
        let link = ::fcplug::ctypes::ConvRepr::from_c_repr(link);

        let server = ::fcplug::ctypes::ConvRepr::from_c_repr(server);
        WebSite {
            name,
            link,
            age,
            server,
        }
    }
}
#[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
pub struct Server {
    pub hostname: ::std::string::String,

    pub port: i32,
}
#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct FfiServer {
    pub hostname: ::fcplug::ctypes::FfiArray<u8>,

    pub port: i32,
}
impl ::fcplug::ctypes::ConvRepr for Server {
    type CRepr = FfiServer;
    #[inline(always)]
    fn into_c_repr(self) -> Self::CRepr {
        let Server { hostname, port } = self;
        let hostname = hostname.into_c_repr();

        FfiServer { hostname, port }
    }
    #[inline(always)]
    fn from_c_repr(c: Self::CRepr) -> Self {
        let FfiServer { hostname, port } = c;
        let hostname = ::fcplug::ctypes::ConvRepr::from_c_repr(hostname);

        Server { hostname, port }
    }
}
pub trait RustFfi {
    fn search(req: &SearchRequest) -> WebSite;
}
#[no_mangle]
#[inline]
pub extern "C" fn rustffi_search(req: FfiSearchRequest) -> *mut FfiWebSite {
    ::std::boxed::Box::into_raw(::std::boxed::Box::new(
        <WebSite as ::fcplug::ctypes::ConvRepr>::into_c_repr(<crate::Test as RustFfi>::search(
            &::std::mem::ManuallyDrop::new(
                <SearchRequest as ::fcplug::ctypes::ConvRepr>::from_c_repr(req),
            ),
        )),
    ))
}
#[no_mangle]
#[inline]
pub extern "C" fn rustffi_search_free_ret(ret_ptr: *mut FfiWebSite) {
    if !ret_ptr.is_null() {
        let _ = <WebSite as ::fcplug::ctypes::ConvRepr>::from_c_repr(unsafe {
            *::std::boxed::Box::from_raw(ret_ptr)
        });
    }
}
