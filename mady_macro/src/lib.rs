extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

mod graph;
mod parser;
mod gen;

#[proc_macro_attribute]
pub fn grad(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let parser = parser::grad::Parser::new();
    let itemfn = parse_macro_input!(input as ItemFn);
    parser.gen(itemfn).into()
}
