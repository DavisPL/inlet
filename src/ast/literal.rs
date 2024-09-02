use crate::span::Span;

use super::Unit;

#[derive(Debug)]
pub enum Lit {
    NumLit(NumLit),
    UnitLit(Unit),
}

#[derive(Clone, PartialEq, Debug)]
pub struct NumLit {
    value: i32,
    span: Span,
}

impl NumLit {
    pub fn new() -> Self {
        NumLit {
            value: 0,
            span: Span::new(),
        }
    }

    pub fn with_value(mut self, value: i32) -> Self {
        self.value = value;
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
