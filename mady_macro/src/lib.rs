use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn};

// TODO add module level doc

/// this scope describe the main function of this library.
///
/// "grad" is a macro which let you get to the partial derivative of each overload of function.
///
///  the usage of #[grad] attribute will be introduced below.
///     
///  ```ignore
/// #[grad(f64, f64)]
/// fn add(a: f64, b: f64) -> f64 {
///     a + b
/// }
///  ```
///
/// The output function should worked like the function below.
///
/// But the actuall output should be more complicated and not so human readable.
///
/// output of add: (original output of add, (a's partial derivative to add(a,b), b's partial derivative to add(a,b)))
///
/// ```ignore
/// fn add(a: f64, b: f64)->(f64 ,f64 ,f64){
///     (a+b, (1 ,1))
/// }
/// ```
///
/// All function call in grad functions should be either differentiable ,min ,max or elementary arithmetic.
///
/// If the function call isn't supported, you can create a grad function your own.
///
/// Below is an example to add sin support to be called in grad functions.
/// ```ignore
/// impl GradSin for f64 {
///     fn gradsin(self) -> self {
///         self.cos()
///     }
/// }
/// ```
///
/// Also of note that the function "add" will disappear, if you want to keep "add" and "gradadd", you can use #[macro@derive_grad]
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

///
/// 'derive_grad' is a macro which let you get to the partial derivative of each overload of function.
///
/// It's very similar to what #[macro@grad] do to your function, the only difference is that #[derive_grad] will keep your original function
///
/// ```ignore
/// fn original()...
///
/// fn grad_original()...
/// ```
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
