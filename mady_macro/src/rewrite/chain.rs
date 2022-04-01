use std::ops::{Deref, DerefMut};

use syn::{Expr, ExprBlock};

use super::graph::Graph;

type FoldChain<T> = Chain<T, Var, Var>;

struct Chain<T, N, E> {
    ast: T,
    graph: Graph<N, E>,
}

struct Var {
    ty: Type,
}

enum Type {
    /// not init
    TMP,
    /// init to Zero::zero
    Grad,
    /// not init, `if ...{...}`
    IF,
    /// not init, `if...{...}else{...}`
    IFEL,
}

trait Fold {
    fn expr(&mut self, c: &mut FoldChain<Expr>) -> FoldChain<Expr>;
    fn expr_block(&mut self, c: FoldChain<ExprBlock>) -> FoldChain<Expr>;
}

trait OnFold {}

impl<T, N, E> DerefMut for Chain<T, N, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ast
    }
}

impl<T, N, E> Deref for Chain<T, N, E> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.ast
    }
}
