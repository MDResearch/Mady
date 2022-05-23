use syn::{parse_quote, Error};

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
                stmts.push(parse_quote! {
                    let mut #ident;
                });
                for e in c.graph.to_edges(n) {
                    let edge = c.graph.edge_weight(e);
                    let ident = edge.to_ident();
                    stmts.push(parse_quote! {
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
    let mut tys = c.tys().into_iter().rev();
    for n in c.graph.topological_iter() {
        grads.insert(n);
        let node = c.graph.node_weight(n);
        match node {
            VarType::Tmp(v) | VarType::Grad(v) => {
                let node_ident = v.to_ident();
                if roots.contains(&n) {
                    let node_ty = tys.next().ok_or(ParseError::NotFindType.new(v.span()))?;
                    stmts.push(parse_quote! {
                        #node_ident = #node_ty::one();
                    });
                }
                for e in c.graph.to_edges(n) {
                    let edge = c.graph.edge_weight(e).to_ident();
                    let t = c.graph.to_node(e);
                    let to_node = c.graph.node_weight(t).id().to_ident();

                    if grads.contains(&t) {
                        stmts.push(parse_quote! {
                            #to_node += #node_ident.mady_chain(#edge).clone();
                        });
                    } else {
                        grads.insert(t);
                        stmts.push(parse_quote! {
                            #to_node = #node_ident.mady_chain(#edge).clone();
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

// pub fn _gen_types(c: &Recorder) -> Result<Vec<syn::Stmt>, Error> {
//     let mut stmts = vec![];
//     let roots = HashSet::<_>::from_iter(c.graph.roots());
//     let mut grads = HashSet::new();
//     for n in c.graph.topological_iter() {
//         grads.insert(n);
//         let node = c.graph.node_weight(n);
//         match node {
//             VarType::Tmp(v) | VarType::Grad(v) => {
//                 let ty = v.to_type_ident();
//                 let node_grad_ty = v.to_grad_type_ident();
//                 let annotate = v
//                     .ty()
//                     .clone()
//                     .ok_or(ParseError::CantInferType.new(node.span()))?;
//                 stmts.push(parse_quote! {
//                     type #ty = #annotate;
//                 });

//                 if roots.contains(&n) {
//                     stmts.push(parse_quote! {
//                         type #node_grad_ty = <#ty as One>::O0;
//                     });
//                 }

//                 for e in c.graph.to_edges(n) {
//                     let edge = c.graph.edge_weight(e);
//                     let ty = edge.to_type_ident();
//                     let annotate = edge
//                         .ty()
//                         .clone()
//                         .ok_or(ParseError::CantInferType.new(edge.span()))?;

//                     stmts.push(parse_quote! {
//                         type #ty = #annotate;
//                     });

//                     let to_node = c.graph.to_node(e);
//                     if !grads.contains(&to_node) {
//                         grads.insert(to_node);
//                         let grad_ty = c.graph.node_weight(to_node).id().to_grad_type_ident();

//                         stmts.push(parse_quote! {
//                             type #grad_ty = <#node_grad_ty as MadyChain<#ty>>::O0;
//                         });
//                     }
//                 }
//             }
//             VarType::Null => {}
//             _ => unimplemented!(),
//         }
//     }

//     Ok(stmts)
// }
