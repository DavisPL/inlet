#[derive(Clone, PartialEq, Debug)]
pub struct NumLit {
    value: i32,
}

impl NumLit {
    pub fn new() -> Self {
        NumLit { value: 0 }
    }

    pub fn with_value(mut self, value: i32) -> Self {
        self.value = value;
        self
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
