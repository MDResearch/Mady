mod annotator;
mod error;
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
            fn rotate_37(a: Complex<f64>) -> Complex<f64> {
                a * Complex {
                    real: 0.6_f64,
                    imaginary: 0.8_f64,
                }
            }
        };

        let mut parser = new();

        match parser.gen(ts) {
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
