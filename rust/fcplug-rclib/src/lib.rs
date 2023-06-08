pub use fcplug_macros::ffi_method;
pub use crate::abi::{ABIResult, Buffer};

mod abi;
mod fcplug;

pub trait ABIMessage: protobuf::Message + Default {}

impl<T: protobuf::Message + Default> ABIMessage for T {}

pub fn callback<A: ABIMessage, R: ABIMessage, F: Fn(A) -> ABIResult<R>>(_ffi_method_name: &str, f: F, args: Buffer) -> Buffer {
    let args_obj = args.decode::<A>();

    #[cfg(debug_assertions)]
        let txt = format!("invoking method={}, args_bytes={:?}, args_obj={:?}", _ffi_method_name, args.read(), args_obj);


    let res_obj = args_obj.map_or_else(Err, f);

    #[cfg(debug_assertions)]
        let txt = format!("{}, res_obj={:?}", txt, res_obj);

    let res = Buffer::from_abi_result(res_obj);

    #[cfg(debug_assertions)]
    println!("{}, res_bytes={:?}", txt, res.read());

    res
}

#[test]
fn it_works() {
    assert_eq!(4, 2 + 2);
}
