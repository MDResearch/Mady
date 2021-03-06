use syn::spanned::Spanned;
use syn::{parse_quote_spanned, Error};

use crate::error::ParseError;
use crate::gen::Chain;
use crate::generator::gen_backward;
use crate::utils::{grad_method, ops_to_string};

use crate::parser::{Parser, Recorder, Register};

#[derive(Default)]
pub struct Folder;

struct AfterFolder;

impl Folder {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Register for Folder {
    fn register(self, p: Parser) -> Parser {
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
            .ok_or(ParseError::NotFindNode.new(t.span()))?;

        let grad_fn = grad_method(
            ops_to_string(&t.op).ok_or(ParseError::UnsupportedSyntax.new(t.op.span()))?,
        );

        let mut destruct = vec![];
        for i in c.graph.to_edges(parent) {
            let var = c.graph.edge_weight(i);
            destruct.push(var.to_ident())
        }

        let span = t.span();
        let left = t.left;
        let right = t.right;
        let ast = parse_quote_spanned! {span=>
            {
                let mady_tmp;
                (mady_tmp, (#(#destruct),*)) = #left.#grad_fn(#right);
                mady_tmp
            }
        };
        Ok(ast)
    }

    fn chain_block(&mut self, c: &mut Self::Input, t: syn::Block) -> Result<syn::Block, Self::Err> {
        let stmt = if c.is_sig_level() {
            let outs = c
                .graph
                .grad_node()
                .into_iter()
                .map(|x| c.graph.node_weight(x).id().to_ident());
            let backward = gen_backward(c)?;
            parse_quote_spanned! {t.span()=>
                {
                    let _mady_return = {#t};
                    #(#backward)*
                    (_mady_return, (#(#outs),*))
                }
            }
        } else {
            t
        };
        Ok(stmt)
    }

    fn chain_returntype(
        &mut self,
        c: &mut Self::Input,
        t: syn::ReturnType,
    ) -> Result<syn::ReturnType, Self::Err> {
        let outs = c.tys().iter().take(c.tys().len() - 1);
        match t {
            syn::ReturnType::Default => todo!(),
            syn::ReturnType::Type(_, t) => Ok(parse_quote_spanned! {t.span()=>
                -> (#t, (#(#outs),*))
            }),
        }
    }

    fn chain_expr_methodcall(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprMethodCall,
    ) -> Result<syn::Expr, Self::Err> {
        let parent = c
            .peek_stack()
            .ok_or(ParseError::NotFindNode.new(t.span()))?;

        let mut destruct = vec![];
        for i in c.graph.to_edges(parent) {
            let var = c.graph.edge_weight(i);
            destruct.push(var.to_ident())
        }
        let mut t = t;
        t.method = grad_method(t.method);

        let ast = parse_quote_spanned! {t.span()=>
            {
                let mady_tmp;
                (mady_tmp, (#(#destruct),*)) = #t;
                mady_tmp
            }
        };
        Ok(ast)
    }

    fn chain_expr_call(
        &mut self,
        c: &mut Self::Input,
        t: syn::ExprCall,
    ) -> Result<syn::Expr, Self::Err> {
        let parent = c
            .peek_stack()
            .ok_or(ParseError::NotFindNode.new(t.span()))?;

        let mut destruct = vec![];
        for i in c.graph.to_edges(parent) {
            let var = c.graph.edge_weight(i);
            destruct.push(var.to_ident())
        }
        let mut t = t;
        if let syn::Expr::Path(ref mut p) = *t.func {
            if let Some(name) = p.path.segments.last_mut() {
                name.ident = grad_method(&name.ident);
            } else {
                return Err(ParseError::UnexpectError.new(p.span()));
            }
        } else {
            return Err(ParseError::UnsupportedSyntax.new(t.func.span()));
        }

        let ast = parse_quote_spanned! {t.span()=>
            {
                let mady_tmp;
                (mady_tmp, (#(#destruct),*)) = #t;
                mady_tmp
            }
        };
        Ok(ast)
    }
}
