pub struct Diagnostic {
    message: String,
}

impl Diagnostic {
    pub fn new(message: String) -> Self {
        Diagnostic { message }
    }
}

impl From<&str> for Diagnostic {
    fn from(value: &str) -> Self {
        Diagnostic::new(value.to_owned())
    }
}

impl From<String> for Diagnostic {
    fn from(value: String) -> Self {
        Diagnostic::new(value)
    }
}
