use crate::ast::ident::Ident;

#[derive(Debug, Clone)]
pub struct Path {
    segments: Vec<Ident>,
}

impl Path {
    pub fn new() -> Self {
        Path { segments: vec![] }
    }

    pub fn with_segments(mut self, segments: Vec<Ident>) -> Self {
        self.segments = segments;
        self
    }
}
