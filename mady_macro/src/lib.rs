extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

mod graph;
mod parser;

#[proc_macro_attribute]
pub fn grad(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut parser = parser::grad::Parser::new();
    let itemfn = parse_macro_input!(input as ItemFn);
    parser.gen(itemfn).into()
}
