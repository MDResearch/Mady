use syn::{parse_quote, Error};

use crate::parser::{Recorder, VarType};

use std::collections::HashSet;

pub fn gen_declare(c: &Recorder) -> Result<Vec<syn::Stmt>, Error> {
    let mut stmts = vec![];
    for n in c.graph.nodes() {
        let node = c.graph.node_weight(n);
        let ident = n.to_ident();
        match node.ty() {
            VarType::Grad | VarType::Out => {
                let ty = n.to_grad_type_ident();
                stmts.push(parse_quote! {
                    let mut #ident: #ty;
                });
            }
            VarType::TMP => unimplemented!("node tmp RFU"),
            VarType::IF | VarType::IFEL => unimplemented!("logic WIP"),
        }

        for e in c.graph.to_edges(n) {
            let edge = c.graph.edge_weight(e);
            let ident = e.to_ident();
            match edge.ty() {
                VarType::TMP => {
                    let ty = e.to_type_ident();
                    stmts.push(parse_quote! {
                        let #ident: #ty;
                    });
                }
                VarType::Grad | VarType::Out => unimplemented!("edge grad RFU"),
                VarType::IF | VarType::IFEL => unimplemented!("logic WIP"),
            }
        }
    }
    Ok(stmts)
}

pub fn gen_backward(c: &Recorder) -> Result<Vec<syn::Stmt>, Error> {
    let mut stmts = vec![];
    let roots = HashSet::<_>::from_iter(c.graph.roots());
    let mut grads = HashSet::new();
    for n in c.graph.topological_iter() {
        grads.insert(n);
        let node = n.to_ident();
        let node_ty = n.to_type_ident();
        if roots.contains(&n) {
            stmts.push(parse_quote! {
                #node = #node_ty::one();
            });
        }

        for e in c.graph.to_edges(n) {
            let edge = e.to_ident();
            let t = c.graph.to_node(e);
            let to_node = t.to_ident();

            if grads.contains(&t) {
                stmts.push(parse_quote! {
                    #to_node += #node.mady_chain(#edge);
                });
            } else {
                grads.insert(t);
                stmts.push(parse_quote! {
                    #to_node = #node.mady_chain(#edge);
                });
            }
        }
    }

    Ok(stmts)
}

pub fn gen_types(c: &Recorder) -> Result<Vec<syn::Stmt>, Error> {
    let mut stmts = vec![];
    let roots = HashSet::<_>::from_iter(c.graph.roots());
    let mut grads = HashSet::new();
    for n in c.graph.topological_iter() {
        grads.insert(n);
        let ty = n.to_type_ident();
        let node = c.graph.node_weight(n);
        let node_grad_ty = n.to_grad_type_ident();
        let annotate = node.annotate().clone().ok_or(Error::new(
            node.span(),
            "cannot infer type, please add type here",
        ))?;
        stmts.push(parse_quote! {
            type #ty = #annotate;
        });

        if roots.contains(&n) {
            stmts.push(parse_quote! {
                type #node_grad_ty = <#ty as One>::O0;
            });
        }

        for e in c.graph.to_edges(n) {
            let ty = e.to_type_ident();
            let edge = c.graph.edge_weight(e);
            let annotate = edge.annotate().clone().ok_or(Error::new(
                edge.span(),
                "cannot infer type, please add type here",
            ))?;

            stmts.push(parse_quote! {
                type #ty = #annotate;
            });

            let to_node = c.graph.to_node(e);
            if !grads.contains(&to_node) {
                grads.insert(to_node);
                let grad_ty = to_node.to_grad_type_ident();

                stmts.push(parse_quote! {
                    type #grad_ty = <#node_grad_ty as MadyChain<#ty>>::O0;
                });
            }
        }
    }

    Ok(stmts)
}
