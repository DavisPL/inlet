use std::fmt::Display;

#[derive(Clone, PartialEq, Debug)]
pub struct Span {
    pub from: Location,
    pub to: Location,
}

impl Span {
    pub fn new() -> Self {
        Span {
            from: Location::new(),
            to: Location::new(),
        }
    }

    pub fn from(mut self, from: Location) -> Self {
        self.from = from;
        self
    }

    pub fn to(mut self, to: Location) -> Self {
        self.to = to;
        self
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}:{} .. {}:{}",
            self.from.line, self.from.column, self.to.line, self.to.column
        ))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new() -> Self {
        Location { line: 0, column: 0 }
    }

    pub fn set_line(&mut self, line: usize) {
        self.line = line;
    }

    pub fn set_column(&mut self, column: usize) {
        self.column = column;
    }

    pub fn with_line(mut self, line: usize) -> Self {
        self.line = line;
        self
    }

    pub fn with_column(mut self, column: usize) -> Self {
        self.column = column;
        self
    }
}
