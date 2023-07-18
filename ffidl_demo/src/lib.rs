use std::collections::HashMap;
use crate::gen::{A, B, GetUserRequest, GetUserResponse, RustFfi, User};

mod gen;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

struct Test;

impl RustFfi for Test {
    fn get_user(s:&::std::string::String) -> ::std::string::String{
        println!("get_user: shuffle={s}");
        s.to_string()+"这是回复"
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
