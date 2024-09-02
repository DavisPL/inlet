use crate::ast::{block::Block, file::File, ident::Ident};

use super::{origin, Origin};

#[derive(Debug)]
pub enum Item {
    ItemFn(ItemFn),
    ItemMod(ItemMod),
}

#[derive(Debug)]
pub struct ItemFn {
    ident: Ident,
    body: Block,
    ret_origin: Origin,
}

impl ItemFn {
    pub fn new() -> Self {
        ItemFn {
            ident: Ident::new(),
            body: Block::new(),
            ret_origin: Origin::Universal,
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
}

#[derive(Debug)]
pub struct ItemMod {
    ident: Ident,
    file: File,
}

impl ItemMod {
    pub fn new() -> Self {
        ItemMod {
            ident: Ident::new(),
            file: File::new(),
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
}
