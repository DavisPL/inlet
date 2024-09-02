#[derive(Debug)]
pub struct LexError {
    pub reason: String,
}

impl LexError {
    pub fn new(reason: String) -> Self {
        LexError { reason }
    }
}

impl From<String> for LexError {
    fn from(value: String) -> Self {
        LexError::new(value)
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub reason: String,
}

impl ParseError {
    pub fn new(reason: String) -> Self {
        ParseError { reason }
    }
}

impl From<String> for ParseError {
    fn from(value: String) -> Self {
        ParseError::new(value)
    }
}
