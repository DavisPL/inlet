use crate::ast::Ident;
use crate::span::Span;
use crate::ast::Expr;

use super::Unit;

#[derive(Debug)]
pub enum Stmt {
    Local(Local)
}

#[derive(Debug)]
pub struct Local {
    ident: Ident,
    expr: Expr,
    span: Span
}

impl Local {
    pub fn new() -> Self {
        Local { ident: Ident::new(), span: Span::new(), expr: Expr::Unit(Unit::new()) }
    }

    pub fn with_ident(mut self, ident: Ident) -> Self {
        self.ident = ident;
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    pub fn with_expr(mut self, expr: Expr) -> Self {
        self.expr = expr;
        self
    }
}