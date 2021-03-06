use proc_macro2::Span;
use syn::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("cannot find value in this scope")]
    NotFindValue,

    #[error("cannot find node in link stack, maybe use unsupported syntax")]
    NotFindNode,

    #[error("cannot find type in return type list, maybe use unsupported syntax")]
    NotFindType,

    #[error("unsupported syntax")]
    UnsupportedSyntax,

    #[error("cannot infer type, please add type here")]
    CantInferType,

    #[error("unexpect node type, maybe use unsupported syntax")]
    UnexpectNodeType,

    #[error("unexpect error, maybe use unsupported syntax")]
    UnexpectError,
}

impl ParseError {
    pub fn new(self, span: Span) -> Error {
        // panic!("Error: {}", self);
        Error::new(span, self)
    }
}
