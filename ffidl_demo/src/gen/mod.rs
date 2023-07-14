use crate::gen::ffidl::{C_GetUserRequest, C_GetUserResponse};

mod ffidl;

#[no_mangle]
pub extern "C" fn ffidl_types(a: C_GetUserRequest, b: bool) -> C_GetUserResponse {
    unimplemented!();
}
