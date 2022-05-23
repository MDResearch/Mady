use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Error, Token};

use crate::gen::*;
use crate::generator::gen_declare;
use crate::graph::{Edge, Graph, Node};
use crate::utils::into_hash;

type ParserChian = dyn Chain<Input = Recorder, Err = Error>;
pub type ParseGraph = Graph<VarType, Var>;

#[derive(Default)]
pub struct Parser {
    before: Vec<Box<ParserChian>>,
    after: Vec<Box<ParserChian>>,
    recorder: Option<Recorder>,
}

#[derive(Debug, Default, Clone)]
pub struct Recorder {
    pub graph: ParseGraph,
    // TODO reserve for future use
    // attr_stack:Vec<>,
    link_stack: Vec<Node>,
    node_counter: usize,
    level_counter: usize,
    tys: Vec<syn::TypePath>,
}

#[derive(Debug, Clone)]
pub struct Var {
    id: usize,
    span: Span,
}

#[derive(Debug, Clone, Copy)]
pub struct Id(usize);

struct Attr {
    // TODO reserve for future use
}

#[derive(Debug, Clone)]
pub enum VarType {
    /// init to first assign
    /// store gradient
    Tmp(Var),
    /// init to first assign
    /// store gradient & return as output
    Grad(Var),
    /// init to false, `if ...{...}`
    IF(Var, ParseGraph),
    /// init to false, `if...{...}else{...}`
    ///
    /// `(true, false)`
    IFEL(Var, ParseGraph, ParseGraph),
    /// will optimize out by Mady & rustc
    Null,
}

pub trait Register {
    fn register(self, p: Parser) -> Parser;
}

impl Var {
    pub fn new(r: &mut Recorder, span: Span) -> Self {
        let id = r.node_counter;
        r.node_counter += 1;
        Self { id, span }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn to_ident(&self) -> Ident {
        format_ident!("_mady_var_{}", self.id)
    }

    pub fn to_string(&self) -> String {
        format!("_mady_var_{}", self.id)
    }
}

impl VarType {
    pub fn span(&self) -> Span {
        match self {
            Self::Tmp(v) | Self::Grad(v) => v.span(),
            _ => todo!(),
        }
    }

    pub fn id(&self) -> Id {
        match self {
            VarType::Tmp(v) | VarType::Grad(v) | VarType::IF(v, _) | VarType::IFEL(v, _, _) => {
                Id::new(v.id)
            }
            VarType::Null => todo!(),
        }
    }
}

impl Recorder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn peek_stack(&self) -> Option<Node> {
        self.link_stack.last().copied()
    }

    pub fn pop_stack(&mut self) -> Option<Node> {
        self.link_stack.pop()
    }

    pub fn push_stack(&mut self, value: Node) {
        self.link_stack.push(value)
    }

    pub fn tys(&self) -> &Vec<syn::TypePath> {
        &self.tys
    }

    pub fn push_ty(&mut self, path: syn::TypePath) {
        self.tys.push(path);
    }

    pub fn add_node_and_push_stack(&mut self, value: VarType) -> Node {
        let node = self.graph.add_node(value);
        self.link_stack.push(node);
        node
    }

    pub fn add_edges<T>(&mut self, parent: Node, children: T) -> Vec<Edge>
    where
        T: IntoIterator<Item = Node>,
    {
        let mut edges = vec![];
        for i in children {
            let var = Var::new(self, Span::call_site());
            edges.push(self.graph.add_edge(var, (parent, i)))
        }
        edges
    }

    pub fn is_top_level(&self) -> bool {
        self.level_counter == 1
    }

    pub fn is_sig_level(&self) -> bool {
        self.level_counter == 0
    }

    pub fn enter_block(&mut self) {
        self.level_counter += 1;
    }

    pub fn exit_block(&mut self) {
        self.level_counter -= 1;
    }
}

impl Id {
    fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn to_ident(&self) -> Ident {
        format_ident!("_mady_var_{}", self.0)
    }

    pub fn to_string(&self) -> String {
        format!("_mady_var_{}", self.0)
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

    pub fn gen(&mut self, attr: Vec<syn::TypePath>, t: syn::ItemFn) -> Result<syn::ItemFn, Error> {
        let mut chain = Recorder::new();
        for i in attr {
            chain.push_ty(i);
        }
        let mut func = self.fold_chain_itemfn(&mut chain, t)?;
        let mut declare = gen_declare(&chain)?;
        declare.extend(func.block.stmts);
        func.block.stmts = declare;
        self.recorder = Some(chain);
        Ok(func)
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
