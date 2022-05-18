use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// this scope describe how to use this library
///
/// "grad" is a macro which let you get to the differentiation of the formula you inputed
///
///  the usage of [grad] attribute will be introduced below
///     
///  ```ignore
/// #[grad]
/// fn add(a: f64, b: f64) -> f64 {
///     a + b
/// }
///  ```
///
/// The 1st line decribe the method of importing grad
///  
/// The 2nd line describe a function,and this function include the formula to be different.
/// The parameter involve which Immutable number used in the formula
///
/// The third line decribe the formula you want to different; for this instance, it different a `a + b` formula
///
#[proc_macro_attribute]
pub fn grad(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut parser = mady_macro_core::new();
    let ts = match parser.gen(parse_macro_input!(input as ItemFn)) {
        Ok(ts) => quote! {#ts},
        Err(err) => err.to_compile_error(),
    };
    TokenStream::from(ts)
}
