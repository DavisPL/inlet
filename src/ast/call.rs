use crate::ast::path::Path;

#[derive(Debug)]
pub struct Call {
    path: Path,
}

impl Call {
    pub fn new() -> Self {
        Call { path: Path::new() }
    }

    pub fn with_path(mut self, path: Path) -> Self {
        self.path = path;
        self
    }
}
