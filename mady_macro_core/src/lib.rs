mod annotator;
mod folder;
mod gen;
mod generator;
mod graph;
mod linker;
mod parser;
mod utils;

use annotator::Annotator;
use folder::Folder;
use linker::Linker;
use parser::Parser;

pub fn new() -> Parser {
    Parser::new()
        .register(Linker::new())
        .register(Annotator::new())
        .register(Folder::new())
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote;

    use super::*;

    #[test]
    fn gen() {
        let ts = parse_quote! {
            fn a(a:usize) -> usize {
                a + 10_usize
            }
        };

        let mut parser = new();

        let ts = match parser.gen(ts) {
            Ok(ts) => quote! {#ts},
            Err(err) => err.to_compile_error(),
        };
        dbg!(ts.to_string());

        let graph = parser.unwarp();
        dbg!(graph);
    }
}
