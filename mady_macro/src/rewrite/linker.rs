use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::LinkedList;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use syn::spanned::Spanned;
use syn::Error;

use super::graph::Node;
use super::parser::{Recorder, Register, Var, VarType};
use crate::gen::Chain;

#[derive(Default)]
pub struct Linker {
    stack: Vec<LinkedList<(u64, Node)>>,
}

struct BeforeLinker(Rc<RefCell<Linker>>);

struct AfterLinker(Rc<RefCell<Linker>>);

impl Linker {
    pub fn new() -> Self {
        Default::default()
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
        self.0
            .borrow_mut()
            .stack
            .pop()
            .ok_or(Error::new(t.span(), "unexpect out of block"))?;
            
        Ok(t)
    }

    fn chain_path(&mut self, c: &mut Self::Input, t: syn::Path) -> Result<syn::Path, Self::Err> {
        if let Some(v) = t.get_ident() {
            let hash = into_hash(v);

            for s in self.0.borrow_mut().stack.iter().rev() {
                for (var_hash, node) in s.iter().rev() {
                    if &hash == var_hash {
                        c.push_stack(*node);
                        return Ok(t);
                    }
                }
            }
        }

        Err(Error::new(t.span(), "cannot find var in block stack"))
    }

    fn chain_pat_ident(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatIdent,
    ) -> Result<syn::Pat, Self::Err> {
        let node = c.add_node_and_push_stack(Var::new(VarType::Grad));
        self.0
            .borrow_mut()
            .stack
            .last_mut()
            .ok_or(Error::new(t.span(), "unexpect out of block"))?
            .push_back((into_hash(&t.ident), node));

        Ok(syn::Pat::Ident(t))
    }

    fn chain_expr_binary(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprBinary,
    ) -> Result<syn::Expr, Self::Err> {
        let right = c
            .pop_stack()
            .ok_or(Error::new(t.span(), "cannot find varible at `right` side"))?;
        let left = c
            .pop_stack()
            .ok_or(Error::new(t.span(), "cannot find varible at `left` side"))?;
        let parent = c.add_node_and_push_stack(Var::new(VarType::Grad));
        c.add_tmp_edges(parent, [left, right]);

        Ok(syn::Expr::Binary(t))
    }
}

fn into_hash<T>(v: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    v.hash(&mut hasher);
    hasher.finish()
}
