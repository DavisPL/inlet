use crate::ast::call::Call;
use crate::span::Span;

#[derive(Debug)]
pub struct Block {
    calls: Vec<Call>,
    span: Span,
}

impl Block {
    pub fn new() -> Self {
        Block {
            calls: vec![],
            span: Span::new(),
        }
    }

    pub fn with_calls(mut self, calls: Vec<Call>) -> Self {
        self.calls = calls;
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}
