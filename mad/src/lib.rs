extern crate proc_macro;
use proc_macro::TokenStream;

mod ad;
mod parser;
//mod algorithm;
// unresolved module是洨

#[proc_macro_attribute]
pub fn ad(attr: TokenStream, input: TokenStream) -> TokenStream {
    dbg!(ad::parse(attr, input))
}
