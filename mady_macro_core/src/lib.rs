mod annotator;
mod folder;
mod graph;
mod linker;
mod parser;
mod utils;

use folder::Folder;
use graph::{Edge, Node};
use linker::Linker;
use parser::Parser;
use proc_macro2::Ident;
use quote::format_ident;

pub fn new() -> Parser {
    Parser::new()
        .register(Linker::new())
        .register(Folder::new())
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn gen() {
        let ts = parse_quote! {
            fn a(a:usize, b:usize) -> usize {
                a + b
            }
        };

        let (ts, graph) = new().gen(ts);

        dbg!(ts.to_string());
        dbg!(graph);
    }
}
