use syn::spanned::Spanned;
use syn::{parse_quote, Error};

use crate::gen::Chain;
use crate::utils::grad_method;

use super::parser::{Recorder, Register};

#[derive(Default)]
pub struct Folder;

struct AfterFolder;

impl Folder {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Register for Folder {
    fn register(self, p: super::parser::Parser) -> super::parser::Parser {
        p.add_after(AfterFolder)
    }
}

impl Chain for AfterFolder {
    type Input = Recorder;
    type Err = Error;

    fn chain_expr_binary(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprBinary,
    ) -> Result<syn::Expr, Self::Err> {
        // check op
        let op = match t.op {
            syn::BinOp::Add(_) => "add",
            syn::BinOp::Sub(_) => "sub",
            syn::BinOp::Mul(_) => "mul",
            syn::BinOp::Div(_) => "div",
            // syn::BinOp::AddEq(_) => todo!(),
            // syn::BinOp::SubEq(_) => todo!(),
            // syn::BinOp::MulEq(_) => todo!(),
            // syn::BinOp::DivEq(_) => todo!(),
            _ => return Err(Error::new(t.op.span(), "unsupport Op")),
        };

        let parent = c
            .peek_stack()
            .ok_or(Error::new(t.span(), "cannot find node when fold binary"))?;

        let grad_fn = grad_method(op);

        let mut destruct = vec![];
        for i in c.graph.to_edges(parent) {
            destruct.push(i.to_ident())
        }

        let right = c
            .graph
            .to_node(
                c.graph
                    .to_edges(parent)
                    .nth(1)
                    .ok_or(Error::new(t.span(), "not find right node"))?,
            )
            .to_ident();

        let ast = parse_quote! {
            {
                let mady_tmp;
                (mady_tmp, (#(#destruct),*)) = #grad_fn(#right);
                mady_tmp
            }
        };
        Ok(ast)
    }

    // fn chain_expr_methodcall(&mut self, c: &mut Self::Input, t: syn::ExprMethodCall) -> Result<syn::Expr, Self::Err> {

    //     let parent = c
    //         .peek_stack()
    //         .ok_or(Error::new(t.span(), "cannot find node when fold binary"))?;

    //     let grad_fn = Marker::new_method(
    //         op,
    //         parent,
    //         c.graph
    //             .to_edges(parent)
    //             .map(|x| c.graph.to_node(x))
    //             .collect(),
    //     )
    //     .grad_fn();

    //     let mut destruct = vec![];
    //     for i in c.graph.to_edges(parent) {
    //         destruct.push(i.to_ident())
    //     }

    //     let right = c
    //         .graph
    //         .to_node(
    //             c.graph
    //                 .to_edges(parent)
    //                 .nth(1)
    //                 .ok_or(Error::new(t.span(), "not find right node"))?,
    //         )
    //         .to_ident();

    //     let ast = parse_quote! {
    //         {
    //             let mady_tmp;
    //             (mady_tmp, (#(#destruct),*)) = #grad_fn(#right);
    //             mady_tmp
    //         }
    //     };
    //     Ok(ast)

    // }
}
