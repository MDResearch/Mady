use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::str::FromStr;
use syn_codegen::{Data, Definitions, Fields, Node, Type, Variants};

pub fn gen(defs: &Definitions) -> TokenStream {
    let mut ts = TokenStream::new();
    for node in &defs.types {
        ts.extend(visit_node(node));
    }
    quote! {
        use super::ChainIter;

        /// fold chain of responsibility trait
        /// it is a before chain -> fold -> after chain trait
        #[allow(unused)]
        pub trait FoldChain
        where
            Self: ChainIter,
        {
            #ts
        }
    }
}

fn visit_node(node: &Node) -> TokenStream {
    match &node.data {
        Data::Private => visit_private(&node.ident),
        Data::Struct(map) => visit_struct(&node.ident, map),
        Data::Enum(map) => visit_enum(&node.ident, map),
    }
}

fn visit_private(ident: &str) -> TokenStream {
    let chain_name = format_ident!("chain_{}", ident.to_lowercase());
    let name = format_ident!("fold_chain_{}", ident.to_lowercase());
    let ty = format_tokenstream!("syn::{}", ident);
    quote! {
        fn #name(&mut self, c: &mut <Self as ChainIter>::Input, t: #ty) -> Result<#ty, <Self as ChainIter>::Err> {
            let mut t = t;
            for i in self.before() {
                t = i.#chain_name(c, t)?;
            }
            for i in self.after() {
                t = i.#chain_name(c, t)?;
            }
            Ok(t)
        }
    }
}

fn visit_struct(ident: &str, map: &Fields) -> TokenStream {
    let mut fold_ts = TokenStream::new();
    for (field_name, field_ty) in map {
        let field_name = format_tokenstream!("t.{}", field_name);
        let fold = gen_fold_type(&field_name.to_string(), field_ty);
        fold_ts.extend(quote! {
            #field_name = #fold;
        })
    }

    let chain_name = format_ident!("chain_{}", ident.to_lowercase());
    let name = format_ident!("fold_chain_{}", ident.to_lowercase());
    let ty = format_tokenstream!("syn::{}", ident);

    quote! {
        fn #name(&mut self, c: &mut <Self as ChainIter>::Input, t: #ty) -> Result<#ty, <Self as ChainIter>::Err> {
            let mut t = t;
            for i in self.before() {
                t = i.#chain_name(c, t)?;
            }
            #fold_ts
            for i in self.after() {
                t = i.#chain_name(c, t)?;
            }
            Ok(t)
        }
    }
}

fn visit_enum(ident: &str, map: &Variants) -> TokenStream {
    let mut ts = TokenStream::new();
    let ty = format_tokenstream!("syn::{}", ident);

    // gen enum
    {
        let mut fold_ts = TokenStream::new();
        for (field, tys) in map {
            if tys.is_empty() {
                continue;
            }

            let destructuring = if tys.len() == 1 {
                format_tokenstream!("tmp")
            } else {
                format_tokenstream!(
                    "{}",
                    (0..tys.len()).fold(String::new(), |a, x| format!("{} tmp_{},", a, x))
                )
            };

            let fold_name = format_ident!(
                "fold_chain_{}_{}",
                ident.to_lowercase(),
                field.to_lowercase()
            );

            let field_ty = format_tokenstream!("syn::{}::{}", ident, field);
            fold_ts.extend(quote! {
                #field_ty(#destructuring) => self.#fold_name(c, (#destructuring))?,
            });
        }

        fold_ts = quote! {
            t = match t {
                #fold_ts
                _ => t
            };
        };

        let name = format_ident!("fold_chain_{}", ident.to_lowercase());
        let chain_name = format_ident!("chain_{}", ident.to_lowercase());

        ts.extend(quote!{
            fn #name(&mut self, c: &mut <Self as ChainIter>::Input, t: #ty) -> Result<#ty, <Self as ChainIter>::Err> {
                let mut t = t;
                for i in self.before() {
                    t = i.#chain_name(c, t)?;
                }
                #fold_ts
                for i in self.after() {
                    t = i.#chain_name(c, t)?;
                }
                Ok(t)
            }
        });
    }

    // gen field
    for (field, tys) in map {
        if tys.is_empty() {
            continue;
        }
        let mut fold_ts = TokenStream::new();

        for (i, field_ty) in tys.iter().enumerate() {
            let field_name = if tys.len() == 1 {
                format_tokenstream!("t")
            } else {
                format_tokenstream!("t.{}", i)
            };

            let fold = gen_fold_type(&field_name.to_string(), field_ty);
            fold_ts.extend(quote! {
                #field_name = #fold;
            })
        }

        let structuring = if tys.len() == 1 {
            format_tokenstream!("t")
        } else {
            format_tokenstream!(
                "{}",
                (0..tys.len()).fold(String::new(), |a, x| format!("{} t.{},", a, x))
            )
        };

        let destructuring = if tys.len() == 1 {
            format_tokenstream!("tmp")
        } else {
            format_tokenstream!(
                "{}",
                (0..tys.len()).fold(String::new(), |a, x| format!("{} tmp_{},", a, x))
            )
        };

        let name = format_ident!(
            "fold_chain_{}_{}",
            ident.to_lowercase(),
            field.to_lowercase()
        );
        let chain_name = format_ident!("chain_{}_{}", ident.to_lowercase(), field.to_lowercase());
        let input_ty = gen_types(tys);
        let field_ty = format_tokenstream!("syn::{}::{}", ident, field);

        ts.extend(quote! {
            fn #name(&mut self, c: &mut <Self as ChainIter>::Input, t: #input_ty) -> Result<#ty, <Self as ChainIter>::Err> {
                let mut t = t;
                for i in self.before() {
                    t = match i.#chain_name(c, t)? {
                        #field_ty(#destructuring) => (#destructuring),
                        tmp => return Ok(tmp),
                    };
                }
                #fold_ts
                for i in self.after() {
                    t = match i.#chain_name(c, t)? {
                        #field_ty(#destructuring) => (#destructuring),
                        tmp => return Ok(tmp),
                    };
                }
                Ok(#field_ty(#structuring))
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

fn gen_fold_type(field: &String, ty: &Type) -> TokenStream {
    match ty {
        Type::Syn(s) => {
            if s == "Reserved" {
                format_tokenstream!("{}", field)
            } else {
                format_tokenstream!("self.fold_chain_{}(c, {})?", s.to_lowercase(), field)
            }
        }
        Type::Option(t) => {
            let field = format_tokenstream!("{}", field);
            let ts = gen_fold_type(&"o".to_string(), &*t);
            quote! {
                match #field {
                    Some(o) => Some(#ts),
                    None => None,
                }
            }
        }
        Type::Vec(t) => {
            let field = format_tokenstream!("{}", field);
            let ts = gen_fold_type(&"v".to_string(), &*t);
            quote! {
                {
                    let mut tmp = vec![];
                    for v in #field {
                        tmp.push(#ts);
                    }
                    tmp
                }
            }
        }
        Type::Tuple(t) => {
            let field = format_tokenstream!("{}", field);
            let list = t
                .iter()
                .enumerate()
                .map(|(i, x)| gen_fold_type(&format!("{}.{}", field, i), x));
            quote! {(#(#list),*,)}
        }
        Type::Box(t) => {
            let field = format_tokenstream!("{}", field);
            let ts = gen_fold_type(&format!("*{}", field), &*t);
            quote! {
                Box::new(#ts)
            }
        }
        _ => format_tokenstream!("{}", field),
    }
}
