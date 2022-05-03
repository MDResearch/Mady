use crate::gen::Chain;
use crate::graph::Node;
use crate::utils::{ops_to_string, Marker};

use syn::{spanned::Spanned, Error};

use crate::parser::{Recorder, Register};

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

    fn chain_typepath(
        &mut self,
        c: &mut Self::Input,
        t: syn::TypePath,
    ) -> Result<syn::TypePath, Self::Err> {
        let node = c
            .peek_stack()
            .ok_or(Error::new(t.span(), "cannot find varible name"))?;
        *c.graph.node_weight_mut(node).annotate_mut() = Some(t.clone());
        Ok(t)
    }

    fn chain_exprbinary(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprBinary,
    ) -> Result<syn::ExprBinary, Self::Err> {
        let parent = c
            .peek_stack()
            .ok_or(Error::new(t.span(), "cannot find node when fold binary"))?;

        let iter = c.graph.to_nodes(parent);
        let marker = Marker::new_method(
            ops_to_string(&t.op).ok_or(Error::new(t.op.span(), "unsupport Op"))?,
            iter[0],
            vec![iter[1]],
        );

        for (i, e) in c.graph.to_edges(parent).into_iter().enumerate() {
            *c.graph.edge_weight_mut(e).annotate_mut() = Some(marker.grad(i))
        }
        *c.graph.node_weight_mut(parent).annotate_mut() = Some(marker.output(0));
        
        Ok(t)
    }
}
