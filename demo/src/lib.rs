use std::collections::HashMap;

use fcplug::{ABIResult, GoFfiResult, RustFfiArg, TBytes, TryIntoTBytes};
use fcplug::protobuf::PbMessage;

use crate::ffidl_gen::{Client, GoFfi, RustFfi, SearchRequest, Server, WebSite};

mod ffidl_gen;

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

    #[test]
    fn search_client() {
        let req = SearchRequest {
            query: "query abc".to_string(),
            page_number: 10,
            result_per_page: 30,
        }.try_into_tbytes::<PbMessage<_>>().unwrap();
        let cli = unsafe { Test::search_client::<Client>(req).unwrap() };
        println!("{:?}", cli);
    }

    #[test]
    fn empty() {
        let ret = unsafe { Test::test_empty() };
        println!("{:?}", ret);
    }
}

pub struct Test;


impl RustFfi for Test {
    fn search_web_site(mut req: RustFfiArg<SearchRequest>) -> ABIResult<TBytes<WebSite>> {
        let req = req.try_to_object::<PbMessage<_>>();
        println!("request: {:?}", req);
        WebSite {
            name: "andeya".to_string(),
            link: "a/b/c".to_string(),
            age: 40,
            server: HashMap::from([
                ("a".to_string(), Server { hostname: "github.com1".to_string(), port: 801 }),
                ("b".to_string(), Server { hostname: "github.com2".to_string(), port: 802 }),
                ("c".to_string(), Server { hostname: "github.com3".to_string(), port: 803 }),
            ]),
            a: vec![],
            b: vec![],
        }.try_into_tbytes::<PbMessage<_>>()
    }

    fn rs_test_empty() -> ABIResult<()> {
        // Err(ResultMsg{ code: 3, msg: "rs_test_empty test".to_string() })
        Ok(())
    }
}

impl GoFfi for Test {
    unsafe fn search_client_set_result(mut go_ret: RustFfiArg<Client>) -> GoFfiResult {
        GoFfiResult::from_ok(go_ret.try_to_object::<PbMessage<Client>>()?)
    }
}
