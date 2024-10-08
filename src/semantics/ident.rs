use crate::{
    ast::{Expr, File},
    symbol::SymbolTable,
    visit::{visit_expr, visit_item_fn, visit_local, Visit},
};

use super::{error::SemaError, Analysis, FunctionData};

/// This analysis ensures that every identifier is defined before usage.
pub struct IdentAnalysis<'a> {
    file: &'a File,
    functions: &'a SymbolTable<FunctionData>,
    table: SymbolTable<()>,
    errors: Vec<SemaError>,
}

impl Analysis for IdentAnalysis<'_> {
    fn analyze(&mut self) -> super::error::SemaResult<()> {
        self.visit_file(self.file);

        if self.errors.len() > 0 {
            return Err(self.errors.clone());
        }

        Ok(())
    }
}

impl<'a> IdentAnalysis<'a> {
    pub fn new(file: &'a File, functions: &'a SymbolTable<FunctionData>) -> IdentAnalysis<'a> {
        IdentAnalysis {
            file,
            functions,
            table: SymbolTable::new(),
            errors: vec![],
        }
    }
}

impl Visit for IdentAnalysis<'_> {
    fn visit_item_fn(&mut self, node: &crate::ast::ItemFn) {
        // Add function parameters to the symbol table
        for param in &node.params {
            self.table.insert(param.ident.to_string(), ());
        }

        visit_item_fn(self, node);

        // Clear the table for other functions
        self.table.clear();
    }

    fn visit_local(&mut self, node: &crate::ast::Local) {
        self.table.insert(node.ident.to_string(), ());
        visit_local(self, node);
    }

    fn visit_expr(&mut self, node: &crate::ast::Expr) {
        match node {
            Expr::Path(path) => {
                let rep: String = path.to_string();

                // Make sure this identifier has been defined
                if self.table.find(&rep).is_none() {
                    self.errors.push(
                        SemaError::new()
                            .with_message(format!("Couldn't find a definition for '{}'", rep))
                            .with_span(path.span.clone()),
                    );
                }
            }

            Expr::FunCall(fun_call) => {
                let fun_name = &fun_call.path.to_string();

                match self.functions.find(fun_name) {
                    Some(data) => {
                        // Check that the origin of every passed argument matches
                        if data.params.len() != fun_call.args.len() {
                            self.errors.push(
                                SemaError::new()
                                    .with_message(format!(
                                        "Function '{}' expects {} arguments, but {} were provided",
                                        fun_name,
                                        data.params.len(),
                                        fun_call.args.len()
                                    ))
                                    .with_span(fun_call.path.span.clone()),
                            );
                        }
                    }

                    None => {
                        self.errors.push(
                            SemaError::new()
                                .with_message(format!(
                                    "Couldn't find a definition for function '{}'",
                                    fun_name
                                ))
                                .with_span(fun_call.path.span.clone()),
                        );
                    }
                }
            }

            _ => { /* No need to handle other expressions */ }
        }

        visit_expr(self, node);
    }
}
