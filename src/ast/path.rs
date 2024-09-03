use std::fmt::Display;

use crate::{ast::ident::Ident, span::Span};

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub segments: Vec<Ident>,
    pub span: Span,
}

impl Path {
    pub fn new() -> Self {
        Path {
            segments: vec![],
            span: Span::new(),
        }
    }

    pub fn with_segments(mut self, segments: Vec<Ident>) -> Self {
        self.segments = segments;
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

impl From<String> for Path {
    fn from(value: String) -> Self {
        let segments = value
            .split("::")
            .map(|raw| Ident::new().with_raw(raw.to_owned()))
            .collect();
        Path {
            segments,
            span: Span::new(),
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rep = self
            .segments
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("::");

        f.write_fmt(format_args!("{}", rep))
    }
}
