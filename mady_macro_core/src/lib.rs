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

}
