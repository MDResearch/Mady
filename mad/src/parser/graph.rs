use std::collections::LinkedList;
use std::error::Error;
use std::ops::Sub;
use std::str::FromStr;

use crate::graph::Graph;
use proc_macro2::TokenStream;
use quote::quote;
use syn::fold::{fold_block, fold_expr, fold_expr_assign, Fold};
use syn::{parse2, parse_quote, BinOp, Block, Expr, ExprAssign, ExprAssignOp, ExprBinary, Local};

#[derive(Debug, Default)]
struct Parser {
    variables: Vec<Variable>,
    stack: Vec<LinkedList<usize>>,
    grads: Vec<usize>,
    ad_graph: Graph<Node, usize>,
}

#[derive(Debug, Default)]
struct Variable {
    name: String,
}

#[derive(Debug)]
enum Node {
    Opt(Operator),
    Var(usize),
}

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Parser {
    /// create new Parser
    ///
    /// use stack<list<Varible>> to record varible
    pub fn new() -> Self {
        Default::default()
    }

    /// add a var to current stack block
    fn push_var(&mut self, name: String) -> Result<usize, Box<dyn Error>> {
        let index = self.variables.len();
        self.stack
            .last_mut()
            .ok_or("No Block in Stack")?
            .push_back(index);
        self.variables.push(Variable { name });
        // !rewrite var
        let index = self.ad_graph.add_node(Node::Var(index));
        Ok(index)
    }

    /// gen a var by auto gen key
    ///
    /// like `mad_var_{auto gen key}`
    fn push_var_key(&mut self) -> Result<(TokenStream, usize), Box<dyn Error>> {
        let index = self.variables.len();
        let name = format!("mad_var_{}", index);
        let var = TokenStream::from_str(name.as_str())?;
        Ok((var, self.push_var(name)?))
    }

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
    /// `
    /// a = b * c;
    /// // covert to
    /// a = {
    ///     let mad_var_id = b * c;
    ///     mad_var_id
    /// };
    /// `
    fn fold_expr(&mut self, i: Expr) -> syn::Expr {
        match i {
            Expr::Binary(v) => {
                let left = self.fold_expr(*v.left);
                let right = self.fold_expr(*v.right);
                let op = v.op;
                let (left_var, left_id) = self.push_var_key().expect("no stack to push");
                let (right_var, right_id) = self.push_var_key().expect("no stack to push");

                // todo wait @Eason0729 check add_edge(from, to)
                match op {
                    BinOp::Add(_) => {
                        // self.ad_graph.add_edge(value, nodes);
                        // self.ad_graph.add_edge(value, nodes);
                    }
                    BinOp::Sub(_) => {}
                    BinOp::Mul(_) => {}
                    BinOp::Div(_) => {}
                    _ => {}
                }
                parse_quote! {
                    {
                        #left_var = #left.clone();
                        #right_var = #right.clone();
                        #left #op #right
                    }
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

    #[test]
    fn test_expr() {
        let s = quote! {
            {
                a * b;
            }
        };
        let ast: Block = parse2(s).expect("unknow tokenstream");
        let s = quote! {
            {
                {
                    mad_var_0 = a.clone();
                    mad_var_1 = b.clone();
                    a * b
                };
            }
        };
        let res: Block = parse2(s).expect("unknow tokenstream");
        let mut parser = Parser::new();
        let ast = parser.fold_block(ast);
        assert_eq!(ast, res);
    }

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
