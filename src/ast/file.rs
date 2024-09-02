use crate::ast::item::Item;

#[derive(Debug)]
pub struct File {
    items: Vec<Item>,
}

impl File {
    pub fn new() -> Self {
        File { items: vec![] }
    }

    pub fn with_items(mut self, items: Vec<Item>) -> Self {
        self.items = items;
        self
    }
}
