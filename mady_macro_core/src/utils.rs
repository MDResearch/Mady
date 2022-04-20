use super::{Edge, Node};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

impl Node {
    pub fn to_ident(&self) -> Ident {
        format_ident!("mady_tmp_{}", self.index())
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

    pub fn to_string(&self) -> String {
        let index = self.index();
        format!("mady_tmp_{}_{}", index.0, index.1)
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
        let ty = method_node.to_ident();
        let tys = arg_nodes.into_iter().map(|x| x.to_ident());
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
        let tys = arg_nodes.iter().map(|x| x.to_ident());
        let ty_trait = format_ident!("GradFn{}", to_upper_camel_case(fn_name.to_string()));

        Self {
            ty_name: quote! {
                <#ty_trait<#(#tys),*>>
            },
        }
    }

    pub fn grad(&self, n: usize) -> TokenStream {
        let fn_name = self.ty_name.clone();
        let ident = format_ident!("G{}", n);
        quote! {
            #fn_name::#ident
        }
    }

    pub fn output(&self, n: usize) -> TokenStream {
        let fn_name = self.ty_name.clone();
        let ident = format_ident!("O{}", n);
        quote! {
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
