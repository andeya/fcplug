#![allow(warnings, clippy::all)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct A {
    pub user_id: i32,

    pub user_name: ::std::string::String,

    pub is_male: bool,

    pub extra: ::std::option::Option<::std::collections::HashMap<::std::string::String, B>>,
}
#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct C_A {
    pub user_id: i32,

    pub user_name: ::fcplug::ctypes::C_String,

    pub is_male: bool,

    pub extra: *mut ::fcplug::ctypes::C_Map<::fcplug::ctypes::C_String, C_B>,
}
impl ::fcplug::ctypes::ConvReprC for A {
    type ReprC = C_A;
    #[inline(always)]
    fn into_repr_c(self) -> Self::ReprC {
        let A {
            user_id,
            user_name,
            is_male,
            extra,
        } = self;
        let user_name = user_name.into_repr_c();

        let extra = if let Some(extra) = extra {
            ::std::boxed::Box::into_raw(::std::boxed::Box::new(extra.into_repr_c()))
        } else {
            ::std::ptr::null_mut()
        };
        C_A {
            user_id,
            user_name,
            is_male,
            extra,
        }
    }
    #[inline(always)]
    fn from_repr_c(c: Self::ReprC) -> Self {
        let C_A {
            user_id,
            user_name,
            is_male,
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
        A {
            user_id,
            user_name,
            is_male,
            extra,
        }
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GetUserResponse {
    pub users: ::std::vec::Vec<User>,

    pub resp_map:
        ::std::option::Option<::std::collections::HashMap<::std::string::String, GetUserRequest>>,

    pub req: GetUserRequest,
}
#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct C_GetUserResponse {
    pub users: ::fcplug::ctypes::C_DynArray<C_User>,

    pub resp_map: *mut ::fcplug::ctypes::C_Map<::fcplug::ctypes::C_String, C_GetUserRequest>,

    pub req: C_GetUserRequest,
}
impl ::fcplug::ctypes::ConvReprC for GetUserResponse {
    type ReprC = C_GetUserResponse;
    #[inline(always)]
    fn into_repr_c(self) -> Self::ReprC {
        let GetUserResponse {
            users,
            resp_map,
            req,
        } = self;
        let users = users.into_repr_c();
        let resp_map = if let Some(resp_map) = resp_map {
            ::std::boxed::Box::into_raw(::std::boxed::Box::new(resp_map.into_repr_c()))
        } else {
            ::std::ptr::null_mut()
        };
        let req = req.into_repr_c();
        C_GetUserResponse {
            users,
            resp_map,
            req,
        }
    }
    #[inline(always)]
    fn from_repr_c(c: Self::ReprC) -> Self {
        let C_GetUserResponse {
            users,
            resp_map,
            req,
        } = c;
        let users = ::fcplug::ctypes::ConvReprC::from_repr_c(users);
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
impl ::fcplug::ctypes::ConvReprC for C {
    type ReprC = C_C;
    #[inline(always)]
    fn into_repr_c(self) -> Self::ReprC {
        self
    }
    #[inline(always)]
    fn from_repr_c(c: Self::ReprC) -> Self {
        c
    }
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
impl ::fcplug::ctypes::ConvReprC for B {
    type ReprC = C_B;
    #[inline(always)]
    fn into_repr_c(self) -> Self::ReprC {
        self
    }
    #[inline(always)]
    fn from_repr_c(c: Self::ReprC) -> Self {
        c
    }
}

pub trait RustFfi {
    fn get_user(shuffle: &::std::string::String) -> ::std::string::String;
    fn get_user2() -> GetUserResponse;
    fn test4(shuffle: &bool) -> i8;
    fn test5(shuffle: &bool) -> B;
}
#[no_mangle]
#[inline]
pub extern "C" fn rustffi_get_user(
    shuffle: ::fcplug::ctypes::C_String,
) -> *mut ::fcplug::ctypes::C_String {
    ::std::boxed::Box::into_raw(::std::boxed::Box::new(
        <::std::string::String as ::fcplug::ctypes::ConvReprC>::into_repr_c(
            <crate::Test as RustFfi>::get_user(
                &<::std::string::String as ::fcplug::ctypes::ConvReprC>::from_repr_c(shuffle),
            ),
        ),
    ))
}
#[no_mangle]
#[inline]
pub extern "C" fn rustffi_get_user_free_ret(ret_ptr: *mut ::fcplug::ctypes::C_String) {
    if !ret_ptr.is_null() {
        let _ = <::std::string::String as ::fcplug::ctypes::ConvReprC>::from_repr_c(unsafe {
            *::std::boxed::Box::from_raw(ret_ptr)
        });
    }
}

#[no_mangle]
#[inline]
pub extern "C" fn rustffi_get_user2() -> *mut C_GetUserResponse {
    ::std::boxed::Box::into_raw(::std::boxed::Box::new(
        <GetUserResponse as ::fcplug::ctypes::ConvReprC>::into_repr_c(
            <crate::Test as RustFfi>::get_user2(),
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
    <crate::Test as RustFfi>::test4(&<bool as ::fcplug::ctypes::ConvReprC>::from_repr_c(shuffle))
}

#[no_mangle]
#[inline]
pub extern "C" fn rustffi_test5(shuffle: bool) -> C_B {
    <crate::Test as RustFfi>::test5(&<bool as ::fcplug::ctypes::ConvReprC>::from_repr_c(shuffle))
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct User {
    pub user_id: i32,

    pub user_name: ::std::string::String,

    pub is_male: bool,

    pub r#pure: A,

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

    pub r#pure: C_A,

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

        let r#pure = r#pure.into_repr_c();
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

        let r#pure = ::fcplug::ctypes::ConvReprC::from_repr_c(r#pure);
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
