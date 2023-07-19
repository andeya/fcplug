#![feature(box_patterns)]

use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::ItemFn;

/// Register an FFI method that communicates using a custom protocol.
/// format description: `#[ffi_raw_callee]`
/// command to check expanded code: `cargo +nightly rustc -- -Zunstable-options
/// --pretty=expanded`
#[proc_macro_attribute]
#[cfg(not(test))] // Work around for rust-lang/rust#62127
pub fn ffi_raw_callee(_args: TokenStream, item: TokenStream) -> TokenStream {
    let raw_item = proc_macro2::TokenStream::from(item.clone());
    let raw_sig = syn::parse_macro_input!(item as ItemFn).sig;
    let raw_ident = raw_sig.ident;
    let new_ident = Ident::new(&format!("ffi_raw_{}", raw_ident), Span::call_site());
    let new_item = quote! {
        #[inline]
        #[no_mangle]
        pub extern "C" fn #new_ident(mut req: ::fcplug::callee::Buffer) -> ::fcplug::callee::FFIResult {
            #raw_item
            ::fcplug::callee::callback(::std::stringify!(#new_ident), #raw_ident, &mut req)
        }
    };
    TokenStream::from(new_item)
}

/// Register an FFI method that communicates using a protobuf protocol.
/// format description: `#[ffi_pb_callee]`
/// command to check expanded code: `cargo +nightly rustc -- -Zunstable-options
/// --pretty=expanded`
#[proc_macro_attribute]
#[cfg(not(test))] // Work around for rust-lang/rust#62127
pub fn ffi_pb_callee(_args: TokenStream, item: TokenStream) -> TokenStream {
    let raw_item = proc_macro2::TokenStream::from(item.clone());
    let raw_sig = syn::parse_macro_input!(item as ItemFn).sig;
    let raw_ident = raw_sig.ident;
    let new_ident = Ident::new(&format!("ffi_pb_{}", raw_ident), Span::call_site());
    let new_item = quote! {
        #[inline]
        #[no_mangle]
        pub extern "C" fn #new_ident(mut req: ::fcplug::callee::Buffer) -> ::fcplug::callee::FFIResult {
            #raw_item
            ::fcplug::callee::callback_pb(::std::stringify!(#new_ident), #raw_ident, &mut req)
        }
    };
    TokenStream::from(new_item)
}
