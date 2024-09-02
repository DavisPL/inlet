use crate::ast::call::Call;

#[derive(Debug)]
pub struct Block {
    calls: Vec<Call>,
}

impl Block {
    pub fn new() -> Self {
        Block { calls: vec![] }
    }

    pub fn with_calls(mut self, calls: Vec<Call>) -> Self {
        self.calls = calls;
        self
    }
}
