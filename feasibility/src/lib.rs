use std::collections::{HashMap, VecDeque};

// #![allow(improper_ctypes)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug, Eq, PartialEq)]
pub enum X {
    A(Vec<String>, usize),
}

#[derive(Debug, Eq, PartialEq)]
pub struct TestObject {
    map: HashMap<String, Vec<String>>,
    queue: VecDeque<String>,
    r#enum: X,
}

impl TestObject {
    fn new_test() -> Self {
        let mut obj = TestObject {
            map: Default::default(),
            queue: Default::default(),
            r#enum: X::A(vec!["a".to_string()], 1),
        };
        obj.map.insert("k".to_string(), vec!["v".to_string()]);
        obj.queue.push_back("q".to_string());
        obj
    }
}

// #[no_mangle]
// pub extern "C" fn get_leak_ptr() -> u64 {
//     Box::leak(Box::new(TestObject::new_test())) as *mut TestObject as u64
// }

// extern "C" {
//     pub fn get_leak_ptr() -> u64;
// }

#[cfg(test)]
mod tests {
    // mod bindings;
    use super::*;

    #[test]
    fn it_works() {
        let obj = unsafe { Box::from_raw(bindings::get_leak_ptr() as *mut TestObject) };
        assert_eq!(TestObject::new_test(), *obj);
    }
}
