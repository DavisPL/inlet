use crate::{
    ast::{block::Block, file::File, ident::Ident},
    span::Span,
};

use super::{origin, Origin};

#[derive(Debug)]
pub enum Item {
    ItemFn(ItemFn),
    ItemMod(ItemMod),
}

#[derive(Debug)]
pub struct ItemFn {
    pub ident: Ident,
    pub body: Block,
    pub ret_origin: Origin,
    pub span: Span,
}

impl ItemFn {
    pub fn new() -> Self {
        ItemFn {
            ident: Ident::new(),
            body: Block::new(),
            ret_origin: Origin::Universal,
            span: Span::new(),
        }
    }

    pub fn with_ident(mut self, ident: Ident) -> Self {
        self.ident = ident;
        self
    }

    pub fn with_body(mut self, body: Block) -> Self {
        self.body = body;
        self
    }

    pub fn with_ret_origin(mut self, ret_origin: Origin) -> Self {
        self.ret_origin = ret_origin;
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

#[derive(Debug)]
pub struct ItemMod {
    pub ident: Ident,
    pub file: File,
    pub span: Span,
}

impl ItemMod {
    pub fn new() -> Self {
        ItemMod {
            ident: Ident::new(),
            file: File::new(),
            span: Span::new(),
        }
    }

    pub fn with_ident(mut self, ident: Ident) -> Self {
        self.ident = ident;
        self
    }

    pub fn with_file(mut self, body: File) -> Self {
        self.file = body;
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}
