use crate::ast::path::Path;

#[derive(Debug)]
pub enum Origin {
    Universal,
    Exact(Path),
}
