use crate::{
    graph::{Edge, Graph, Node},
    parser::{Var, VarType},
};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use syn::{parse_quote, TypePath};

impl Node {
    pub fn to_ident(&self) -> Ident {
        format_ident!("mady_tmp_{}", self.index())
    }

    pub fn to_type_ident(&self) -> Ident {
        format_ident!("mady_ty_{}", self.index())
    }

    pub fn to_grad_type_ident(&self) -> Ident {
        format_ident!("mady_gty_{}", self.index())
    }

    pub fn to_string(&self) -> String {
        format!("mady_tmp_{}", self.index())
    }
}

impl Edge {
    pub fn to_ident(&self) -> Ident {
        let index = self.index();
        format_ident!("mady_tmp_{}_{}", index.0, index.1)
    }

    pub fn to_type_ident(&self) -> Ident {
        let index = self.index();
        format_ident!("mady_ty_{}_{}", index.0, index.1)
    }

    pub fn to_grad_type_ident(&self) -> Ident {
        let index = self.index();
        format_ident!("mady_gty_{}_{}", index.0, index.1)
    }

    pub fn to_string(&self) -> String {
        let index = self.index();
        format!("mady_tmp_{}_{}", index.0, index.1)
    }
}

impl Graph<Var, Var> {
    pub fn out_nodes(&self) -> Vec<Node> {
        self.nodes()
            .into_iter()
            .filter(|x| self.node_weight(*x).ty() == &VarType::Out)
            .collect()
    }
}

pub fn to_upper_camel_case(name: String) -> String {
    name.split('_')
        .map(|x| format!("{}{}", x[..1].to_uppercase(), &x[1..]))
        .collect::<String>()
}

pub struct Marker {
    ty_name: TokenStream,
}

impl Marker {
    pub fn new_method<T>(method_name: T, method_node: Node, arg_nodes: Vec<Node>) -> Self
    where
        T: ToString,
    {
        let ty = method_node.to_type_ident();
        let tys = arg_nodes.into_iter().map(|x| x.to_type_ident());
        let ty_trait = format_ident!("Grad{}", to_upper_camel_case(method_name.to_string()));

        Self {
            ty_name: quote! {
                <#ty as #ty_trait<#(#tys),*>>
            },
        }
    }

    pub fn new_call<T>(fn_name: T, arg_nodes: &Vec<Node>) -> Self
    where
        T: ToString,
    {
        let tys = arg_nodes.iter().map(|x| x.to_type_ident());
        let ty_trait = format_ident!("GradFn{}", to_upper_camel_case(fn_name.to_string()));

        Self {
            ty_name: quote! {
                <#ty_trait<#(#tys),*>>
            },
        }
    }

    pub fn grad(&self, n: usize) -> TypePath {
        let fn_name = self.ty_name.clone();
        let ident = format_ident!("G{}", n);
        parse_quote! {
            #fn_name::#ident
        }
    }

    pub fn output(&self, n: usize) -> TypePath {
        let fn_name = self.ty_name.clone();
        let ident = format_ident!("O{}", n);
        parse_quote! {
            #fn_name::#ident
        }
    }
}

pub fn grad_method<T>(method_name: T) -> Ident
where
    T: ToString,
{
    format_ident!("grad_{}", method_name.to_string())
}

pub fn grad_call<T>(fn_name: T) -> Ident
where
    T: ToString,
{
    format_ident!("grad_fn_{}", fn_name.to_string())
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
