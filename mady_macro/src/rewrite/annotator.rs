use crate::gen::Chain;

use super::graph::{Edge, Node};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_quote, spanned::Spanned, Error, PatPath};

use super::parser::{Recorder, Register, Var, VarType};

#[derive(Default)]
pub struct Annotator;

struct AfterAnnotator;

impl Annotator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Register for Annotator {
    fn register(self, p: super::parser::Parser) -> super::parser::Parser {
        p.add_after(AfterAnnotator)
    }
}

impl Chain for AfterAnnotator {
    type Input = Recorder;
    type Err = Error;
    fn chain_pat_type(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatType,
    ) -> Result<syn::Pat, Self::Err> {
        let node = c
            .peek_stack()
            .ok_or(Error::new(t.span(), "cannot find varible name"))?;
        if let syn::Type::Path(ref v) = *t.ty {
            *c.graph.node_weight_mut(node).annotate_mut() = Some(v.clone());
        }
        Ok(syn::Pat::Type(t))
    }
}
