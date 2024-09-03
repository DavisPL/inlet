use crate::ast::{
    BinExp, Block, Expr, File, FnCall, Ident, Item, ItemFn, ItemMod, Lit, Local, NumLit, Origin,
    Path, Stmt, Unit,
};

pub trait Visit: Sized {
    fn visit_file(&mut self, node: &File) {
        visit_file(self, node)
    }

    fn visit_item(&mut self, node: &Item) {
        visit_item(self, node)
    }

    fn visit_item_fn(&mut self, node: &ItemFn) {
        visit_item_fn(self, node)
    }

    fn visit_item_mod(&mut self, node: &ItemMod) {
        visit_item_mod(self, node)
    }

    fn visit_block(&mut self, node: &Block) {
        visit_block(self, node)
    }

    fn visit_stmt(&mut self, node: &Stmt) {
        visit_stmt(self, node)
    }

    fn visit_local(&mut self, node: &Local) {
        visit_local(self, node)
    }

    fn visit_expr(&mut self, node: &Expr) {
        visit_expr(self, node)
    }

    fn visit_bin_expr(&mut self, node: &BinExp) {
        visit_bin_expr(self, node)
    }

    fn visit_fn_call(&mut self, node: &FnCall) {
        visit_fn_call(self, node)
    }

    fn visit_lit(&mut self, node: &Lit) {
        visit_lit(self, node)
    }

    fn visit_ident(&mut self, _node: &Ident) {
        // Nothing to do here...
    }

    fn visit_origin(&mut self, _node: &Origin) {
        // Nothing to do here...
    }

    fn visit_num_lit(&mut self, _node: &NumLit) {
        // Nothing to do here...
    }

    fn visit_unit_lit(&mut self, _node: &Unit) {
        // Nothing to do here...
    }

    fn visit_path(&mut self, _node: &Path) {
        // Nothing to do here...
    }
}

pub fn visit_file(visitor: &mut impl Visit, node: &File) {
    for item in &node.items {
        visitor.visit_item(item)
    }
}

pub fn visit_item(visitor: &mut impl Visit, node: &Item) {
    match node {
        Item::ItemFn(item) => visitor.visit_item_fn(item),
        Item::ItemMod(item) => visitor.visit_item_mod(item),
    }
}

pub fn visit_item_fn(visitor: &mut impl Visit, node: &ItemFn) {
    visitor.visit_ident(&node.ident);
    visitor.visit_origin(&node.ret_origin);
    visitor.visit_block(&node.body);
}

pub fn visit_item_mod(visitor: &mut impl Visit, node: &ItemMod) {
    visitor.visit_ident(&node.ident);
    visitor.visit_file(&node.file);
}

pub fn visit_block(visitor: &mut impl Visit, node: &Block) {
    for stmt in &node.stmts {
        visitor.visit_stmt(stmt);
    }
}

pub fn visit_stmt(visitor: &mut impl Visit, node: &Stmt) {
    match node {
        Stmt::Local(local) => visitor.visit_local(local),
    }
}

pub fn visit_local(visitor: &mut impl Visit, node: &Local) {
    visitor.visit_ident(&node.ident);
    visitor.visit_expr(&node.expr);
}

pub fn visit_expr(visitor: &mut impl Visit, node: &Expr) {
    match node {
        Expr::Bin(bin_expr) => visitor.visit_bin_expr(bin_expr),
        Expr::Ident(ident) => visitor.visit_ident(ident),
        Expr::Lit(lit) => visitor.visit_lit(lit),
        Expr::FunCall(fn_call) => visitor.visit_fn_call(fn_call),
        Expr::Path(path) => visitor.visit_path(path),
    }
}

pub fn visit_bin_expr(visitor: &mut impl Visit, node: &BinExp) {
    visitor.visit_expr(node.lhs.as_ref());
    visitor.visit_expr(node.rhs.as_ref());
}

pub fn visit_lit(visitor: &mut impl Visit, node: &Lit) {
    match node {
        Lit::NumLit(num_lit) => visitor.visit_num_lit(num_lit),
        Lit::UnitLit(unit_lit) => visitor.visit_unit_lit(unit_lit),
    }
}

pub fn visit_fn_call(visitor: &mut impl Visit, node: &FnCall) {
    visitor.visit_path(&node.path);

    for arg in &node.args {
        visitor.visit_expr(arg);
    }
}
