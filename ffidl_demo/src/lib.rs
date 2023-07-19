use std::collections::HashMap;
use crate::gen::{RustFfi, SearchRequest, Server, WebSite};

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
    fn search(req: &SearchRequest) -> WebSite {
        println!("request: {:?}", req);
        WebSite{
            name: "andeya".to_string(),
            link: "a/b/c".to_string(),
            age: 40,
            server: HashMap::from([
                ("a".to_string(),Server{ hostname: "github.com1".to_string(), port: 801 }),
                ("b".to_string(),Server{ hostname: "github.com2".to_string(), port: 802 }),
                ("c".to_string(),Server{ hostname: "github.com3".to_string(), port: 803 }),
            ]),
        }
    }
}
