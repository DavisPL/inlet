use crate::visit::{visit_item, Visit};

pub struct OriginVisitor {}

impl Visit for OriginVisitor {
    fn visit_item(&mut self, node: &crate::ast::Item) {
        println!("[DEBUG] Visiting item...");
        visit_item(self, node)
    }

    fn visit_item_fn(&mut self, node: &crate::ast::ItemFn) {
        println!("[DEBUG] Visiting function named {:?}", node.ident)
    }
}
