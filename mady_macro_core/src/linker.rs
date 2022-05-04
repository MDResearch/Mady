use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;

use syn::spanned::Spanned;
use syn::Error;

use super::graph::Node;
use super::parser::{Recorder, Register, Var, VarType};
use crate::gen::Chain;
use crate::utils::into_hash;

#[derive(Default)]
pub struct Linker {
    stack: Vec<LinkedList<(u64, Node)>>,
}

struct BeforeLinker(Rc<RefCell<Linker>>);

struct AfterLinker(Rc<RefCell<Linker>>);

impl Linker {
    pub fn new() -> Self {
        let mut tmp = Self::default();
        tmp.stack.push(Default::default());
        tmp
    }
}

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

    fn chain_block(&mut self, c: &mut Self::Input, t: syn::Block) -> Result<syn::Block, Self::Err> {
        self.0.borrow_mut().stack.push(Default::default());
        c.enter_block();
        Ok(t)
    }
}

impl Chain for AfterLinker {
    type Input = Recorder;
    type Err = Error;

    fn chain_block(&mut self, c: &mut Self::Input, t: syn::Block) -> Result<syn::Block, Self::Err> {
        self.0
            .borrow_mut()
            .stack
            .pop()
            .ok_or(Error::new(t.span(), "unexpect out of block"))?;
        c.exit_block();
        Ok(t)
    }

    fn chain_exprpath(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprPath,
    ) -> Result<syn::ExprPath, Self::Err> {
        let hash = into_hash(
            t.path
                .get_ident()
                .ok_or(Error::new(t.span(), "cannot convert to ident"))?,
        );

        for s in self.0.borrow_mut().stack.iter().rev() {
            for (var_hash, node) in s.iter().rev() {
                if &hash == var_hash {
                    c.push_stack(*node);
                    return Ok(t);
                }
            }
        }
        Err(Error::new(t.span(), "cannot find var in block stack"))
    }

    fn chain_patident(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatIdent,
    ) -> Result<syn::PatIdent, Self::Err> {
        let node = if c.block_level() == 0 {
            c.add_node_and_push_stack(Var::new(VarType::Out, t.span()))
        } else {
            c.add_node_and_push_stack(Var::new(VarType::Grad, t.span()))
        };

        self.0
            .borrow_mut()
            .stack
            .last_mut()
            .ok_or(Error::new(t.span(), "unexpect out of block"))?
            .push_back((into_hash(&t.ident), node));
        Ok(t)
    }

    fn chain_exprbinary(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprBinary,
    ) -> Result<syn::ExprBinary, Self::Err> {
        let right = c
            .pop_stack()
            .ok_or(Error::new(t.span(), "cannot find varible at `right` side"))?;
        let left = c
            .pop_stack()
            .ok_or(Error::new(t.span(), "cannot find varible at `left` side"))?;
        let parent = c.add_node_and_push_stack(Var::new(VarType::Grad, t.span()));
        c.add_tmp_edges(parent, [left, right]);

        Ok(t)
    }

    fn chain_litint(&mut self, c: &mut Self::Input, t: syn::LitInt) -> Result<syn::LitInt, Self::Err> {
        c.add_node_and_push_stack(Var::new(VarType::Grad, t.span()));
        Ok(t)
    }

    fn chain_litfloat(&mut self, c: &mut Self::Input, t: syn::LitFloat) -> Result<syn::LitFloat, Self::Err> {
        c.add_node_and_push_stack(Var::new(VarType::Grad, t.span()));
        Ok(t)
    }
}
