use syn::{parse_quote_spanned, Error};

use crate::{
    error::ParseError,
    parser::{Recorder, VarType},
};

use std::collections::HashSet;

pub fn gen_declare(c: &Recorder) -> Result<Vec<syn::Stmt>, Error> {
    let mut stmts = vec![];
    for n in c.graph.nodes() {
        let node = c.graph.node_weight(n);
        match node {
            VarType::Tmp(v) | VarType::Grad(v) => {
                let ident = v.to_ident();
                stmts.push(parse_quote_spanned! {v.span()=>
                    let mut #ident;
                });
                for e in c.graph.to_edges(n) {
                    let edge = c.graph.edge_weight(e);
                    let ident = edge.to_ident();
                    stmts.push(parse_quote_spanned! {edge.span()=>
                        let #ident;
                    });
                }
            }
            VarType::IF(..) | VarType::IFEL(..) => unimplemented!("logic WIP"),
            VarType::Null => {}
        }
    }
    Ok(stmts)
}

pub fn gen_backward(c: &Recorder) -> Result<Vec<syn::Stmt>, Error> {
    let mut stmts = vec![];
    let roots = HashSet::<_>::from_iter(c.graph.roots());
    let mut grads = HashSet::new();
    let mut tys = c.tys().iter().rev();
    for n in c.graph.topological_iter() {
        grads.insert(n);
        let node = c.graph.node_weight(n);
        match node {
            VarType::Tmp(v) | VarType::Grad(v) => {
                let node_ident = v.to_ident();
                if roots.contains(&n) {
                    let node_ty = tys.next().ok_or(ParseError::NotFindType.new(v.span()))?;
                    stmts.push(parse_quote_spanned! {v.span()=>
                        #node_ident = <#node_ty>::mady_one();
                    });
                }
                for e in c.graph.to_edges(n) {
                    let edge = c.graph.edge_weight(e);
                    let edge_ident = edge.to_ident();
                    let t = c.graph.to_node(e);
                    let to_node = c.graph.node_weight(t).id().to_ident();

                    if grads.contains(&t) {
                        stmts.push(parse_quote_spanned! {edge.span()=>
                            #to_node += #node_ident.mady_chain(#edge_ident).clone();
                        });
                    } else {
                        grads.insert(t);
                        stmts.push(parse_quote_spanned! {edge.span()=>
                            #to_node = #node_ident.mady_chain(#edge_ident).clone();
                        });
                    }
                }
            }
            VarType::IF(..) | VarType::IFEL(..) => unimplemented!("logic WIP"),
            VarType::Null => {}
        }
    }

    Ok(stmts)
}
