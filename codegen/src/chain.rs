use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::str::FromStr;
use syn_codegen::{Data, Definitions, Node, Type, Variants};

pub fn gen(defs: &Definitions) -> TokenStream {
    let mut ts = TokenStream::new();
    for node in defs.types.iter() {
        ts.extend(visit_node(node));
    }
    quote! {
        /// chain of responsibility trait
        /// it is a call & return trait
        #[allow(unused)]
        pub trait Chain {
            type Input;
            type Err;
            #ts
        }
    }
}

fn visit_node(node: &Node) -> TokenStream {
    match &node.data {
        Data::Private | Data::Struct(..) => visit_private_and_struct(&node.ident),
        Data::Enum(map) => visit_enum(&node.ident, &map),
    }
}

fn visit_private_and_struct(ident: &str) -> TokenStream {
    let name = format_ident!("chain_{}", ident.to_lowercase());
    let ty = format_tokenstream!("syn::{}", ident);
    quote! {
        fn #name(&mut self, c: &mut Self::Input, t: #ty) -> Result<#ty, Self::Err> {
            Ok(t)
        }
    }
}

fn visit_enum(ident: &str, map: &Variants) -> TokenStream {
    let mut ts = TokenStream::new();
    let ty = format_tokenstream!("syn::{}", ident);
    // gen enum
    ts.extend(visit_private_and_struct(ident));

    // gen field
    for (field, tys) in map {
        if tys.is_empty() {
            continue;
        }
        let name = format_ident!("chain_{}_{}", ident.to_lowercase(), field.to_lowercase());
        let input_ty = gen_types(tys);
        let field_ty = format_tokenstream!("syn::{}::{}", ident, field);

        let destruct = if tys.len() == 1 {
            format_tokenstream!("t")
        } else {
            format_tokenstream!(
                "{}",
                (0..tys.len()).fold(String::new(), |a, x| format!("{} t.{},", a, x))
            )
        };

        ts.extend(quote! {
            fn #name(&mut self, c: &mut Self::Input, t: #input_ty) -> Result<#ty, Self::Err> {
                Ok(#field_ty(#destruct))
            }
        })
    }
    ts
}

fn gen_type(ty: &Type) -> TokenStream {
    match ty {
        Type::Syn(s) => format_tokenstream!("syn::{}", s),
        Type::Std(s) => format_tokenstream!("{}", s),
        Type::Ext(s) => format_tokenstream!("proc_macro2::{}", s),
        Type::Group(s) | Type::Token(s) => format_tokenstream!("syn::token::{}", s),
        Type::Tuple(tys) => gen_types(tys),
        Type::Box(ty) => format_tokenstream!("Box<{}>", gen_type(&*ty)),
        Type::Punctuated(_) | Type::Option(_) | Type::Vec(_) => {
            unimplemented!("unexpect : {:?}", ty)
        }
    }
}

fn gen_types(tys: &Vec<Type>) -> TokenStream {
    if tys.len() == 1 {
        gen_type(&tys[0])
    } else {
        let mut ts = TokenStream::new();
        for i in tys {
            let ty = gen_type(i);
            ts.extend(quote! {
                #ty,
            });
        }
        quote! {
            (#ts)
        }
    }
}
