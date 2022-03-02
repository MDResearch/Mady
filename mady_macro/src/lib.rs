extern crate proc_macro;
use proc_macro::TokenStream;

mod graph;
mod parser;

#[proc_macro_attribute]
pub fn grad(_attr: TokenStream, _input: TokenStream) -> TokenStream {
    todo!()
}
