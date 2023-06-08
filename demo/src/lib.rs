use fcplug_rclib::*;
mod idl;
use idl::Echo;

#[ffi_method]
fn echo(args: Echo) -> ABIResult<Echo> {
    let mut r=Echo::new();
    r.set_msg("input is: ".to_string() + args.get_msg());
    Ok(r)
}

#[test]
fn test_echo() {
    let mut args=Echo::new();
    args.set_msg("andeya".to_string());
    let r = ffi_echo(args.into());
    println!("r={:?}", r.to_abi_result::<Echo>());
}
