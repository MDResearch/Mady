use std::collections::LinkedList;
use std::error::Error;
use std::ops::Sub;
use std::str::FromStr;

use crate::graph::Graph;
use proc_macro2::{token_stream, Ident, TokenStream};
use quote::quote;
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

#[derive(Debug, Default)]
struct Variable {
    name: String,
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
    fn new_var(&mut self, name: String) -> usize {
        let index = self.variables.len();
        self.variables.push(Variable { name });
        index
    }

    fn new_tmp(&mut self) -> (String, usize) {
        let index = self.variables.len();
        let name = format!("mad_var_{}", index);
        (name.clone(), self.new_var(name))
    }

    /// add a var to stack block
    ///
    /// return usize mean the index in varibles
    fn new_loacl(&mut self, name: String) -> Result<usize, Box<dyn Error>> {
        let index = self.new_var(name);
        self.stack
            .last_mut()
            .ok_or("No Block in Stack")?
            .push_back(index);
        Ok(index)
    }

    // ! dont use
    /// gen a var by auto gen key
    ///
    /// like `mad_var_{auto gen key}`
    // fn push_var_key(&mut self) -> Result<(TokenStream, usize), Box<dyn Error>> {
    //     let index = self.variables.len();
    //     let name = format!("mad_var_{}", index);
    //     let var = TokenStream::from_str(name.as_str())?;
    //     Ok((var, self.new_var(name)))
    // }

    // ! dont use
    /// gen a var by auto gen key
    /// but it push it to stack
    /// like `mad_var_{auto gen key}`
    // fn push_loacl_key(&mut self) -> Result<(TokenStream, usize), Box<dyn Error>> {
    //     let index = self.variables.len();
    //     let name = format!("mad_var_{}", index);
    //     let var = TokenStream::from_str(name.as_str())?;
    //     Ok((var, self.new_loacl(name)?))
    // }

    fn find_local(&self, name: String) -> Option<usize> {
        self.stack
            .last()?
            .iter()
            .position(|&x| self.variables[x].name == name)
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
        let new_ident = parse_str::<Ident>(&format!("grads_{}", i.sig.ident))?;

        let g = Graph::<Ident, Ident>::new();
        for stmt in &i.block.stmts {}

        Ok(quote! {
            {
                #i

                fn #new_ident(){
                    todo!()
                }
            }
        })
    }

    fn build_graph(&mut self, e: Expr) -> Result<(usize, Expr), Expr> {
        match e {
            Expr::Binary(v) => {
                let (left, left_expr) = self.build_graph(*v.left)?;
                let (right, right_expr) = self.build_graph(*v.right)?;
                let (edge_left_name, edge_left) = self.new_tmp();
                let (edge_right_name, edge_right) = self.new_tmp();
                let (_, ops) = self.new_tmp();

                self.ad_graph.add_edge(edge_right, (ops, left));
                self.ad_graph.add_edge(edge_left, (ops, right));

                let edge_left_ts = TokenStream::from_str(edge_left_name.as_str()).unwrap();
                let edge_right_ts = TokenStream::from_str(edge_right_name.as_str()).unwrap();

                let ts = parse_quote! {
                    {
                        let tmp;
                        (tmp, (#edge_left_ts, #edge_right_ts)) = #left_expr.grad_add(#right_expr);
                        tmp
                    }
                };
                Ok((ops, ts))
            }
            Expr::Path(v) => {
                todo!()
                // Ok((,v))
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
    use syn::{parse2, Block};

    use super::{Fold, Parser};
    use quote::quote;

    // #[test]
    // fn test_expr() {
    //     let s = quote! {
    //         {
    //             a * b;
    //         }
    //     };
    //     let ast: Block = parse2(s).expect("unknow tokenstream");
    //     let s = quote! {
    //         {
    //             {
    //                 mad_var_0 = a.clone();
    //                 mad_var_1 = b.clone();
    //                 a * b
    //             };
    //         }
    //     };
    //     let res: Block = parse2(s).expect("unknow tokenstream");
    //     let mut parser = Parser::new();
    //     let ast = parser.fold_block(ast);
    //     assert_eq!(ast, res);
    // }

    #[test]
    fn assign_var() {
        let a = 10;
        a.clone();
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
