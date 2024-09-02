use crate::ast::path::Path;

#[derive(Debug, Clone)]
pub enum Origin {
    Universal,
    Exact(Path),
}
