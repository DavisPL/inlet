use crate::ast::stmt::Stmt;
use crate::span::Span;

#[derive(Debug)]
pub struct Block {
    stmts: Vec<Stmt>,
    span: Span,
}

impl Block {
    pub fn new() -> Self {
        Block {
            stmts: vec![],
            span: Span::new(),
        }
    }

    pub fn with_calls(mut self, calls: Vec<Stmt>) -> Self {
        self.stmts = calls;
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}
