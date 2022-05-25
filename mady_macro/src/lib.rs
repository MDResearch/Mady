use proc_macro::TokenStream;
use quote::{format_ident, quote};
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
/// The 3..N line decribe the formula you want to different; for this instance, it different a `a + b` formula
///
#[proc_macro_attribute]
pub fn grad(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut parser = mady_macro_core::new();
    let tys = parse_macro_input!(attr as Arg);
    let ts = match parser.gen(tys.0, parse_macro_input!(input as ItemFn)) {
        Ok(ts) => quote! {#ts},
        Err(err) => err.to_compile_error(),
    };
    TokenStream::from(ts)
}

#[proc_macro_attribute]
pub fn derive_grad(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut ts = input.clone();
    let mut parser = mady_macro_core::new();
    let tys = parse_macro_input!(attr as Arg);
    let mut func = parse_macro_input!(input as ItemFn);
    func.sig.ident = format_ident!("grad_{}", func.sig.ident);
    let fn_ts = match parser.gen(tys.0, func) {
        Ok(ts) => quote! {#ts},
        Err(err) => err.to_compile_error(),
    };
    ts.extend(TokenStream::from(fn_ts));
    ts
}

struct Arg(Vec<syn::TypePath>);

impl syn::parse::Parse for Arg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let tys =
            syn::punctuated::Punctuated::<syn::TypePath, syn::Token!(,)>::parse_terminated(input)?
                .into_iter()
                .collect();
        Ok(Self(tys))
    }
}
