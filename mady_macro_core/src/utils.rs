use crate::graph::Node;
use crate::parser::{ParseGraph, VarType};
use proc_macro2::Ident;
use quote::format_ident;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

impl ParseGraph {
    pub fn grad_node(&self) -> Vec<Node> {
        self.nodes()
            .into_iter()
            .filter(|x| {
                if let VarType::Grad(_) = self.node_weight(*x) {
                    true
                } else {
                    false
                }
            })
            .collect()
    }
}

pub fn grad_method<T>(method_name: T) -> Ident
where
    T: ToString,
{
    format_ident!("grad_{}", method_name.to_string())
}

pub fn ops_to_string(op: &syn::BinOp) -> Option<&'static str> {
    Some(match op {
        syn::BinOp::Add(_) => "add",
        syn::BinOp::Sub(_) => "sub",
        syn::BinOp::Mul(_) => "mul",
        syn::BinOp::Div(_) => "div",
        // syn::BinOp::AddEq(_) => todo!(),
        // syn::BinOp::SubEq(_) => todo!(),
        // syn::BinOp::MulEq(_) => todo!(),
        // syn::BinOp::DivEq(_) => todo!(),
        _ => return None,
    })
}

pub fn into_hash<T>(v: &T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    v.hash(&mut hasher);
    hasher.finish()
}
