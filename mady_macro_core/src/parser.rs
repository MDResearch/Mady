use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Error;

use super::graph::{Edge, Graph, Node};
use crate::gen::*;

type ParserChian = dyn Chain<Input = Recorder, Err = Error>;

#[derive(Default)]
pub struct Parser {
    before: Vec<Box<ParserChian>>,
    after: Vec<Box<ParserChian>>,
}

#[derive(Debug, Default)]
pub struct Recorder {
    pub graph: Graph<Var, Var>,
    stack: Vec<Node>,
}

#[derive(Debug)]
pub struct Var {
    ty: VarType,
    annotate: Option<syn::TypePath>,
}

#[derive(Debug)]
pub enum VarType {
    /// not init
    TMP,
    /// init to Zero::zero
    Grad,
    /// not init, `if ...{...}`
    IF,
    /// not init, `if...{...}else{...}`
    IFEL,
}

pub trait Register {
    fn register(self, p: Parser) -> Parser;
}

impl Var {
    pub fn new(ty: VarType) -> Self {
        Self { ty, annotate: None }
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
}

impl Recorder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn peek_stack(&self) -> Option<Node> {
        self.stack.last().map(|x| *x)
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
            edges.push(self.graph.add_edge(Var::new(VarType::TMP), (parent, i)))
        }
        edges
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

    pub fn gen(mut self, t: syn::ItemFn) -> (TokenStream, Recorder) {
        let mut chain = Recorder::default();
        let ts = match self.fold_chain_item_fn(&mut chain, t) {
            Ok(i) => quote! {#i},
            Err(s) => s.to_compile_error(),
        };
        (ts, chain)
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
