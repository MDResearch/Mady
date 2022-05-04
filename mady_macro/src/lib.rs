use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn grad(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut parser = mady_macro_core::new();
    let ts = match parser.gen(parse_macro_input!(input as ItemFn)) {
        Ok(ts) => quote! {#ts},
        Err(err) => err.to_compile_error(),
    };
    TokenStream::from(ts)
}
