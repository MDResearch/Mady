use std::time::{SystemTime, UNIX_EPOCH};

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Error, Token};

use super::graph::{Edge, Graph, Node};
use crate::{
    gen::*,
    generator::{gen_declare, gen_types},
};

type ParserChian = dyn Chain<Input = Recorder, Err = Error>;

#[derive(Default)]
pub struct Parser {
    before: Vec<Box<ParserChian>>,
    after: Vec<Box<ParserChian>>,
    recorder: Option<Recorder>,
}

#[derive(Debug, Default, Clone)]
pub struct Recorder {
    pub graph: Graph<Var, Var>,
    stack: Vec<Node>,
    // todo; add attr
    counter: usize,
}

#[derive(Debug, Clone)]
pub struct Var {
    ty: VarType,
    annotate: Option<syn::TypePath>,
    span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VarType {
    /// not init
    TMP,
    /// init to first assign
    /// store gradient
    Grad,
    /// init to first assign
    /// store gradient & return as output
    Out,
    /// init to false, `if ...{...}`
    IF,
    /// init to false, `if...{...}else{...}`
    IFEL,
}

pub trait Register {
    fn register(self, p: Parser) -> Parser;
}

impl Var {
    pub fn new(ty: VarType, span: Span) -> Self {
        Self {
            ty,
            annotate: None,
            span,
        }
    }

    pub fn annotate(&self) -> &Option<syn::TypePath> {
        &self.annotate
    }

    pub fn annotate_mut(&mut self) -> &mut Option<syn::TypePath> {
        &mut self.annotate
    }

    pub fn ty(&self) -> &VarType {
        &self.ty
    }

    pub fn ty_mut(&mut self) -> &mut VarType {
        &mut self.ty
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

impl Recorder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn peek_stack(&self) -> Option<Node> {
        self.stack.last().copied()
    }

    pub fn pop_stack(&mut self) -> Option<Node> {
        self.stack.pop()
    }

    pub fn push_stack(&mut self, value: Node) {
        self.stack.push(value)
    }

    pub fn add_node_and_push_stack(&mut self, value: Var) -> Node {
        let node = self.graph.add_node(value);
        self.stack.push(node);
        node
    }

    pub fn add_tmp_edges<T>(&mut self, parent: Node, children: T) -> Vec<Edge>
    where
        T: IntoIterator<Item = Node>,
    {
        let mut edges = vec![];
        for i in children {
            edges.push(
                self.graph
                    .add_edge(Var::new(VarType::TMP, Span::call_site()), (parent, i)),
            )
        }
        edges
    }

    pub fn block_level(&self) -> usize {
        self.counter
    }

    pub fn enter_block(&mut self) {
        self.counter += 1;
    }

    pub fn exit_block(&mut self) {
        self.counter += 1;
    }
}

impl Parser {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn register<T>(self, r: T) -> Self
    where
        T: Register,
    {
        r.register(self)
    }

    pub fn add_before<T>(mut self, next: T) -> Self
    where
        T: Chain<Input = Recorder, Err = Error> + 'static,
    {
        self.before.push(Box::new(next));
        self
    }

    pub fn add_after<T>(mut self, next: T) -> Self
    where
        T: Chain<Input = Recorder, Err = Error> + 'static,
    {
        self.after.push(Box::new(next));
        self
    }

    pub fn gen(&mut self, t: syn::ItemFn) -> Result<TokenStream, Error> {
        let mut chain = Recorder::new();
        let mut func = self.fold_chain_itemfn(&mut chain, t)?;
        let types = gen_types(&chain)?;
        let mut declare = gen_declare(&chain)?;
        let time = (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            & (usize::MAX as u128)) as usize;
        let mod_name = format_ident!("mady_{}", time);
        let func_name = func.sig.ident.clone();
        let vis = match func.vis.clone() {
            syn::Visibility::Public(p) => quote! {
                #p #func_name;
            },
            syn::Visibility::Restricted(p) => quote! {
                #p #func_name;
            },
            syn::Visibility::Crate(_) | syn::Visibility::Inherited => quote! {},
        };
        func.vis = syn::Visibility::Public(syn::VisPublic {
            pub_token: <Token!(pub)>::default(),
        });
        declare.extend(func.block.stmts);
        func.block.stmts = declare;
        self.recorder = Some(chain);
        Ok(quote! {
            use #mod_name::#func_name;
            #vis
            mod #mod_name{
                use super::*;
                #(#types)*
                #func
            }
        })
    }

    pub fn unwarp(self) -> Recorder {
        self.recorder.unwrap()
    }
}

impl ChainIter for Parser {
    type Input = Recorder;
    type Err = Error;

    fn before(
        &mut self,
    ) -> Box<dyn Iterator<Item = &mut Box<dyn Chain<Input = Self::Input, Err = Self::Err>>> + '_>
    {
        Box::new(self.before.iter_mut())
    }

    fn after(
        &mut self,
    ) -> Box<dyn Iterator<Item = &mut Box<dyn Chain<Input = Self::Input, Err = Self::Err>>> + '_>
    {
        Box::new(self.after.iter_mut())
    }
}

impl FoldChain for Parser {}
