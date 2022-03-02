use std::collections::{hash_map::DefaultHasher, LinkedList};
use std::error::Error;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

use crate::graph::{Edge, Graph, Node};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::fold::{fold_block, fold_expr, fold_pat, Fold};
use syn::{parse_quote, BinOp, Block, Expr, ExprAssign, ExprReturn, ItemFn, Local, Pat, Stmt};

impl<N, E> Node<N, E>
where
    N: Display,
{
    fn ident(&self, graph: &Graph<N, E>) -> Ident {
        new_ident(self.value(graph))
    }
}

impl<N, E> Edge<N, E>
where
    E: Display,
{
    fn ident(&self, graph: &Graph<N, E>) -> Ident {
        new_ident(self.value(graph))
    }
}

#[derive(Debug, Default, Clone)]
struct Parser {
    // local variables
    variables: Vec<Variable>,
    // temporary variables
    stack: Vec<LinkedList<usize>>,
    // graph
    ad_graph: Graph<usize, usize>,
}

// let a=(b+c)*d;
// -------------
// let tmp_00001=b+c;
// let a=tmp_00001*d;
// -------------
// tmp_00001 is tmp
// a,b,c,d is local

#[derive(Debug, Clone)]
struct Variable {
    hash: Option<u64>,
    ty: Ty,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Ty {
    TMP,
    GRAD,
    TOP,
    IF,
    IFEL,
    FOR,
    LOOP,
}

impl Parser {
    /// create new Parser
    ///
    /// use stack<list<Varible>> to record varible
    pub fn new() -> Self {
        let mut tmp: Self = Default::default();
        tmp.enter_block();
        tmp
    }

    fn new_tmp(&mut self) -> usize {
        let index = self.variables.len();
        self.variables.push(Variable {
            hash: None,
            ty: Ty::TMP,
        });
        index
    }

    fn new_tmp_node(&mut self) -> Node<usize, usize> {
        let index = self.new_tmp();

        self.ad_graph.add_node(index)
    }

    /// add a var to stack block
    ///
    /// return usize mean the index in varibles
    fn new_local_node<T>(&mut self, name: &T) -> Result<Node<usize, usize>, Box<dyn Error>>
    where
        T: Hash,
    {
        if self.stack.len() == 1 {
            return Err("No Block in Stack".into());
        }

        let index = self.variables.len();
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        self.variables.push(Variable {
            hash: Some(hasher.finish()),
            ty: Ty::TMP,
        });
        let node = self.ad_graph.add_node(index);
        self.stack.last_mut().unwrap().push_back(node.index());
        Ok(node)
    }

    fn new_grad_arg_node<T>(&mut self, name: T) -> Node<usize, usize>
    where
        T: Hash,
    {
        let index = self.variables.len();
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        self.variables.push(Variable {
            hash: Some(hasher.finish()),
            ty: Ty::GRAD,
        });
        let node = self.ad_graph.add_node(index);
        self.stack.first_mut().unwrap().push_back(node.index());

        node
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

    fn gen_vars(&mut self) -> TokenStream {
        // mark roots
        for i in self.ad_graph.roots() {
            let &index = Node::new(i).value(&self.ad_graph);
            self.variables[index].ty = Ty::TOP;
        }

        // gen tokenstream
        let mut ts = TokenStream::new();
        let _grads = TokenStream::new();
        for (i, var) in self.variables.iter().enumerate() {
            let ident = new_ident(i);

            let stmt = match var.ty {
                Ty::TMP => {
                    quote! {
                        let mut #ident = Zero::zero();
                    }
                }
                Ty::GRAD => {
                    quote! {
                        let mut #ident = Zero::zero();
                    }
                }
                Ty::TOP => {
                    quote! {
                        let #ident = One::one();
                    }
                }
                Ty::IF | Ty::IFEL | Ty::FOR | Ty::LOOP => {
                    quote! {
                        let #ident;
                    }
                }
            };
            ts.extend(stmt);
        }

        ts
    }

    fn gen_backward(&self) -> TokenStream {
        let mut ts = TokenStream::new();
        for node in self.ad_graph.topological_iter() {
            let node_ident = node.ident(&self.ad_graph);
            for edge in node.children(&self.ad_graph) {
                let edge_ident = edge.ident(&self.ad_graph);
                let to_ident = edge.to(&self.ad_graph).ident(&self.ad_graph);
                let stmt = quote! {
                    #to_ident += #node_ident.clone() * #edge_ident;
                };
                ts.extend(stmt);
            }
        }
        ts
    }

    fn gen_fn(&mut self, i: ItemFn) -> ItemFn {
        // gen grad var node
        self.fold_signature(i.sig.clone());

        // gen fn
        let block = self.fold_block(*i.block);
        let vars = self.gen_vars();
        let backward = self.gen_backward();
        let block = parse_quote! {
            {
                #vars
                #block
                #backward
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
                    left: Box::new(left),
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
                    // arg node
                    // use Err prevent unexpect linking
                    self.new_grad_arg_node(&v.ident);
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
            Stmt::Item(_v) => todo!(),
            Stmt::Expr(v) => self
                .parse_expr(v)
                .map(|(id, expr)| (id, Stmt::Expr(expr)))
                .map_err(Stmt::Expr),
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

    fn fold_expr_return(&mut self, i: ExprReturn) -> ExprReturn {
        let arg: Vec<_> = self
            .stack
            .first()
            .unwrap()
            .iter()
            .map(|&x| new_ident(x))
            .collect();
        let expr: Expr = match i.expr {
            Some(e) => {
                let tmp = fold_expr(self, *e);
                parse_quote! {
                    (#tmp, (#(#arg),*))
                }
            }
            None => {
                parse_quote! {
                    #(#arg),*
                }
            }
        };
        ExprReturn {
            expr: Some(Box::new(expr)),
            ..i
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
        let res = quote! {
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
        let ast = quote! {#ast};
        assert_eq!(ast.to_string(), res.to_string());
    }

    #[test]
    fn test_expr_assign() {
        let ast = parse_quote! {
            {
                c = a - b;
            }
        };
        let res = quote! {
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
        let ast = quote! {#ast};
        assert_eq!(ast.to_string(), res.to_string());
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
        let res = quote! {
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
        let ast = quote! {#ast};
        assert_eq!(ast.to_string(), res.to_string());
    }

    #[test]
    fn test_expr_local_assign() {
        let ast = parse_quote! {
            {
                let (a, b)=(1, 2);
                let c = a / b;
            }
        };
        let res = quote! {
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
        let ast = quote! {#ast};
        assert_eq!(ast.to_string(), res.to_string());
    }

    #[test]
    fn test_gen_var() {
        let mut parser = Parser::new();
        parser.enter_block();
        parser.new_tmp();
        let grad = parser.new_grad_arg_node("");
        let root = parser.new_tmp_node();
        root.link(&mut parser.ad_graph, 0, &grad);

        let ast = parser.gen_vars();

        let res = quote! {
            let mut mady_0 = Zero::zero();
            let mut mady_1 = Zero::zero();
            let mady_2 = One::one();
        };
        assert_eq!(ast.to_string(), res.to_string());
    }

    // #[test]
    // fn test_gen_backward() {
    //     // let mut parser = Parser::new();
    //     // parser.enter_block();
    //     // parser.new_tmp();
    //     // let grad = parser.new_grad_node("").unwrap();
    //     // let root = parser.new_tmp_node();
    //     // root.link(&mut parser.ad_graph, 0, &grad);

    //     // let ast = parser.gen_vars();

    //     // let res = quote! {
    //     //     let mady_0 = Zero::zero();
    //     //     let mady_1 = Zero::zero();
    //     //     let mady_2 = One::one();
    //     // };
    //     // assert_eq!(ast.0.to_string(), res.to_string());
    //     // assert_eq!(ast.1.to_string(), quote! {(mady_1,)}.to_string());
    // }
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
