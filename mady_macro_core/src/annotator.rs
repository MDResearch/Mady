use std::str::FromStr;

use crate::error::ParseError;
use crate::gen::Chain;

use crate::utils::{null, ops_to_string, Marker};

use proc_macro2::TokenStream;
use syn::parse_quote;
use syn::{spanned::Spanned, Error};

use crate::parser::{Parser, Recorder, Register, VarType};

#[derive(Default)]
pub struct Annotator;

struct AfterAnnotator;

impl Annotator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Register for Annotator {
    fn register(self, p: Parser) -> Parser {
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
            .ok_or(ParseError::NotFindNode.new(t.span()))?;
        if let VarType::Tmp(v) | VarType::Grad(v) = c.graph.node_weight_mut(node) {
            *v.ty_mut() = Some(t.clone());
            Ok(t)
        } else {
            Err(ParseError::UnexpectNodeType.new(t.path.span()))
        }
    }

    fn chain_exprbinary(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprBinary,
    ) -> Result<syn::ExprBinary, Self::Err> {
        let parent = c
            .peek_stack()
            .ok_or(ParseError::NotFindNode.new(t.left.span()))?;

        let iter = c.graph.to_nodes(parent);
        let marker = Marker::new_method(
            ops_to_string(&t.op).ok_or(ParseError::UnsupportedSyntax.new(t.op.span()))?,
            c.graph.node_weight(iter[0]).id(),
            &vec![c.graph.node_weight(iter[1]).id()],
        );

        for (i, e) in c.graph.to_edges(parent).into_iter().enumerate() {
            *c.graph.edge_weight_mut(e).ty_mut() = Some(marker.grad(i))
        }

        if let VarType::Tmp(v) | VarType::Grad(v) = c.graph.node_weight_mut(parent) {
            *v.ty_mut() = Some(marker.output(0));
            Ok(t)
        } else {
            Err(ParseError::UnexpectNodeType.new(t.span()))
        }
    }

    fn chain_local(&mut self, c: &mut Self::Input, t: syn::Local) -> Result<syn::Local, Self::Err> {
        // drop left node
        let left = c
            .pop_stack()
            .ok_or(ParseError::NotFindNode.new(t.pat.span()))?;
        *c.graph.node_weight_mut(left) = VarType::Null;

        Ok(t)
    }

    fn chain_litint(
        &mut self,
        c: &mut Self::Input,
        t: syn::LitInt,
    ) -> Result<syn::LitInt, Self::Err> {
        let node = c
            .peek_stack()
            .ok_or(ParseError::NotFindNode.new(t.span()))?;
        if t.suffix().is_empty() {
            return Err(ParseError::CantInferType.new(t.span()));
        }

        if let VarType::Tmp(v) = c.graph.node_weight_mut(node) {
            let ty = TokenStream::from_str(t.suffix()).unwrap();
            *v.ty_mut() = Some(parse_quote! {#ty});
            Ok(t)
        } else {
            Err(ParseError::UnexpectNodeType.new(t.span()))
        }
    }

    fn chain_litfloat(
        &mut self,
        c: &mut Self::Input,
        t: syn::LitFloat,
    ) -> Result<syn::LitFloat, Self::Err> {
        let node = c
            .peek_stack()
            .ok_or(ParseError::NotFindNode.new(t.span()))?;
        if t.suffix().is_empty() {
            return Err(ParseError::CantInferType.new(t.span()));
        }

        if let VarType::Tmp(v) = c.graph.node_weight_mut(node) {
            let ty = TokenStream::from_str(t.suffix()).unwrap();
            *v.ty_mut() = Some(parse_quote! {#ty});
            Ok(t)
        } else {
            Err(ParseError::UnexpectNodeType.new(t.span()))
        }
    }
}
