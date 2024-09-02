use crate::ast::Lit;
use crate::span::Span;

use super::Ident;

#[derive(Debug)]
pub enum Expr {
    Bin(BinExp),
    Lit(Lit),
    Ident(Ident),
    Unit(Unit)
}

#[derive(Debug)]
pub enum Op {
    Plus
}

#[derive(Debug)]
pub struct BinExp {
    lhs: Box<Expr>,
    op: Op,
    rhs: Box<Expr>,
    span: Span
}

impl BinExp {
    /// Creates a new binary expression. Note that the constructor requires all fields immediately
    /// rather than follwing the builder pattern.
    pub fn new(lhs: Expr, op: Op, rhs: Expr) -> Self {
        BinExp {
            lhs: Box::new(lhs),
            op: Op::Plus,
            rhs: Box::new(rhs),
            span: Span::new()
        }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

#[derive(Debug)]
pub struct Unit {
    span: Span
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