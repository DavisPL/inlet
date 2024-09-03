use crate::ast::Expr;
use crate::ast::Ident;
use crate::span::Span;

use super::Lit;
use super::Unit;

#[derive(Debug)]
pub enum Stmt {
    Local(Local),
    Return(Return),
    Claim(Claim)
}

#[derive(Debug)]
pub struct Local {
    pub ident: Ident,
    pub expr: Expr,
    pub span: Span,
}

impl Local {
    pub fn new() -> Self {
        Local {
            ident: Ident::new(),
            span: Span::new(),
            expr: Expr::Lit(Lit::UnitLit(Unit::new())),
        }
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

#[derive(Debug)]
pub struct Return {
    pub expr: Expr,
    pub span: Span,
}

impl Return {
    pub fn new() -> Self {
        Return {
            expr: Expr::Lit(Lit::UnitLit(Unit::new())),
            span: Span::new(),
        }
    }

    pub fn with_expr(mut self, expr: Expr) -> Self {
        self.expr = expr;
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

#[derive(Debug)]
pub struct Claim {
    pub ident: Ident,
    pub span: Span
}

impl Claim {
    pub fn new() -> Self {
        Claim {
            ident: Ident::new(),
            span: Span::new()
        }
    }

    pub fn with_ident(mut self, ident: Ident) -> Self {
        self.ident = ident;
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}