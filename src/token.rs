use std::fmt::write;

use crate::ast::{Ident, NumLit};

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    // Keywords
    KwFn,
    KwMod,
    KwLet,
    KwReturn,

    // Constructs
    Ident(Ident),
    NumLit(NumLit),

    // Delimiters
    Comma,
    Colon,
    ColonColon,
    Semi,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,

    // Symbols
    Equal,
    Star,
    Plus,
    RArrow,

    // Other
    EOF,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KwFn => write!(f, "fn"),
            Self::KwMod => write!(f, "mod"),
            Self::KwLet => write!(f, "let"),
            Self::KwReturn => write!(f, "return"),

            Self::Ident(ident) => write!(f, "{}", ident.to_str()),
            Self::NumLit(lit) => write!(f, "{:?}", lit.value()),

            Self::Colon => write!(f, ":"),
            Self::ColonColon => write!(f, "::"),
            Self::Semi => write!(f, ";"),
            Self::LParen => write!(f, "("),
            Self::RParen => write!(f, ")"),
            Self::LBracket => write!(f, "["),
            Self::RBracket => write!(f, "]"),
            Self::LBrace => write!(f, "{{"),
            Self::RBrace => write!(f, "}}"),

            Self::Equal => write!(f, "="),
            Self::Star => write!(f, "*"),
            Self::Plus => write!(f, "+"),
            Self::RArrow => write!(f, "->"),

            Self::EOF => write!(f, "EOF"),

            _ => write!(f, "<UNKNOWN>"),
        }
    }
}
