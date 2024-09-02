use crate::ast::item::Item;
use crate::span::Span;

#[derive(Debug)]
pub struct File {
    pub items: Vec<Item>,
    pub span: Span,
}

impl File {
    pub fn new() -> Self {
        File {
            items: vec![],
            span: Span::new(),
        }
    }

    pub fn with_items(mut self, items: Vec<Item>) -> Self {
        self.items = items;
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}
