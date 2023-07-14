use crate::gen::ffidl::{B, C_GetUserRequest, C_GetUserResponse, GetUserRequest, GetUserResponse};

mod ffidl;

#[no_mangle]
pub extern "C" fn ffidl_types(a: C_GetUserRequest, b: bool) -> C_GetUserResponse {
    unimplemented!();
}

struct MyImplRustFfi;
impl ffidl::RustFfi for MyImplRustFfi {
    fn get_user(req: &GetUserRequest, shuffle: &bool) -> GetUserResponse {
        todo!()
    }

    fn get_user2() -> GetUserResponse {
        todo!()
    }

    fn test4(shuffle: &bool) -> i8 {
        todo!()
    }

    fn test5(shuffle: &bool) -> B {
        todo!()
    }
}
