#![feature(box_patterns)]

use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{ItemFn, ReturnType, Type, TypePath, TypeTuple};

/// Register an FFI method that communicates using a custom protocol.
/// format description: `#[ffi_raw_method]`
/// command to check expanded code: `cargo +nightly rustc -- -Zunstable-options
/// --pretty=expanded`
#[proc_macro_attribute]
#[cfg(not(test))] // Work around for rust-lang/rust#62127
pub fn ffi_raw_method(_args: TokenStream, item: TokenStream) -> TokenStream {
    let raw_item = proc_macro2::TokenStream::from(item.clone());
    let raw_sig = syn::parse_macro_input!(item as ItemFn).sig;
    let raw_ident = raw_sig.ident;
    let new_ident = Ident::new(&format!("ffi_raw_{}", raw_ident), Span::call_site());
    let new_item = quote! {
        #[inline]
        #[no_mangle]
        pub extern "C" fn #new_ident(mut req: ::fcplug_callee::Buffer) -> ::fcplug_callee::FFIResult {
            #raw_item
            ::fcplug_callee::callback(::std::stringify!(#new_ident), #raw_ident, &mut req)
        }
    };
    TokenStream::from(new_item)
}

/// Register an FFI method that communicates using a protobuf protocol.
/// format description: `#[ffi_pb_method]`
/// command to check expanded code: `cargo +nightly rustc -- -Zunstable-options
/// --pretty=expanded`
#[proc_macro_attribute]
#[cfg(not(test))] // Work around for rust-lang/rust#62127
pub fn ffi_pb_method(_args: TokenStream, item: TokenStream) -> TokenStream {
    let raw_item = proc_macro2::TokenStream::from(item.clone());
    let raw_sig = syn::parse_macro_input!(item as ItemFn).sig;
    let raw_ident = raw_sig.ident;
    let new_ident = Ident::new(&format!("ffi_pb_{}", raw_ident), Span::call_site());
    let new_item = quote! {
        #[inline]
        #[no_mangle]
        pub extern "C" fn #new_ident(mut req: ::fcplug_callee::Buffer) -> ::fcplug_callee::FFIResult {
            #raw_item
            ::fcplug_callee::protobuf::callback(::std::stringify!(#new_ident), #raw_ident, &mut req)
        }
    };
    TokenStream::from(new_item)
}


/// Register an FFI method that communicates using a flatbuffer protocol.
/// format description: `#[ffi_fb_method]`
/// command to check expanded code: `cargo +nightly rustc -- -Zunstable-options
/// --pretty=expanded`
#[proc_macro_attribute]
#[cfg(not(test))] // Work around for rust-lang/rust#62127
pub fn ffi_fb_method(_args: TokenStream, item: TokenStream) -> TokenStream {
    const FORMAT: &'static str = "fn ${FN_NAME}<'a>(req: FbRequest<'a, EchoRequest<'a>>) -> (${RESPONSE_TYPE}Args<'a>, FbResponseWriter<${RESPONSE_TYPE}<'a>>){}";
    let format_msg = format!("The function signature format must satisfy: {}", FORMAT);
    let raw_item = proc_macro2::TokenStream::from(item.clone());
    let raw_sig = syn::parse_macro_input!(item as ItemFn).sig;
    let raw_ident = raw_sig.ident;
    let new_ident = Ident::new(&format!("ffi_fb_{}", raw_ident), Span::call_site());
    // TODO: improve
    let resp_type = match raw_sig.output {
        ReturnType::Type(_, box Type::Tuple(TypeTuple { elems, .. })) => {
            if let Type::Path(TypePath { path, .. }) = elems.first().expect(&format_msg) {
                let ident = format!("{}", path.segments.first().expect(&format_msg).ident);
                if ident.ends_with("Args") {
                    Ident::new(ident.strip_suffix("Args").expect(&format_msg), Span::call_site())
                } else {
                    panic!("{}", format_msg)
                }
            } else {
                panic!("{}", format_msg)
            }
        }
        _ => panic!("{}", format_msg)
    };

    let new_item = quote! {
        #[inline]
        #[no_mangle]
        pub extern "C" fn #new_ident(mut req: ::fcplug_callee::Buffer) -> ::fcplug_callee::FFIResult {
            #raw_item
            let (_resp_, mut _w_) = #raw_ident(::fcplug_callee::flatbuf::FbRequest::try_from_buffer(&mut req)?);
            let _resp_ = #resp_type::create(&mut _w_, &_resp_);
            _w_.finish_minimal(_resp_);
            ::fcplug_callee::FFIResult::ok(::fcplug_callee::ABIResponse::try_into_buffer(_w_).unwrap())
        }
    };
    TokenStream::from(new_item)
}
