use std::collections::{hash_map::DefaultHasher, LinkedList};
use std::error::Error;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

use crate::graph::{Edge, Graph, Node};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::fold::{fold_block, fold_expr, fold_pat, fold_signature, Fold};

use syn::{
    parse_quote, BinOp, Block, Expr, ExprAssign, ExprParen, ExprReturn, FnArg, ItemFn, Local, Pat,
    PatIdent, PatType, ReturnType, Signature, Stmt, Type,
};

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
struct InnerParser {
    grads: Vec<usize>,
    // temporary variables
    variables: Vec<Variable>,
    // local variables
    stack: Vec<LinkedList<usize>>,
    // graph
    graph: Graph<usize, usize>,
}

#[derive(Debug, Default, Clone)]
pub struct Parser {
    grads: Vec<(Ident, Type)>,
    return_ty: Vec<Type>,
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
    ty: Ty,
    hash: Option<u64>,
    marker: Option<Type>,
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

impl InnerParser {
    /// create new Parser
    ///
    /// use stack<list<Varible>> to record varible
    fn new<T, K>(grads: T) -> Self
    where
        T: IntoIterator<Item = (K, Type)>,
        K: Hash,
    {
        let mut tmp: Self = Default::default();
        for (hash, ty) in grads {
            tmp.new_grad_node(hash, ty);
        }
        tmp
    }

    fn gen(mut self, i: Block) -> Block {
        self.gen_block(i)
    }

    fn new_tmp(&mut self) -> usize {
        let index = self.variables.len();
        self.variables.push(Variable {
            ty: Ty::TMP,
            hash: None,
            marker: None,
        });
        index
    }

    fn new_tmp_node(&mut self) -> Node<usize, usize> {
        let index = self.new_tmp();

        self.graph.add_node(index)
    }

    /// add a var to stack block
    ///
    /// return usize mean the index in varibles
    fn new_local_node<T>(&mut self, name: &T) -> Result<Node<usize, usize>, Box<dyn Error>>
    where
        T: Hash,
    {
        if self.stack.is_empty() {
            return Err("No Block in Stack".into());
        }

        let index = self.variables.len();
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        self.variables.push(Variable {
            ty: Ty::TMP,
            hash: Some(hasher.finish()),
            marker: None,
        });
        let node = self.graph.add_node(index);
        self.stack.last_mut().unwrap().push_front(node.index());
        Ok(node)
    }

    fn new_grad_node<T>(&mut self, name: T, ty: Type) -> Node<usize, usize>
    where
        T: Hash,
    {
        let index = self.variables.len();
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        self.variables.push(Variable {
            ty: Ty::GRAD,
            hash: Some(hasher.finish()),
            marker: Some(ty),
        });
        let node = self.graph.add_node(index);
        self.grads.push(node.index());

        node
    }

    fn find_local(&mut self, ident: &Ident) -> Option<Node<usize, usize>> {
        let mut hasher = DefaultHasher::new();
        ident.hash(&mut hasher);
        let hash = hasher.finish();

        for list in self.stack.iter().rev() {
            for &index in list {
                if self.variables[index].hash == Some(hash) {
                    return Some(Node::new(index));
                }
            }
        }

        for &index in &self.grads {
            if self.variables[index].hash == Some(hash) {
                return Some(Node::new(index));
            }
        }

        None
    }

    fn node_marker(&self, node: Node<usize, usize>) -> &Option<Type> {
        &self.variables[*node.value(&self.graph)].marker
    }

    fn marker(&self, pointer: usize) -> &Option<Type> {
        &self.variables[pointer].marker
    }

    fn node_marker_mut(&mut self, node: Node<usize, usize>) -> &mut Option<Type> {
        &mut self.variables[*node.value(&self.graph)].marker
    }

    fn marker_mut(&mut self, pointer: usize) -> &mut Option<Type> {
        &mut self.variables[pointer].marker
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

    fn gen_vars(&mut self) -> Vec<Stmt> {
        // gen tokenstream
        let mut stmts = vec![];
        for (i, var) in self.variables.iter().enumerate() {
            let ident = new_ident(i);

            let stmt = match var.ty {
                Ty::TMP => {
                    let ty = var.marker.clone().expect("require type");
                    parse_quote! {
                        let mut #ident = #ty::zero();
                    }
                }
                Ty::GRAD => {
                    let ty = var.marker.clone().expect("require type");
                    parse_quote! {
                        let mut #ident = #ty::zero();
                    }
                }
                Ty::TOP => {
                    let ty = var.marker.clone().expect("require type");
                    parse_quote! {
                        let #ident = #ty::one();
                    }
                }
                Ty::IF | Ty::IFEL | Ty::FOR | Ty::LOOP => {
                    parse_quote! {
                        let #ident;
                    }
                }
            };
            stmts.push(stmt);
        }

        stmts
    }

    fn gen_return(&self) -> TokenStream {
        let arg: Vec<_> = self.grads.iter().map(|&x| new_ident(x)).collect();
        quote! {
            (#(#arg),*)
        }
    }

    fn gen_backward(&mut self) -> Vec<Stmt> {
        // mark roots
        for i in self.graph.roots() {
            let &index = Node::new(i).value(&self.graph);
            self.variables[index].ty = Ty::TOP;
        }

        // clean up
        for node in self.graph.node_iter() {
            if self.variables[*node.value(&self.graph)].ty != Ty::TOP {
                self.variables[*node.value(&self.graph)].marker = None;
            }
        }

        let mut stmts = vec![];
        for node in self.graph.topological_iter() {
            let node_ident = node.ident(&self.graph);
            let node_type = self.variables[*node.value(&self.graph)]
                .marker
                .clone()
                .expect("need type");

            for edge in node.children(&self.graph) {
                let edge_ident = edge.ident(&self.graph);
                let edge_type = self.variables[*edge.value(&self.graph)]
                    .marker
                    .clone()
                    .expect("need type");
                let to = edge.to(&self.graph);
                let to_ident = to.ident(&self.graph);
                let to_value = &mut self.variables[*to.value(&self.graph)].marker;

                if to_value == &None {
                    *to_value = Some(parse_quote! {<#node_type as Mul<#edge_type>>::Output});
                }

                let stmt = parse_quote! {
                    #to_ident += #node_ident.clone() * #edge_ident;
                };
                stmts.push(stmt);
            }
        }
        stmts
    }

    fn gen_block(&mut self, i: Block) -> Block {
        // gen fn
        let mut block = self.fold_block(i).stmts;

        // gen return
        let return_expr = match block.last() {
            Some(Stmt::Expr(..)) => {
                let grads = self.gen_return();
                let expr = block
                    .last_mut()
                    .expect("cannot compile without return value");
                *expr = parse_quote! {
                    let mady_return = #expr;
                };
                quote! {
                    (mady_return, #grads)
                }
            }
            Some(Stmt::Semi(Expr::Return(..), ..)) => {
                let expr = block.pop().expect("cannot compile without return value");
                quote! {
                    #expr
                }
            }
            _ => panic!("cannot compile without return value"),
        };

        let backward = self.gen_backward();
        let vars = self.gen_vars();

        parse_quote! {
            {
                #(#vars)*
                #(#block)*
                #(#backward)*
                #return_expr
            }
        }
    }
}

impl InnerParser {
    fn parse_expr(&mut self, e: Expr) -> Result<(Node<usize, usize>, Expr), Expr> {
        match e {
            // a + b
            //
            // `a + b` is Expr::Binary
            Expr::Binary(v) => {
                let (ops_fn, ops_trait) = grad_ops(v.op).map_err(|_| Expr::Binary(v.clone()))?;

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
                ops.link(&mut self.graph, edge_left, &left);
                ops.link(&mut self.graph, edge_right, &right);

                let left_type = self.variables[*left.value(&self.graph)]
                    .marker
                    .clone()
                    .expect("need type");
                let right_type = self.variables[*right.value(&self.graph)]
                    .marker
                    .clone()
                    .expect("need type");

                let ty = quote! {
                    <#left_type as #ops_trait<#right_type>>
                };

                self.variables[*ops.value(&self.graph)].marker = Some(parse_quote! {#ty::O0});
                self.variables[edge_left].marker = Some(parse_quote! {#ty::G0});
                self.variables[edge_right].marker = Some(parse_quote! {#ty::G1});

                let edge_left_ident = new_ident(edge_left);
                let edge_right_ident = new_ident(edge_right);
                let ts = parse_quote! {
                    {
                        let (mady_tmp0, (mady_tmp1, mady_tmp2)) = #left_expr.#ops_fn(#right_expr);
                        #edge_left_ident = mady_tmp1;
                        #edge_right_ident = mady_tmp2;
                        mady_tmp0
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
                let (left, left_expr) = self
                    .parse_expr(*v.left.clone())
                    .map_err(|_| Expr::Assign(v.clone()))?;
                let (right, right_expr) = self
                    .parse_expr(*v.right.clone())
                    .map_err(|_| Expr::Assign(v.clone()))?;
                let edge = self.new_tmp();
                let edge_ident = new_ident(edge);

                left.link(&mut self.graph, edge, &right);

                let right_type = self.node_marker(right).clone().expect("need type");

                *self.node_marker_mut(left) = Some(right_type.clone());
                *self.marker_mut(edge) = Some(parse_quote! {<#right_type as One>::O0});

                let right_expr = parse_quote! {
                    {
                        #edge_ident = #right_type::one();
                        #right_expr
                    }
                };

                let assign = ExprAssign {
                    left: Box::new(left_expr),
                    right: Box::new(right_expr),
                    ..v
                };

                Ok((left, Expr::Assign(assign)))
            }

            Expr::Paren(v) => {
                let (inner, inner_expr) = self
                    .parse_expr(*v.expr.clone())
                    .map_err(|_| Expr::Paren(v.clone()))?;
                let paren = ExprParen {
                    expr: Box::new(inner_expr),
                    ..v
                };

                Ok((inner, Expr::Paren(paren)))
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
            // ex: let a:usize = 5;
            Stmt::Local(v) => {
                let (left, left_pat) = self
                    .parse_pat(v.pat.clone())
                    .map_err(|_| Stmt::Local(v.clone()))?;
                if let Some((eq, expr)) = v.init.clone() {
                    let (right, right_expr) =
                        self.parse_expr(*expr).map_err(|_| Stmt::Local(v.clone()))?;
                    let edge = self.new_tmp();
                    let edge_ident = new_ident(edge);

                    left.link(&mut self.graph, edge, &right);

                    let right_type = self.node_marker(right).clone().expect("need type");

                    *self.node_marker_mut(left) = Some(right_type.clone());
                    *self.marker_mut(edge) = Some(parse_quote! {<#right_type as One>::O0});

                    let right_expr = parse_quote! {
                        {
                            #edge_ident = #right_type::one();
                            #right_expr
                        }
                    };
                    let local = Local {
                        init: Some((eq, Box::new(right_expr))),
                        pat: left_pat,
                        ..v
                    };

                    Ok((left, Stmt::Local(local)))
                } else {
                    Ok((left, Stmt::Local(Local { pat: left_pat, ..v })))
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

impl Fold for InnerParser {
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
        let grads = self.gen_return();
        let expr: Expr = match i.expr {
            Some(e) => {
                let tmp = fold_expr(self, *e);
                parse_quote! {
                    (#tmp, #grads)
                }
            }
            None => panic!("cannot compile without return value"),
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

// fn name,trait name
fn grad_ops(op: BinOp) -> Result<(Ident, Ident), ()> {
    match op {
        BinOp::Add(_) => Ok(grad_fn("add")),
        BinOp::Sub(_) => Ok(grad_fn("sub")),
        BinOp::Mul(_) => Ok(grad_fn("mul")),
        BinOp::Div(_) => Ok(grad_fn("div")),
        _ => Err(()),
    }
}

// fn name,trait name
fn grad_fn<T>(name: T) -> (Ident, Ident)
where
    T: Into<String>,
{
    let name = name.into();
    let traitname: String = name
        .split('_')
        .map(|x| x[..1].to_ascii_uppercase() + &x[1..])
        .collect();

    let gradfn = format!("grad_{}", name);
    let gradtrait = format!("Grad{}", traitname);
    (
        Ident::new(gradfn.as_str(), Span::call_site()),
        Ident::new(gradtrait.as_str(), Span::call_site()),
    )
}

impl Fold for Parser {
    fn fold_fn_arg(&mut self, i: FnArg) -> FnArg {
        if let FnArg::Typed(PatType { pat, ty, .. }) = i.clone() {
            if let Pat::Ident(PatIdent { ident, .. }) = *pat {
                self.grads.push((ident, *ty))
            }
        }
        i
    }

    fn fold_return_type(&mut self, i: ReturnType) -> ReturnType {
        match i {
            ReturnType::Default => panic!("cannot compile without return value"),
            ReturnType::Type(arr, ty) => {
                let ty = *ty;
                match ty {
                    Type::Path(_) => self.return_ty.push(ty.clone()),
                    Type::Array(_)
                    | Type::Ptr(_)
                    | Type::Reference(_)
                    | Type::Slice(_)
                    | Type::Tuple(_) => unimplemented!("not support destructure"),
                    _ => panic!("unsupport type"),
                }

                let arg: Vec<_> = self.grads.iter().map(|x| x.1.clone()).collect();
                let ty = parse_quote! {
                    (#ty, (#(#arg),*))
                };
                ReturnType::Type(arr, Box::new(ty))
            }
        }
    }

    fn fold_signature(&mut self, i: syn::Signature) -> syn::Signature {
        let ident = Ident::new(
            format!("{}{}", "grad_", i.ident).as_str(),
            Span::call_site(),
        );
        let sig = Signature { ident, ..i };
        fold_signature(self, sig)
    }
}

impl Parser {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn gen(mut self, i: ItemFn) -> TokenStream {
        let org = i.clone();
        let sig = self.fold_signature(i.sig);
        let block = Box::new(InnerParser::new(self.grads).gen(*i.block));

        let i = ItemFn { sig, block, ..i };
        quote! {
            #org

            #i
        }
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

    use super::*;

    //     #[test]
    //     fn test_expr_binary() {
    //         let ast = parse_quote! {
    //             a + b
    //         };
    //         let res = quote! {
    //             {
    //                 let (mady_tmp0, (mady_tmp1, mady_tmp2)) = a.grad_add(b);
    //                 mady_3 = mady_tmp1;
    //                 mady_4 = mady_tmp2;
    //                 mady_tmp0
    //             }
    //         };
    //         let mut parser = InnerParser::new::<_, Ident>([]);
    //         parser.enter_block();
    //         parser.new_local_node(&"a").unwrap();
    //         parser.new_local_node(&"b").unwrap();
    //         let (_, ast) = parser.parse_expr(ast).unwrap();
    //         let ast = quote! {#ast};
    //         assert_eq!(ast.to_string(), res.to_string());
    //     }

    //     #[test]
    //     fn test_expr_assign() {
    //         let ast = parse_quote! {
    //             {
    //                 c = a - b;
    //             }
    //         };
    //         let res = quote! {
    //             {
    //                 c = {
    //                     mady_6 = c.one();
    //                     {
    //                         let (mady_tmp0, (mady_tmp1, mady_tmp2)) = a.grad_sub(b);
    //                         mady_4 = mady_tmp1;
    //                         mady_5 = mady_tmp2;
    //                         mady_tmp0
    //                     }
    //                 };
    //             }
    //         };
    //         let mut parser = InnerParser::new::<_, Ident>([]);
    //         parser.enter_block();
    //         parser.new_local_node(&"a").unwrap();
    //         parser.new_local_node(&"b").unwrap();
    //         parser.new_local_node(&"c").unwrap();
    //         let ast = parser.fold_expr(ast);
    //         let ast = quote! {#ast};
    //         assert_eq!(ast.to_string(), res.to_string());
    //     }

    //     #[test]
    //     fn test_expr_local_declare() {
    //         let ast = parse_quote! {
    //             {
    //                 let (a, b)=(1, 2);
    //                 let c;
    //                 c = a * b;
    //             }
    //         };
    //         let res = quote! {
    //             {
    //                 let (a, b)=(1, 2);
    //                 let c;
    //                 c = {
    //                     mady_6 = c.one();
    //                     {
    //                         let (mady_tmp0, (mady_tmp1, mady_tmp2)) = a.grad_mul(b);
    //                         mady_4 = mady_tmp1;
    //                         mady_5 = mady_tmp2;
    //                         mady_tmp0
    //                     }
    //                 };
    //             }
    //         };
    //         let mut parser = InnerParser::new::<_, Ident>([]);
    //         let ast = parser.fold_expr(ast);
    //         let ast = quote! {#ast};
    //         assert_eq!(ast.to_string(), res.to_string());
    //     }

    //     #[test]
    //     fn test_expr_local_assign() {
    //         let ast = parse_quote! {
    //             {
    //                 let (a, b)=(1, 2);
    //                 let c = a / b;
    //             }
    //         };
    //         let res = quote! {
    //             {
    //                 let (a, b)=(1, 2);
    //                 let c = {
    //                     mady_6 = c.one();
    //                     {
    //                         let (mady_tmp0, (mady_tmp1, mady_tmp2)) = a.grad_div(b);
    //                         mady_4 = mady_tmp1;
    //                         mady_5 = mady_tmp2;
    //                         mady_tmp0
    //                     }
    //                 };
    //             }
    //         };
    //         let mut parser = InnerParser::new::<_, Ident>([]);
    //         let ast = parser.fold_expr(ast);
    //         let ast = quote! {#ast};
    //         assert_eq!(ast.to_string(), res.to_string());
    //     }

    // #[test]
    // fn test_gen_var() {
    //     let mut parser = InnerParser::new::<_, Ident>([]);
    //     parser.enter_block();
    //     parser.new_tmp();
    //     let grad = parser.new_grad_node("", parse_quote! {usize});
    //     let root = parser.new_tmp_node();
    //     root.link(&mut parser.graph, 0, &grad);

    //     let ast = parser.gen_vars();

    //     let ast = quote! {
    //         #(#ast)*
    //     };

    //     let res = quote! {
    //         let mut mady_0 = Zero::zero();
    //         let mut mady_1 = Zero::zero();
    //         let mady_2: usize = One::one();
    //     };
    //     assert_eq!(ast.to_string(), res.to_string());
    // }

    //     #[test]
    //     fn test_gen_backward() {
    //         let ast = parse_quote! {
    //             {
    //                 let c;
    //                 c = a * b;
    //             }
    //         };
    //         let res = quote! {
    //             mady_3 += mady_2.clone() * mady_6;
    //             mady_0 += mady_3.clone() * mady_4;
    //             mady_1 += mady_3.clone() * mady_5;
    //         };
    //         let mut parser = InnerParser::new::<_, &str>(["a", "b"]);
    //         parser.fold_expr(ast);
    //         let ast = parser.gen_backward();
    //         let ast = quote! {
    //             #(#ast)*
    //         };
    //         assert_eq!(ast.to_string(), res.to_string());
    // }

    #[test]
    fn test_gen_macro() {
        let ast = parse_quote! {
            fn a_plus_b(a: usize, b: usize) -> usize {
                let c = a + b;
                c
            }
        };
        // let res = quote! {
        //     mady_3 += mady_2.clone() * mady_6;
        //     mady_0 += mady_3.clone() * mady_4;
        //     mady_1 += mady_3.clone() * mady_5;
        // };

        let ast = Parser::new().gen(ast);
        // assert_eq!(ast.to_string(), res.to_string());
        dbg!(ast.to_string());
    }
}
