use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;

use syn::Error;

use crate::gen::Chain;

use super::graph::Node;
use super::parser::{Recorder, Register, Var};

#[derive(Default)]
pub struct Linker {
    stack: Vec<LinkedList<Node<Var, Var>>>,
}

pub struct BeforeLinker(Rc<RefCell<Linker>>);

pub struct AfterLinker(Rc<RefCell<Linker>>);

impl Register for Linker {
    fn register(self, p: super::parser::Parser) -> super::parser::Parser {
        let share = Rc::new(RefCell::new(self));
        p.add_before(BeforeLinker(share.clone()))
            .add_after(AfterLinker(share))
    }
}

impl Chain for BeforeLinker {
    type Input = Recorder;
    type Err = Error;

    fn chain_block(
        &mut self,
        _c: &mut Self::Input,
        t: syn::Block,
    ) -> Result<syn::Block, Self::Err> {
        self.0.borrow_mut().stack.push(Default::default());
        Ok(t)
    }
}

impl Chain for AfterLinker {
    type Input = Recorder;
    type Err = Error;

    fn chain_block(
        &mut self,
        _c: &mut Self::Input,
        t: syn::Block,
    ) -> Result<syn::Block, Self::Err> {
        self.0.borrow_mut().stack.pop();
        Ok(t)
    }
}
