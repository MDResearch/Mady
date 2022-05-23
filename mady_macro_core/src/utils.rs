use crate::graph::Node;
use crate::parser::{Id, ParseGraph, Var, VarType};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use syn::{parse_quote, TypePath};

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

pub fn to_upper_camel_case(name: String) -> String {
    name.split('_')
        .map(|x| format!("{}{}", x[..1].to_uppercase(), &x[1..]))
        .collect::<String>()
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
