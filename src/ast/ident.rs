use crate::span::Span;

#[derive(Clone, PartialEq, Debug)]
pub struct Ident {
    raw: String,
    span: Span
}

impl Ident {
    pub fn new() -> Self {
        Ident { raw: String::new(), span: Span::new() }
    }

    pub fn with_raw(mut self, raw: String) -> Self {
        self.raw = raw;
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }
}
