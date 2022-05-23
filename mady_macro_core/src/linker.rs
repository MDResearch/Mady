use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;

use syn::spanned::Spanned;
use syn::Error;

use crate::error::ParseError;
use crate::gen::Chain;
use crate::graph::Node;
use crate::parser::{Parser, Recorder, Register, Var, VarType};
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
    fn register(self, p: Parser) -> Parser {
        let share = Rc::new(RefCell::new(self));
        p.add_before(BeforeLinker(share.clone()))
            .add_after(AfterLinker(share))
    }
}

impl Linker {
    fn enter_block(&mut self) {
        self.stack.push(Default::default());
    }

    fn exit_block(&mut self) {
        // should never out of block
        self.stack.pop().unwrap();
    }

    fn new_var(&mut self, hash: u64, node: Node) {
        // should never out of block
        self.stack
            .last_mut()
            .map(|x| x.push_back((hash, node)))
            .unwrap();
    }

    fn replace_phantom(&mut self, hash: u64, new: Node) -> Option<()> {
        self.stack
            .iter_mut()
            .rev()
            .flat_map(|x| x.iter_mut().rev())
            .find_map(|(x, n)| (x == &hash).then(|| *n = new))
    }

    fn find_var(&self, hash: u64) -> Option<Node> {
        self.stack
            .iter()
            .rev()
            .flat_map(|x| x.iter().rev())
            .find_map(|(x, n)| (x == &hash).then(|| *n))
    }
}

impl Chain for BeforeLinker {
    type Input = Recorder;
    type Err = Error;

    fn chain_block(&mut self, c: &mut Self::Input, t: syn::Block) -> Result<syn::Block, Self::Err> {
        self.0.borrow_mut().enter_block();
        c.enter_block();
        Ok(t)
    }
}

impl Chain for AfterLinker {
    type Input = Recorder;
    type Err = Error;

    fn chain_block(&mut self, c: &mut Self::Input, t: syn::Block) -> Result<syn::Block, Self::Err> {
        self.0.borrow_mut().exit_block();
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
                .ok_or(ParseError::UnsupportedSyntax.new(t.path.span()))?,
        );
        if let Some(node) = self.0.borrow().find_var(hash) {
            c.push_stack(node);
            Ok(t)
        } else {
            Err(ParseError::NotFindValue.new(t.span()))
        }
    }

    fn chain_patident(
        &mut self,
        c: &mut Self::Input,
        t: syn::PatIdent,
    ) -> Result<syn::PatIdent, Self::Err> {
        if c.is_sig_level() {
            let var = VarType::Grad(Var::new(c, t.span()));
            self.0
                .borrow_mut()
                .new_var(into_hash(&t.ident), c.graph.add_node(var));
        } else {
            let var = VarType::Tmp(Var::new(c, t.span()));
            self.0
                .borrow_mut()
                .new_var(into_hash(&t.ident), c.add_node_and_push_stack(var));
        };

        Ok(t)
    }

    fn chain_exprbinary(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprBinary,
    ) -> Result<syn::ExprBinary, Self::Err> {
        let right = c
            .pop_stack()
            .ok_or(ParseError::NotFindNode.new(t.right.span()))?;
        let left = c
            .pop_stack()
            .ok_or(ParseError::NotFindNode.new(t.left.span()))?;
        let var = VarType::Tmp(Var::new(c, t.span()));
        let parent = c.add_node_and_push_stack(var);
        c.add_edges(parent, [left, right]);

        Ok(t)
    }

    fn chain_exprassign(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprAssign,
    ) -> Result<syn::ExprAssign, Self::Err> {
        if let syn::Expr::Path(ref p) = *t.left {
            let left = p
                .path
                .get_ident()
                .ok_or(ParseError::UnsupportedSyntax.new(t.left.span()))?;
            let right = c
                .pop_stack()
                .ok_or(ParseError::NotFindNode.new(t.right.span()))?;

            // drop
            c.pop_stack()
                .ok_or(ParseError::NotFindNode.new(t.left.span()))?;

            self.0
                .borrow_mut()
                .replace_phantom(into_hash(left), right)
                .ok_or(ParseError::NotFindValue.new(t.left.span()))?;
            Ok(t)
        } else {
            Err(ParseError::UnsupportedSyntax.new(t.left.span()))
        }
    }

    fn chain_local(&mut self, c: &mut Self::Input, t: syn::Local) -> Result<syn::Local, Self::Err> {
        if let syn::Pat::Ident(ref p) = t.pat {
            if let Some((_, ref init)) = t.init {
                let right = c
                    .pop_stack()
                    .ok_or(ParseError::NotFindNode.new(init.span()))?;
                self.0
                    .borrow_mut()
                    .replace_phantom(into_hash(&p.ident), right)
                    .ok_or(ParseError::NotFindValue.new(p.span()))?;
            }
            // drop
            let left = c
                .pop_stack()
                .ok_or(ParseError::NotFindNode.new(t.pat.span()))?;
            *c.graph.node_weight_mut(left) = VarType::Null;

            Ok(t)
        } else {
            Err(ParseError::UnsupportedSyntax.new(t.span()))
        }
    }

    fn chain_exprmethodcall(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprMethodCall,
    ) -> Result<syn::ExprMethodCall, Self::Err> {
        let mut children = vec![];
        for _ in 0..=t.args.len() {
            children.push(
                c.pop_stack()
                    .ok_or(ParseError::NotFindNode.new(t.args.span()))?,
            );
        }
        let var = VarType::Tmp(Var::new(c, t.span()));
        let parent = c.add_node_and_push_stack(var);

        c.add_edges(parent, children.into_iter().rev());

        Ok(t)
    }

    // fn chain_exprcall(
    //     &mut self,
    //     c: &mut Self::Input,
    //     t: syn::ExprCall,
    // ) -> Result<syn::ExprCall, Self::Err> {
    //     let mut children = vec![];
    //     for _ in 0..=t.args.len() {
    //         children.push(
    //             c.pop_stack()
    //                 .ok_or(ParseError::NotFindNode.new(t.args.span()))?,
    //         );
    //     }
    //     let var = VarType::Tmp(Var::new(c, t.span()));
    //     let parent = c.add_node_and_push_stack(var);

    //     c.add_edges(parent, children.into_iter().rev());

    //     Ok(t)
    // }

    fn chain_returntype_type(
        &mut self,
        c: &mut Self::Input,
        t: (syn::token::RArrow, Box<syn::Type>),
    ) -> Result<syn::ReturnType, Self::Err> {
        match &*t.1 {
            syn::Type::Path(p) => c.push_ty(p.clone()),
            // reserve for future use
            syn::Type::Tuple(_) => todo!(),
            _ => todo!(),
        }
        Ok(syn::ReturnType::Type(t.0, t.1))
    }

    fn chain_litint(
        &mut self,
        c: &mut Self::Input,
        t: syn::LitInt,
    ) -> Result<syn::LitInt, Self::Err> {
        let var = VarType::Tmp(Var::new(c, t.span()));
        c.add_node_and_push_stack(var);
        Ok(t)
    }

    fn chain_litfloat(
        &mut self,
        c: &mut Self::Input,
        t: syn::LitFloat,
    ) -> Result<syn::LitFloat, Self::Err> {
        let var = VarType::Tmp(Var::new(c, t.span()));
        c.add_node_and_push_stack(var);
        Ok(t)
    }
}
