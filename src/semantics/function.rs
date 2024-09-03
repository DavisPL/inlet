use crate::{
    ast::{Expr, File, Origin},
    symbol::SymbolTable,
    visit::{visit_expr, visit_item_fn, visit_item_mod, visit_local, Visit},
};

/// This analysis finds every function declaration and stores its canonical path.
pub struct FunctionAnalysis<'a> {
    file: &'a File,
    prefix: String,
    table: SymbolTable<FunctionData>,
}

#[derive(Clone, Debug)]
pub struct FunctionData {
    pub params: Vec<ParamData>,
    pub ret_origin: Origin,
}

#[derive(Clone, Debug)]
pub struct ParamData {
    pub name: String,
    pub origin: Origin,
}

impl<'a> FunctionAnalysis<'a> {
    pub fn new(
        file: &'a File,
        prefix: String,
        table: &'a SymbolTable<FunctionData>,
    ) -> FunctionAnalysis<'a> {
        FunctionAnalysis {
            file,
            prefix,
            table: table.clone(), // This is probably way too expensive
        }
    }

    pub fn analyze(&mut self) -> super::error::SemaResult<SymbolTable<FunctionData>> {
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
        self.table.insert(
            path,
            FunctionData {
                params: node
                    .params
                    .iter()
                    .map(|param| ParamData {
                        name: param.ident.to_string(),
                        origin: param.origin.clone(),
                    })
                    .collect(),
                ret_origin: node.ret_origin.clone(),
            },
        );
    }

    fn visit_item_mod(&mut self, node: &crate::ast::ItemMod) {
        let _prefix = self.prefix.clone();

        self.prefix += "::";
        self.prefix += node.ident.to_str();

        visit_item_mod(self, node);

        self.prefix = _prefix;
    }
}
