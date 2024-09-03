use std::fmt::Display;

use crate::ast::Lit;
use crate::span::Span;

use super::{Ident, Path};

#[derive(Debug)]
pub enum Expr {
    Bin(BinExp),
    Lit(Lit),
    Ident(Ident),
    FunCall(FnCall),
    Path(Path),
}

#[derive(Debug)]
pub enum Op {
    Add,
    Multiply,
}

#[derive(Debug)]
pub struct BinExp {
    pub lhs: Box<Expr>,
    pub op: Op,
    pub rhs: Box<Expr>,
    pub span: Span,
}

impl BinExp {
    /// Creates a new binary expression. Note that the constructor requires all fields immediately
    /// rather than follwing the builder pattern.
    pub fn new(lhs: Expr, op: Op, rhs: Expr) -> Self {
        BinExp {
            lhs: Box::new(lhs),
            op: Op::Add,
            rhs: Box::new(rhs),
            span: Span::new(),
        }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

#[derive(Debug)]
pub struct Unit {
    span: Span,
}

impl Unit {
    pub fn new() -> Self {
        Unit { span: Span::new() }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

#[derive(Debug)]
pub struct FnCall {
    pub path: Path,
    pub args: Vec<Expr>,
    pub span: Span,
}

impl FnCall {
    pub fn new() -> Self {
        FnCall {
            path: Path::new(),
            args: vec![],
            span: Span::new(),
        }
    }

    pub fn with_path(mut self, path: Path) -> Self {
        self.path = path;
        self
    }

    pub fn with_args(mut self, args: Vec<Expr>) -> Self {
        self.args = args;
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}
