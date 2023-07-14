#![feature(try_trait_v2)]
#![feature(new_uninit)]

pub mod callee;
pub mod caller;
pub mod ctypes;


#[test]
fn it_works() {
    assert_eq!(4, 2 + 2);
}

// #[macro_export]
// macro_rules! include_gen_file {
//     ($gen_file: tt) => {
//         include!(concat!(env!("OUT_DIR"), concat!("/", $gen_file)));
//     };
// }

#[macro_export]
macro_rules! include_goffi_gen {
    () => {
        include!(concat!(env!("OUT_DIR"), concat!("/", "goffi_gen.rs")));
    };
}
