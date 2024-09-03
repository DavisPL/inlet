use crate::{
    ast::{Expr, File},
    symbol::SymbolTable,
    visit::{visit_expr, visit_item_fn, visit_item_mod, visit_local, Visit},
};

/// This analysis finds every function declaration and stores its canonical path.
pub struct FunctionAnalysis<'a> {
    file: &'a File,
    prefix: String,
    table: SymbolTable<()>,
}

impl<'a> FunctionAnalysis<'a> {
    pub fn new(file: &File, prefix: String) -> FunctionAnalysis {
        FunctionAnalysis {
            file,
            prefix,
            table: SymbolTable::new(),
        }
    }

    pub fn analyze(&mut self) -> super::error::SemaResult<SymbolTable<()>> {
        self.visit_file(self.file);
        Ok(self.table.clone())
    }
}

impl Visit for FunctionAnalysis<'_> {
    fn visit_item_fn(&mut self, node: &crate::ast::ItemFn) {
        // Construct the canonical path for this function
        let path = self.prefix.clone() + "::" + node.ident.to_str();

        // TODO: Account for functions that are declared multiple times
        // TODO: Account for functions declared inside other functions
        self.table.insert(path, ());
    }

    fn visit_item_mod(&mut self, node: &crate::ast::ItemMod) {
        let _prefix = self.prefix.clone();

        self.prefix += "::";
        self.prefix += node.ident.to_str();

        visit_item_mod(self, node);

        self.prefix = _prefix;
    }
}
