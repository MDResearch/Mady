use std::collections::{hash_map::DefaultHasher, LinkedList};
use std::error::Error;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

use crate::graph::{Graph, Node};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::fold::{
    fold_block, fold_expr, fold_expr_assign, fold_ident, fold_item_fn, fold_local, fold_pat,
    fold_stmt, Fold,
};
use syn::{
    parse2, parse_quote, parse_str, BinOp, Block, Expr, ExprAssign, ItemFn, Local, Pat, Stmt,
};

impl<N, E> Node<N, E>
where
    N: Display,
{
    fn ident(&self, graph: &Graph<N, E>) -> Ident {
        new_ident(self.value(graph))
    }
}

#[derive(Debug, Default, Clone)]
struct Parser {
    variables: Vec<Variable>,
    stack: Vec<LinkedList<usize>>,
    grads: Vec<usize>,
    // the index in self.varibles
    ad_graph: Graph<usize, usize>,
}

#[derive(Debug, Clone)]
struct Variable {
    hash: Option<u64>,
    grads: bool,
}

impl Parser {
    /// create new Parser
    ///
    /// use stack<list<Varible>> to record varible
    pub fn new() -> Self {
        Default::default()
    }

    fn new_tmp(&mut self) -> usize {
        let index = self.variables.len();
        self.variables.push(Variable {
            hash: None,
            grads: false,
        });
        index
    }

    fn new_tmp_node(&mut self) -> Node<usize, usize> {
        let index = self.new_tmp();
        let node = self.ad_graph.add_node(index);
        node
    }

    /// add a var to stack block
    ///
    /// return usize mean the index in varibles
    fn new_local_node<T>(&mut self, name: &T) -> Result<Node<usize, usize>, Box<dyn Error>>
    where
        T: Hash,
    {
        let index = self.variables.len();
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        self.variables.push(Variable {
            hash: Some(hasher.finish()),
            grads: false,
        });
        let node = self.ad_graph.add_node(index);
        self.stack
            .last_mut()
            .ok_or("No Block in Stack")?
            .push_back(node.index());
        Ok(node)
    }

    fn new_grad_node<T>(&mut self, name: T) -> Result<Node<usize, usize>, Box<dyn Error>>
    where
        T: Hash,
    {
        let index = self.variables.len();
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        self.variables.push(Variable {
            hash: Some(hasher.finish()),
            grads: true,
        });
        let node = self.ad_graph.add_node(index);
        self.stack
            .last_mut()
            .ok_or("No Block in Stack")?
            .push_back(node.index());
        Ok(node)
    }

    fn find_local(&mut self, ident: &Ident) -> Option<Node<usize, usize>> {
        let mut hasher = DefaultHasher::new();
        ident.hash(&mut hasher);
        let hash = hasher.finish();

        for list in self.stack.iter().rev() {
            for &index in list.iter() {
                if self.variables[index].hash == Some(hash) {
                    return Some(Node::new(index));
                }
            }
        }
        None
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

    fn gen_vars(&self) -> (TokenStream, Vec<Ident>) {
        let mut ts = TokenStream::new();
        let mut grads = vec![];
        for (i, var) in self.variables.iter().enumerate() {
            let ident = new_ident(i);
            if var.grads {
                grads.push(ident.clone());
            }
            let stmt = quote! {
                #ident = Zero::zero();
            };
            ts.extend(stmt);
        }
        (ts, grads)
    }

    fn gen_backward(&self) -> TokenStream {
        // todo wait @Eason0729 complete fix graph
        unimplemented!("wait @Eason0729")
    }

    // cousming method
    fn gen_fn(mut self, i: ItemFn) -> ItemFn {
        let i = self.fold_item_fn(i);
        let block = i.block;
        let (vars, grads) = self.gen_vars();
        let block = parse_quote! {
            {
                #vars
                #block
            }
        };
        ItemFn { block, ..i }
    }
}

impl Parser {
    fn parse_expr(&mut self, e: Expr) -> Result<(Node<usize, usize>, Expr), Expr> {
        match e {
            // a + b
            //
            // `a + b` is Expr::Binary
            Expr::Binary(v) => {
                let ops_ts = match parse_ops_tokenstream(v.op) {
                    Ok(t) => t,
                    Err(_) => return Err(Expr::Binary(v)),
                };

                // parent->node(left->right)->edge(left->right)
                let ops = self.new_tmp_node();
                let (left, left_expr) = self
                    .parse_expr(*v.left.clone())
                    .map_err(|_| Expr::Binary(v.clone()))?;
                let (right, right_expr) = self
                    .parse_expr(*v.right.clone())
                    .map_err(|_| Expr::Binary(v.clone()))?;
                let edge_left = self.new_tmp();
                let edge_right = self.new_tmp();
                ops.link(&mut self.ad_graph, edge_left, &left);
                ops.link(&mut self.ad_graph, edge_right, &right);

                let edge_left_ident = new_ident(edge_left);
                let edge_right_ident = new_ident(edge_right);
                let ts = parse_quote! {
                    {
                        let tmp;
                        (tmp, (#edge_left_ident, #edge_right_ident)) = #left_expr.#ops_ts(#right_expr);
                        tmp
                    }
                };
                Ok((ops, ts))
            }

            // a = b + c
            //
            // a, b, c is Expr::Path
            Expr::Path(v) => {
                if let Some(ident) = v.path.get_ident() {
                    let id = self.find_local(ident).expect("not find varible in stack");
                    Ok((id, Expr::Path(v)))
                } else {
                    Err(Expr::Path(v))
                }
            }

            // a = b
            //
            // `=` is Expr::Assign
            Expr::Assign(v) => {
                let (parent, left) = self
                    .parse_expr(*v.left.clone())
                    .map_err(|_| Expr::Assign(v.clone()))?;
                let (child, right) = self
                    .parse_expr(*v.right.clone())
                    .map_err(|_| Expr::Assign(v.clone()))?;
                let edge = self.new_tmp();
                parent.link(&mut self.ad_graph, edge, &child);
                let edge_ident = new_ident(edge);

                let right = parse_quote! {
                    {
                        #edge_ident = #left.one();
                        #right
                    }
                };

                let ts = ExprAssign {
                    left: Box::new(left.clone()),
                    right: Box::new(right),
                    ..v
                };

                Ok((parent, Expr::Assign(ts)))
            }

            _ => Err(fold_expr(self, e)),
        }
    }

    fn parse_pat(&mut self, e: Pat) -> Result<(Node<usize, usize>, Pat), Pat> {
        match e {
            // let a;
            //
            // a is Pat::Ident
            //
            // expect some bug
            Pat::Ident(v) => {
                if let Ok(node) = self.new_local_node(&v.ident) {
                    Ok((node, Pat::Ident(v)))
                } else {
                    Err(Pat::Ident(v))
                }
            }
            _ => Err(fold_pat(self, e)),
        }
    }

    fn parse_stmt(&mut self, e: Stmt) -> Result<(Node<usize, usize>, Stmt), Stmt> {
        match e {
            Stmt::Local(v) => {
                let (parent, left) = self
                    .parse_pat(v.pat.clone())
                    .map_err(|_| Stmt::Local(v.clone()))?;
                if let Some((eq, expr)) = v.init.clone() {
                    let (child, right) =
                        self.parse_expr(*expr).map_err(|_| Stmt::Local(v.clone()))?;
                    let edge = self.new_tmp();
                    parent.link(&mut self.ad_graph, edge, &child);
                    let edge_ident = new_ident(edge);
                    let right = parse_quote! {
                        {
                            #edge_ident = #left.one();
                            #right
                        }
                    };
                    let ts = Local {
                        pat: left,
                        init: Some((eq, Box::new(right))),
                        ..v
                    };
                    Ok((parent, Stmt::Local(ts)))
                } else {
                    Ok((parent, Stmt::Local(Local { pat: left, ..v })))
                }
            }

            // todo `const`
            Stmt::Item(v) => todo!(),
            Stmt::Expr(v) => self
                .parse_expr(v)
                .map(|(id, expr)| (id, Stmt::Expr(expr)))
                .map_err(|expr| Stmt::Expr(expr)),
            Stmt::Semi(v, t) => match self.parse_expr(v) {
                Ok((id, expr)) => Ok((id, Stmt::Semi(expr, t))),
                Err(expr) => Err(Stmt::Semi(expr, t)),
            },
        }
    }
}

impl Fold for Parser {
    fn fold_pat(&mut self, i: Pat) -> Pat {
        match self.parse_pat(i) {
            Ok((.., v)) => v,
            Err(v) => v,
        }
    }

    fn fold_expr(&mut self, i: Expr) -> Expr {
        match self.parse_expr(i) {
            Ok((.., v)) => v,
            Err(v) => v,
        }
    }

    fn fold_stmt(&mut self, i: Stmt) -> Stmt {
        match self.parse_stmt(i) {
            Ok((.., v)) => v,
            Err(v) => v,
        }
    }

    fn fold_block(&mut self, i: Block) -> Block {
        self.enter_block();
        let i = fold_block(self, i);
        self.exit_block().expect("should have block");
        i
    }
}

fn parse_ops_tokenstream(op: BinOp) -> Result<TokenStream, ()> {
    match op {
        BinOp::Add(_) => Ok(quote! {grad_add}),
        BinOp::Sub(_) => Ok(quote! {grad_sub}),
        BinOp::Mul(_) => Ok(quote! {grad_mul}),
        BinOp::Div(_) => Ok(quote! {grad_div}),
        _ => Err(()),
    }
}

fn new_ident<T>(name: T) -> Ident
where
    T: Display,
{
    Ident::new(format!("mady_{}", name).as_str(), Span::call_site())
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::{Fold, Parser};
    use quote::quote;

    #[test]
    fn test_expr_binary() {
        let ast = parse_quote! {
            a + b
        };
        let res = parse_quote! {
            {
                let tmp;
                (tmp, (mady_3, mady_4)) = a.grad_add(b);
                tmp
            }
        };
        let mut parser = Parser::new();
        parser.enter_block();
        parser.new_local_node(&"a").unwrap();
        parser.new_local_node(&"b").unwrap();
        let (_, ast) = parser.parse_expr(ast).unwrap();
        assert_eq!(ast, res);
    }

    #[test]
    fn test_expr_assign() {
        let ast = parse_quote! {
            {
                c = a - b;
            }
        };
        let res = parse_quote! {
            {
                c = {
                    mady_6 = c.one();
                    {
                        let tmp;
                        (tmp, (mady_4, mady_5)) = a.grad_sub(b);
                        tmp
                    }
                };
            }
        };
        let mut parser = Parser::new();
        parser.enter_block();
        parser.new_local_node(&"a").unwrap();
        parser.new_local_node(&"b").unwrap();
        parser.new_local_node(&"c").unwrap();
        let ast = parser.fold_expr(ast);
        assert_eq!(ast, res);
    }

    #[test]
    fn test_expr_local_declare() {
        let ast = parse_quote! {
            {
                let (a, b)=(1, 2);
                let c;
                c = a * b;
            }
        };
        let res = parse_quote! {
            {
                let (a, b)=(1, 2);
                let c;
                c = {
                    mady_6 = c.one();
                    {
                        let tmp;
                        (tmp, (mady_4, mady_5)) = a.grad_mul(b);
                        tmp
                    }
                };
            }
        };
        let mut parser = Parser::new();
        let ast = parser.fold_expr(ast);
        assert_eq!(ast, res);
    }

    #[test]
    fn test_expr_local_assign() {
        let ast = parse_quote! {
            {
                let (a, b)=(1, 2);
                let c = a / b;
            }
        };
        let res = parse_quote! {
            {
                let (a, b)=(1, 2);
                let c = {
                    mady_6 = c.one();
                    {
                        let tmp;
                        (tmp, (mady_4, mady_5)) = a.grad_div(b);
                        tmp
                    }
                };
            }
        };
        let mut parser = Parser::new();
        let ast = parser.fold_expr(ast);
        assert_eq!(ast, res);
    }

    #[test]
    fn test_gen_var() {
        // let mut parser = Parser::new();
        // parser.new_tmp();
        // parser.new_tmp()

        // quote!{

        // }
        // assert_eq!(parser.gen_vars(), 10);
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
