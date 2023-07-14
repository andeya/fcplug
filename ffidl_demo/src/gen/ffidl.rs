#![allow(warnings, clippy::all)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GetUserResponse {
    pub users: ::std::vec::Vec<User>,

    pub resp: ::std::boxed::Box<GetUserResponse>,

    pub resp_map:
        ::std::option::Option<::std::collections::HashMap<::std::string::String, GetUserResponse>>,

    pub req: GetUserRequest,
}
#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct C_GetUserResponse {
    pub users: ::fcplug::ctypes::C_DynArray<C_User>,

    pub resp: *mut C_GetUserResponse,

    pub resp_map: *mut ::fcplug::ctypes::C_Map<::fcplug::ctypes::C_String, C_GetUserResponse>,

    pub req: C_GetUserRequest,
}
impl ::fcplug::ctypes::ConvReprC for GetUserResponse {
    type ReprC = C_GetUserResponse;
    #[inline(always)]
    fn into_repr_c(self) -> Self::ReprC {
        let GetUserResponse {
            users,
            resp,
            resp_map,
            req,
        } = self;
        let users = users.into_repr_c();
        let resp = resp.into_repr_c();
        let resp_map = if let Some(resp_map) = resp_map {
            ::std::boxed::Box::into_raw(::std::boxed::Box::new(resp_map.into_repr_c()))
        } else {
            ::std::ptr::null_mut()
        };
        let req = req.into_repr_c();
        C_GetUserResponse {
            users,
            resp,
            resp_map,
            req,
        }
    }
    #[inline(always)]
    fn from_repr_c(c: Self::ReprC) -> Self {
        let C_GetUserResponse {
            users,
            resp,
            resp_map,
            req,
        } = c;
        let users = ::fcplug::ctypes::ConvReprC::from_repr_c(users);
        let resp = ::fcplug::ctypes::ConvReprC::from_repr_c(resp);
        let resp_map = if resp_map.is_null() {
            ::std::option::Option::None
        } else {
            ::std::option::Option::Some(::fcplug::ctypes::ConvReprC::from_repr_c(unsafe {
                *::std::boxed::Box::from_raw(resp_map)
            }))
        };
        let req = ::fcplug::ctypes::ConvReprC::from_repr_c(req);
        GetUserResponse {
            users,
            resp,
            resp_map,
            req,
        }
    }
}
#[derive(PartialOrd, Hash, Eq, Ord, Debug, Default)]
#[repr(C)]
#[derive(Clone, PartialEq)]
pub struct C {
    pub user_id: i32,

    pub is_male: bool,
}
pub type C_C = C;
pub trait RustFfi {
    fn get_user(req: &GetUserRequest, shuffle: &bool) -> GetUserResponse;
    fn get_user2() -> GetUserResponse;
    fn test4(shuffle: &bool) -> i8;
    fn test5(shuffle: &bool) -> B;
}
#[no_mangle]
#[inline]
pub extern "C" fn rustffi_get_user(req: C_GetUserRequest, shuffle: bool) -> *mut C_GetUserResponse {
    ::std::boxed::Box::into_raw(::std::boxed::Box::new(
        <GetUserResponse as ::fcplug::ctypes::ConvReprC>::into_repr_c(
            <crate::gen::MyImplRustFfi as RustFfi>::get_user(
                &<GetUserRequest as ::fcplug::ctypes::ConvReprC>::from_repr_c(req),
                &<bool as ::fcplug::ctypes::ConvReprC>::from_repr_c(shuffle),
            ),
        ),
    ))
}
#[no_mangle]
#[inline]
pub extern "C" fn rustffi_get_user_free_ret(ret_ptr: *mut C_GetUserResponse) {
    if !ret_ptr.is_null() {
        let _ = <GetUserResponse as ::fcplug::ctypes::ConvReprC>::from_repr_c(unsafe {
            *::std::boxed::Box::from_raw(ret_ptr)
        });
    }
}

#[no_mangle]
#[inline]
pub extern "C" fn rustffi_get_user2() -> *mut C_GetUserResponse {
    ::std::boxed::Box::into_raw(::std::boxed::Box::new(
        <GetUserResponse as ::fcplug::ctypes::ConvReprC>::into_repr_c(
            <crate::gen::MyImplRustFfi as RustFfi>::get_user2(),
        ),
    ))
}
#[no_mangle]
#[inline]
pub extern "C" fn rustffi_get_user2_free_ret(ret_ptr: *mut C_GetUserResponse) {
    if !ret_ptr.is_null() {
        let _ = <GetUserResponse as ::fcplug::ctypes::ConvReprC>::from_repr_c(unsafe {
            *::std::boxed::Box::from_raw(ret_ptr)
        });
    }
}

#[no_mangle]
#[inline]
pub extern "C" fn rustffi_test4(shuffle: bool) -> i8 {
    <crate::gen::MyImplRustFfi as RustFfi>::test4(
        &<bool as ::fcplug::ctypes::ConvReprC>::from_repr_c(shuffle),
    )
}

#[no_mangle]
#[inline]
pub extern "C" fn rustffi_test5(shuffle: bool) -> C_B {
    <crate::gen::MyImplRustFfi as RustFfi>::test5(
        &<bool as ::fcplug::ctypes::ConvReprC>::from_repr_c(shuffle),
    )
}
#[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
pub struct GetUserRequest {
    pub user_id: i32,

    pub user_name: ::std::string::String,

    pub is_male: bool,
}
#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct C_GetUserRequest {
    pub user_id: i32,

    pub user_name: ::fcplug::ctypes::C_String,

    pub is_male: bool,
}
impl ::fcplug::ctypes::ConvReprC for GetUserRequest {
    type ReprC = C_GetUserRequest;
    #[inline(always)]
    fn into_repr_c(self) -> Self::ReprC {
        let GetUserRequest {
            user_id,
            user_name,
            is_male,
        } = self;
        let user_name = user_name.into_repr_c();

        C_GetUserRequest {
            user_id,
            user_name,
            is_male,
        }
    }
    #[inline(always)]
    fn from_repr_c(c: Self::ReprC) -> Self {
        let C_GetUserRequest {
            user_id,
            user_name,
            is_male,
        } = c;
        let user_name = ::fcplug::ctypes::ConvReprC::from_repr_c(user_name);

        GetUserRequest {
            user_id,
            user_name,
            is_male,
        }
    }
}
#[derive(PartialOrd, Hash, Eq, Ord, Debug, Default)]
#[repr(C)]
#[derive(Clone, PartialEq)]
pub struct B {
    pub user_id: i32,

    pub is_male: bool,

    pub c: C,
}
pub type C_B = B;
pub trait GoFfi {
    unsafe fn get_user(
        req: GetUserRequest,
        shuffle: bool,
    ) -> ::fcplug::ctypes::GoFfiResult<GetUserResponse, GetUserRequest>;
    unsafe fn get_user2(
        req: GetUserRequest,
    ) -> ::fcplug::ctypes::GoFfiResult<GetUserResponse, GetUserRequest>;
    unsafe fn get_user3(shuffle: bool) -> ::fcplug::ctypes::GoFfiResult<GetUserResponse, ()>;
    unsafe fn test4(shuffle: bool) -> i8;
    unsafe fn test5(shuffle: bool) -> B;
}
extern "C" {
    fn goffi_get_user(req: *mut C_GetUserRequest, shuffle: bool) -> *mut C_GetUserResponse;
    fn goffi_get_user_free_ret(ret_ptr: usize);

    fn goffi_get_user2(req: *mut C_GetUserRequest) -> *mut C_GetUserResponse;
    fn goffi_get_user2_free_ret(ret_ptr: usize);

    fn goffi_get_user3(shuffle: bool) -> *mut C_GetUserResponse;
    fn goffi_get_user3_free_ret(ret_ptr: usize);

    fn goffi_test4(shuffle: bool) -> i8;
    fn goffi_test5(shuffle: bool) -> C_B;
}
pub struct ImplGoFfi;
impl GoFfi for ImplGoFfi {
    unsafe fn get_user(
        req: GetUserRequest,
        shuffle: bool,
    ) -> ::fcplug::ctypes::GoFfiResult<GetUserResponse, GetUserRequest> {
        let req = ::std::boxed::Box::into_raw(::std::boxed::Box::new(
            ::fcplug::ctypes::ConvReprC::into_repr_c(req),
        ));
        let c_ret__ = goffi_get_user(req, shuffle);
        let ret__ = <GetUserResponse as ::fcplug::ctypes::ConvReprC>::from_repr_c(
            *::std::boxed::Box::from_raw(c_ret__),
        );
        ::fcplug::ctypes::GoFfiResult::new(ret__, req, c_ret__ as usize, goffi_get_user_free_ret)
    }
    unsafe fn get_user2(
        req: GetUserRequest,
    ) -> ::fcplug::ctypes::GoFfiResult<GetUserResponse, GetUserRequest> {
        let req = ::std::boxed::Box::into_raw(::std::boxed::Box::new(
            ::fcplug::ctypes::ConvReprC::into_repr_c(req),
        ));
        let c_ret__ = goffi_get_user2(req);
        let ret__ = <GetUserResponse as ::fcplug::ctypes::ConvReprC>::from_repr_c(
            *::std::boxed::Box::from_raw(c_ret__),
        );
        ::fcplug::ctypes::GoFfiResult::new(ret__, req, c_ret__ as usize, goffi_get_user2_free_ret)
    }
    unsafe fn get_user3(shuffle: bool) -> ::fcplug::ctypes::GoFfiResult<GetUserResponse, ()> {
        let c_ret__ = goffi_get_user3(shuffle);
        let ret__ = <GetUserResponse as ::fcplug::ctypes::ConvReprC>::from_repr_c(
            *::std::boxed::Box::from_raw(c_ret__),
        );
        ::fcplug::ctypes::GoFfiResult::new(
            ret__,
            ::std::ptr::null_mut::<()>(),
            c_ret__ as usize,
            goffi_get_user3_free_ret,
        )
    }
    unsafe fn test4(shuffle: bool) -> i8 {
        let ret__ = goffi_test4(shuffle);

        ret__
    }
    unsafe fn test5(shuffle: bool) -> B {
        let ret__ = goffi_test5(shuffle);

        ret__
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct User {
    pub user_id: i32,

    pub user_name: ::std::string::String,

    pub is_male: bool,

    pub r#pure: B,

    pub extra: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct C_User {
    pub user_id: i32,

    pub user_name: ::fcplug::ctypes::C_String,

    pub is_male: bool,

    pub r#pure: C_B,

    pub extra: *mut ::fcplug::ctypes::C_Map<::fcplug::ctypes::C_String, ::fcplug::ctypes::C_String>,
}
impl ::fcplug::ctypes::ConvReprC for User {
    type ReprC = C_User;
    #[inline(always)]
    fn into_repr_c(self) -> Self::ReprC {
        let User {
            user_id,
            user_name,
            is_male,
            r#pure,
            extra,
        } = self;
        let user_name = user_name.into_repr_c();

        let extra = if let Some(extra) = extra {
            ::std::boxed::Box::into_raw(::std::boxed::Box::new(extra.into_repr_c()))
        } else {
            ::std::ptr::null_mut()
        };
        C_User {
            user_id,
            user_name,
            is_male,
            r#pure,
            extra,
        }
    }
    #[inline(always)]
    fn from_repr_c(c: Self::ReprC) -> Self {
        let C_User {
            user_id,
            user_name,
            is_male,
            r#pure,
            extra,
        } = c;
        let user_name = ::fcplug::ctypes::ConvReprC::from_repr_c(user_name);

        let extra = if extra.is_null() {
            ::std::option::Option::None
        } else {
            ::std::option::Option::Some(::fcplug::ctypes::ConvReprC::from_repr_c(unsafe {
                *::std::boxed::Box::from_raw(extra)
            }))
        };
        User {
            user_id,
            user_name,
            is_male,
            r#pure,
            extra,
        }
    }
}
