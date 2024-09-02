use crate::span::Span;

#[derive(Clone, PartialEq, Debug)]
pub struct Ident {
    raw: String,
}

impl Ident {
    pub fn new() -> Self {
        Ident { raw: String::new() }
    }

    pub fn with_raw(mut self, raw: String) -> Self {
        self.raw = raw;
        self
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }
}
