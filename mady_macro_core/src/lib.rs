mod annotator;
mod folder;
mod gen;
mod graph;
mod linker;
mod parser;
mod utils;

use annotator::Annotator;
use folder::Folder;
use graph::{Edge, Node};
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
