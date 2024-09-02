use crate::span::Span;

pub type SemaResult<T> = Result<T, Vec<SemaError>>;

#[derive(Clone)]
pub struct SemaError {
    pub message: String,
    pub span: Span,
}

impl SemaError {
    pub fn new() -> Self {
        SemaError {
            message: String::new(),
            span: Span::new(),
        }
    }

    pub fn with_message(mut self, message: String) -> Self {
        self.message = message;
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}
