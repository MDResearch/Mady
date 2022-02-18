use std::collections::{hash_map::DefaultHasher, LinkedList};
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::ops::Sub;
use std::str::FromStr;

use crate::graph::Graph;
use proc_macro2::{token_stream, Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::fold::{fold_block, fold_expr, fold_expr_assign, fold_ident, Fold};
use syn::{
    parse2, parse_quote, parse_str, BinOp, Block, Expr, ExprAssign, ExprAssignOp, ItemFn, Local,
};

#[derive(Debug, Default)]
struct Parser {
    variables: Vec<Variable>,
    stack: Vec<LinkedList<usize>>,
    grads: Vec<usize>,
    // the index in self.varibles
    ad_graph: Graph<usize, usize>,
    parents: Vec<usize>,
}

#[derive(Debug)]
struct Variable {
    hash: u64,
    ident: Ident,
}

impl Parser {
    /// create new Parser
    ///
    /// use stack<list<Varible>> to record varible
    pub fn new() -> Self {
        Default::default()
    }

    /// add a var to varibles
    ///
    /// return usize mean the index in varibles
    fn new_var<T>(&mut self, name: T) -> (Ident, usize)
    where
        T: Into<String>,
    {
        let index = self.variables.len();
        let ident = Ident::new(name.into().as_str(), Span::call_site());
        let mut hasher = DefaultHasher::new();
        ident.hash(&mut hasher);
        self.variables.push(Variable {
            hash: hasher.finish(),
            ident: ident.clone(),
        });
        (ident, index)
    }

    fn new_tmp(&mut self) -> (Ident, usize) {
        let index = self.variables.len();
        let name = format!("mad_var_{}", index);
        self.new_var(name)
    }

    /// add a var to stack block
    ///
    /// return usize mean the index in varibles
    fn new_local<T>(&mut self, name: T) -> Result<(Ident, usize), Box<dyn Error>>
    where
        T: Into<String>,
    {
        let (ident, index) = self.new_var(name);
        self.stack
            .last_mut()
            .ok_or("No Block in Stack")?
            .push_back(index);
        Ok((ident, index))
    }

    fn find_local(&self, ident: &Ident) -> Option<usize> {
        let mut hasher = DefaultHasher::new();
        ident.hash(&mut hasher);
        let hash = hasher.finish();
        self.stack
            .last()?
            .iter()
            .position(|&x| self.variables[x].hash == hash)
    }

    // ! dont use
    // fn link_nodes(&mut self, edge_var_id: usize, from_to: (usize, usize)) -> Result<(), ()> {
    //     self.ad_graph.add_edge(
    //         edge_var_id,
    //         (
    //             self.find_local(from_to.0).ok_or(())?,
    //             self.find_local(from_to.1).ok_or(())?,
    //         ),
    //     );
    //     Ok(())
    // }

    /// enter a block
    ///
    /// push link
    fn enter_block(&mut self) {
        self.stack.push(LinkedList::new());
    }

    /// exit a block
    ///
    /// this will pop stack make var in top block unfindable
    fn exit_block(&mut self) -> Result<(), Box<dyn Error>> {
        self.stack.pop().ok_or("cannot exit block")?;
        Ok(())
    }

    fn grad_parse(input: TokenStream) -> Result<TokenStream, Box<dyn Error>> {
        let i = parse2::<ItemFn>(input)?;

        // generate new ident
        let new_ident = parse_str::<Ident>(&format!("grads_{}", i.sig.ident))?;

        //replace return ... with return '(tuple)'($de_arg1,de_arg2...)
        todo!();

        let mut parser = Parser::new();

        let result = parser.fold_block(*i.block);

        Ok(quote! {
            {
                #input

                fn #new_ident(){
                    #result
                }
            }
        })
    }

    fn build_graph(&mut self, e: Expr) -> Result<(usize, Expr), Expr> {
        match e {
            Expr::Binary(v) => {
                let (left, left_expr) = self.build_graph(*v.left)?;
                let (right, right_expr) = self.build_graph(*v.right)?;
                let (edge_left_ident, edge_left) = self.new_tmp();
                let (edge_right_ident, edge_right) = self.new_tmp();
                let (_, ops) = self.new_tmp();
                let node_ops = self.ad_graph.add_node(ops);

                self.ad_graph.add_edge(edge_right, (node_ops, left));
                self.ad_graph.add_edge(edge_left, (node_ops, right));

                let ts = parse_quote! {
                    {
                        let tmp;
                        (tmp, (#edge_left_ident, #edge_right_ident)) = #left_expr.grad_add(#right_expr);
                        tmp
                    }
                };
                Ok((ops, ts))
            }
            Expr::Path(v) => {
                if let Some(ident) = v.path.get_ident() {
                    let id = self.find_local(ident).expect("not find varible in stack");
                    Ok((id, Expr::Path(v)))
                } else {
                    Err(Expr::Path(v))
                }
            }
            Expr::Let(v) => {
                todo!()
            }
            _ => Err(fold_expr(self, e)),
        }
    }
}

impl Fold for Parser {
    fn fold_expr_assign(&mut self, i: ExprAssign) -> ExprAssign {
        todo!()
    }

    fn fold_expr_assign_op(&mut self, i: ExprAssignOp) -> ExprAssignOp {
        todo!()
    }

    /// record varible
    ///
    /// ```
    /// let (mut a, mut b, mut c)=(0, 0, 0);
    /// a = b * c;
    /// // covert to
    /// a = {
    ///     let mad_var_id = b * c;
    ///     mad_var_id
    /// };
    /// ```
    fn fold_expr(&mut self, i: Expr) -> syn::Expr {
        match i {
            Expr::Assign(stat) => {
                let parent = *stat.left;

                todo!()
            }
            Expr::Binary(v) => {
                let op = v.op;

                match op {
                    BinOp::Add(_) => {
                        panic!("deprecated");
                        let left = self.fold_expr(*v.left);
                        // let right = self.fold_expr(*v.right);

                        // let (edge_left, edge_left_id) =
                        //     self.push_var_key().expect("no stack to push");
                        // let (edge_right, edge_right_id) =
                        //     self.push_var_key().expect("no stack to push");
                        // let (ops, ops_id) = self.push_var_key().expect("no stack to push");

                        // let ops_node = self.ad_graph.add_node(ops_id);

                        // // self.link_nodes(edge_right_id, (ops_node, 0));
                        // // self.link_nodes(edge_left_id, (ops_node, 0));

                        // parse_quote! {
                        //     {
                        //         let tmp;
                        //         (tmp, (#edge_left, #edge_right)) = #left.grad_add(#right);
                        //         tmp
                        //     }
                        // }
                    }
                    // BinOp::Sub(_) => {
                    //     parse_quote! {
                    //         {
                    //             #left_var = #left.clone();
                    //             #right_var = #right.clone();
                    //             #left #op #right
                    //         }
                    //     }
                    // }
                    // BinOp::Mul(_) => {
                    //     parse_quote! {
                    //         {
                    //             #left_var = #left.clone();
                    //             #right_var = #right.clone();
                    //             #left #op #right
                    //         }
                    //     }
                    // }
                    // BinOp::Div(_) => {
                    //     parse_quote! {
                    //         {
                    //             #left_var = #left.clone();
                    //             #right_var = #right.clone();
                    //             #left #op #right
                    //         }
                    //     }
                    // }
                    _ => Expr::Binary(v),
                }
            }
            _ => fold_expr(self, i),
        }
    }

    fn fold_local(&mut self, i: Local) -> Local {
        for attr in i.attrs {
            // something like this
            //
            // ```
            // #[grad]
            // a = 10;
            // ```
            if attr.path.is_ident("grad") {
                // add grad
                match i.pat {
                    syn::Pat::Ident(_) => todo!(),
                    _ => {}
                }
                unimplemented!();
                break;
            }
        }
        todo!()
    }

    fn fold_block(&mut self, i: Block) -> Block {
        self.enter_block();
        let i = fold_block(self, i);
        self.exit_block().expect("should have block");
        i
    }
}

#[cfg(test)]
mod tests {
    use syn::{parse2, Expr};

    use super::{Fold, Parser};
    use quote::quote;

    #[test]
    fn test_expr_binary() {
        let s = quote! {
            a + b
        };
        let ast: Expr = parse2(s).expect("unknow tokenstream");
        let s = quote! {
            {
                let tmp;
                (tmp, (mad_var_2, mad_var_3)) = a.grad_add(b);
                tmp
            }
        };
        let res: Expr = parse2(s).expect("unknow tokenstream");
        let mut parser = Parser::new();
        parser.enter_block();
        let (.., a) = parser.new_local("a").unwrap();
        let (.., b) = parser.new_local("b").unwrap();
        parser.ad_graph.add_node(a);
        parser.ad_graph.add_node(b);
        let (top, ast) = parser.build_graph(ast).unwrap();
        assert_eq!(ast, res);

        // idk, is my fault or it's a bug /cc @Eason0729
        // assert_eq!(&top, parser.ad_graph.roots().first().unwrap());
    }
}

/*
ast = Local(
    Local {
        attrs: [],
        let_token: Let,
        pat: Ident(
            PatIdent {
                attrs: [],
                by_ref: None,
                mutability: None,
                ident: Ident(
                    r,
                ),
                subpat: None,
            },
        ),
        init: Some(
            (
                Eq,
                Binary(
                    ExprBinary {
                        attrs: [],
                        left: Path(
                            ExprPath {
                                attrs: [],
                                qself: None,
                                path: Path {
                                    leading_colon: None,
                                    segments: [
                                        PathSegment {
                                            ident: Ident(
                                                a,
                                            ),
                                            arguments: None,
                                        },
                                    ],
                                },
                            },
                        ),
                        op: Mul(
                            Star,
                        ),
                        right: Path(
                            ExprPath {
                                attrs: [],
                                qself: None,
                                path: Path {
                                    leading_colon: None,
                                    segments: [
                                        PathSegment {
                                            ident: Ident(
                                                b,
                                            ),
                                            arguments: None,
                                        },
                                    ],
                                },
                            },
                        ),
                    },
                ),
            ),
        ),
        semi_token: Semi,
    },
)
*/
