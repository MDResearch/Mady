mod error;
mod folder;
mod gen;
mod generator;
mod graph;
mod linker;
mod parser;
mod utils;

use folder::Folder;
use linker::Linker;
use parser::Parser;

pub fn new() -> Parser {
    Parser::new()
        .register(Linker::new())
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
            fn nd(a: f64, b: f64) -> f64 {
                f64::add(a, b)
            }
        };
        let tys = vec![parse_quote!(f64), parse_quote!(f64)];

        let mut parser = new();

        match parser.gen(tys, ts) {
            Ok(ts) => {
                dbg!(quote! {#ts}.to_string());
            }
            Err(err) => {
                dbg!((err.span().start(), err.span().end()));
                dbg!(err.to_string());
            }
        }

        let graph = parser.unwarp();
        dbg!(graph);
    }
}
