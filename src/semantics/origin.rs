use core::panic;

use crate::{
    ast::{Expr, File, Local, Origin, Path},
    symbol::SymbolTable,
    visit::{self, visit_item_fn, visit_item_mod, visit_local, Visit},
};

use super::{
    error::{SemaError, SemaResult},
    Analysis, FunctionData,
};

pub struct OriginAnalysis<'a> {
    file: &'a File,
    functions: &'a SymbolTable<FunctionData>,
    locals: SymbolTable<LocalData>,
    cur_func: String,
    ret_origin: Origin,
    prefix: String,
    errors: Vec<SemaError>,
}

impl<'a> OriginAnalysis<'a> {
    pub fn new(file: &'a File, functions: &'a SymbolTable<FunctionData>, prefix: String) -> Self {
        OriginAnalysis {
            file,
            functions,
            locals: SymbolTable::new(),
            cur_func: String::new(),
            ret_origin: Origin::Universal,
            prefix: prefix.clone(),
            errors: vec![],
        }
    }
}

impl Analysis for OriginAnalysis<'_> {
    fn analyze(&mut self) -> super::error::SemaResult<()> {
        self.visit_file(self.file);

        if self.errors.len() > 0 {
            Err(self.errors.clone())
        } else {
            Ok(())
        }
    }
}

impl Visit for OriginAnalysis<'_> {
    fn visit_item_mod(&mut self, node: &crate::ast::ItemMod) {
        let _prefix = self.prefix.clone();

        self.prefix += "::";
        self.prefix += node.ident.to_str();

        visit_item_mod(self, node);

        self.prefix = _prefix;
    }

    fn visit_item_fn(&mut self, node: &crate::ast::ItemFn) {
        for param in &node.params {
            self.locals.insert(
                param.ident.to_string(),
                LocalData {
                    origin: param.origin.clone(),
                },
            )
        }

        self.cur_func = node.ident.to_string();
        self.ret_origin = node.ret_origin.clone();
        visit_item_fn(self, node);

        self.ret_origin = Origin::Universal;
        self.locals.clear();
    }

    fn visit_local(&mut self, node: &Local) {
        let origin = ExprVisitor::visit(&self.prefix, self.functions, &self.locals, &node.expr);

        match origin {
            Ok(origin) => {
                self.locals
                    .insert(node.ident.to_string(), LocalData { origin });
            }

            Err(errs) => self.errors.extend(errs),
        }
    }

    fn visit_return(&mut self, node: &crate::ast::Return) {
        let ret_origin = ExprVisitor::visit(&self.prefix, self.functions, &self.locals, &node.expr);

        match ret_origin {
            Ok(ret_origin) => {
                if !ret_origin.satisfies(&self.ret_origin) {
                    let message = format!("Function '{}' should return a value with origin '{}', but a value with origin '{}' is returned instead", self.cur_func, self.ret_origin, ret_origin);
                    self.errors.push(
                        SemaError::new()
                            .with_message(message)
                            .with_span(node.span.clone()),
                    )
                }
            }

            Err(errs) => self.errors.extend(errs),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LocalData {
    origin: Origin,
}

pub struct ExprVisitor<'a> {
    prefix: &'a str,
    functions: &'a SymbolTable<FunctionData>,
    locals: &'a SymbolTable<LocalData>,
    errors: Vec<SemaError>,
    origin: Origin,
}

impl<'a> ExprVisitor<'a> {
    pub fn visit(
        prefix: &'a str,
        functions: &'a SymbolTable<FunctionData>,
        locals: &'a SymbolTable<LocalData>,
        expr: &Expr,
    ) -> SemaResult<Origin> {
        let mut visitor = ExprVisitor {
            prefix,
            functions,
            locals,
            errors: vec![],
            origin: Origin::Universal,
        };

        visitor.visit_expr(expr);

        if visitor.errors.len() > 0 {
            Err(visitor.errors)
        } else {
            Ok(visitor.origin)
        }
    }
}

impl<'a> Visit for ExprVisitor<'a> {
    fn visit_bin_expr(&mut self, _: &crate::ast::BinExp) {
        // Any binary expression that occurs in origin `o` will have that origin
        self.origin = Origin::Exact(Path::from(self.prefix.to_owned()))
    }

    fn visit_path(&mut self, node: &Path) {
        let name = node.to_string();

        if let Some(data) = self.locals.find(&name) {
            self.origin = data.origin;
        } else {
            self.errors.push(
                SemaError::new()
                    .with_message(format!(
                        "Could not find definition of identifier '{}'",
                        name
                    ))
                    .with_span(node.span.clone()),
            )
        }
    }

    fn visit_ident(&mut self, _node: &crate::ast::Ident) {
        panic!("This should be unreachable. If you see this, there is a bug. At some point, idents stopped being expressions and were replaced by paths.");
    }

    fn visit_num_lit(&mut self, _node: &crate::ast::NumLit) {
        // Any literal that occurs in origin `o` will have that origin
        self.origin = Origin::Exact(Path::from(self.prefix.to_owned()))
    }

    fn visit_fn_call(&mut self, node: &crate::ast::FnCall) {
        let name = node.path.to_string();

        if let Some(data) = self.functions.find(&name) {
            self.origin = data.ret_origin;

            for (param, arg) in data.params.iter().zip(node.args.iter()) {
                let actual = ExprVisitor::visit(&self.prefix, self.functions, &self.locals, arg);

                match actual {
                    Ok(origin) => {
                        if !origin.satisfies(&param.origin) {
                            self.errors.push(SemaError::new().with_message(format!("Parameter '{}' of function '{}' must have an origin of '{}', but a value with origin '{}' was provided", param.name, node.path.to_string(), param.origin, origin)).with_span(arg.span()))
                        }
                    }

                    Err(errs) => self.errors.extend(errs),
                }
            }
        } else {
            self.errors.push(
                SemaError::new()
                    .with_message(format!(
                        "Could not find definition of identifier '{}'",
                        name
                    ))
                    .with_span(node.span.clone()),
            )
        }
    }
}
