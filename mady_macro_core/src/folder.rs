use syn::spanned::Spanned;
use syn::{parse_quote, Error};

use crate::gen::Chain;
use crate::generator::gen_backward;
use crate::utils::{grad_method, ops_to_string};

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
        let parent = c
            .peek_stack()
            .ok_or(Error::new(t.span(), "cannot find node when fold binary"))?;

        let grad_fn =
            grad_method(ops_to_string(&t.op).ok_or(Error::new(t.op.span(), "unsupport Op"))?);

        let mut destruct = vec![];
        for i in c.graph.to_edges(parent) {
            destruct.push(i.to_ident())
        }

        let left = t.left;
        let right = t.right;

        let ast = parse_quote! {
            {
                let mady_tmp;
                (mady_tmp, (#(#destruct),*)) = #left.#grad_fn(#right);
                mady_tmp
            }
        };
        Ok(ast)
    }

    fn chain_stmt_expr(
        &mut self,
        c: &mut Self::Input,
        t: syn::Expr,
    ) -> Result<syn::Stmt, Self::Err> {
        let stmt = if c.block_level() == 1 {
            let outs = c.graph.out_nodes().into_iter().map(|x| x.to_ident());
            let backward = gen_backward(c)?;
            parse_quote! {
                {
                    let mady_return = #t;
                    #(#backward)*
                    (mady_return, (#(#outs),*))
                }
            }
        } else {
            t
        };
        Ok(syn::Stmt::Expr(stmt))
    }

    fn chain_returntype(
        &mut self,
        c: &mut Self::Input,
        t: syn::ReturnType,
    ) -> Result<syn::ReturnType, Self::Err> {
        let outs = c
            .graph
            .out_nodes()
            .into_iter()
            .map(|x| x.to_grad_type_ident());
        match t {
            syn::ReturnType::Default => todo!(),
            syn::ReturnType::Type(_, t) => Ok(parse_quote! {
                -> (#t, (#(#outs),*))
            }),
        }
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
