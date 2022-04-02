mod annotator;
mod graph;
mod linker;
mod parser;

use linker::Linker;
use parser::Parser;

pub fn new() -> Parser {
    Parser::new().register(Linker::default())
}
