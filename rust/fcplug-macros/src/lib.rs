use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};

use quote::quote;
use syn::ItemFn;

/// Register method's ABI for callback.
/// format description: `#[ffi_method]`
/// command to check expanded code: `cargo +nightly rustc -- -Zunstable-options
/// --pretty=expanded`
#[proc_macro_attribute]
#[cfg(not(test))] // Work around for rust-lang/rust#62127
pub fn ffi_method(_args: TokenStream, item: TokenStream) -> TokenStream {
    let raw_item = proc_macro2::TokenStream::from(item.clone());
    let raw_sig = syn::parse_macro_input!(item as ItemFn).sig;
    let raw_ident = raw_sig.ident;
    let new_ident = Ident::new(&format!("ffi_{}", raw_ident), Span::call_site());
    let new_item = quote! {
        #[no_mangle]
        pub extern "C" fn #new_ident(args: ::fcplug_rclib::Buffer) -> ::fcplug_rclib::Buffer {
            #raw_item
            ::fcplug_rclib::callback(::std::stringify!(#new_ident), #raw_ident, args)
        }
    };
    TokenStream::from(new_item)
}
