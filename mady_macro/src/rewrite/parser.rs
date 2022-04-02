use std::slice::IterMut;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Expr, ExprBlock, ItemFn, PatType};

use super::graph::{Graph, Node};
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
    pub stack: Vec<Node<Var, Var>>,
}

#[derive(Debug)]
pub struct Var {
    pub ty: VarType,
    pub annotate: Option<PatType>,
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

    pub fn gen(mut self, t: ItemFn) -> TokenStream {
        let mut chain = Recorder::default();
        match self.fold_chain_item_fn(&mut chain, t) {
            Ok(i) => quote! {#i},
            Err(s) => s.to_compile_error(),
        }
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
