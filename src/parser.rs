use std::collections::VecDeque;

use crate::ast::{Block, Call, File, Ident, Item, ItemFn, ItemMod, Origin, Path};

use crate::error::ParseError;
use crate::span::Span;
use crate::token::Token;

pub struct Parser<'a> {
    input: &'a [Token],
    spans: &'a [Span],
    index: usize,
    starts: Vec<Span>,
}

type ParseResult<T> = Result<T, ParseError>;

impl<'a> Parser<'a> {
    pub fn new(input: &'a [Token], spans: &'a [Span]) -> Self {
        Parser {
            input,
            spans,
            index: 0,
            starts: vec![spans[0].clone()],
        }
    }

    /// Parse an entire file
    pub fn parse_file(&mut self) -> ParseResult<File> {
        // Start a new span
        self.start();

        let mut items: Vec<Item> = Vec::new();

        while (self.current() != &Token::EOF && self.current() != &Token::RBrace) {
            items.push(self.parse_item()?);
        }

        Ok(File::new().with_items(items).with_span(self.span()))
    }

    pub fn parse_item(&mut self) -> ParseResult<Item> {
        let token = self.current();

        if token == &Token::KwFn {
            self.parse_item_fn()
        } else if token == &Token::KwMod {
            self.parse_item_mod()
        } else {
            Err(ParseError::from(format!(
                "Expected 'fn' or 'mod', found {}",
                token
            )))
        }
    }

    pub fn parse_item_fn(&mut self) -> ParseResult<Item> {
        // Start a new span
        self.start();

        // Consume the `fn` token
        self.expect(Token::KwFn)?;

        // Read the identifier
        let ident = self.parse_ident()?;

        // Read the parameters
        self.expect(Token::LParen)?;
        // TODO: Parse the parameter list
        self.expect(Token::RParen)?;

        // Read the return type
        self.expect(Token::RArrow)?;
        let ret_origin = self.parse_origin()?;

        // Read the brackets and function body
        self.expect(Token::LBrace)?;
        let body = self.parse_block()?;
        self.expect(Token::RBrace)?;

        Ok(Item::ItemFn(
            ItemFn::new()
                .with_ident(ident)
                .with_body(body)
                .with_ret_origin(ret_origin)
                .with_span(self.span()),
        ))
    }

    pub fn parse_origin(&mut self) -> ParseResult<Origin> {
        self.expect(Token::LBrace)?;

        let mut origin = Origin::Universal;
        if let Token::Ident(ident) = self.current() {
            let path = self.parse_path()?;
            origin = Origin::Exact(path);
        } else {
            self.expect(Token::Star)?;
        }

        self.expect(Token::RBrace)?;
        Ok(origin)
    }

    pub fn parse_ident(&mut self) -> ParseResult<Ident> {
        // Start a new span
        self.start();

        match self.current().clone() {
            Token::Ident(ident) => {
                self.advance(1);
                Ok(ident.clone().with_span(self.span()))
            }
            _ => Err(ParseError::new(format!(
                "Expected identifier, found {}",
                self.current()
            ))),
        }
    }

    pub fn parse_block(&mut self) -> ParseResult<Block> {
        // Start a new span
        self.start();

        let mut calls = Vec::new();

        while self.current() != &Token::RBrace {
            calls.push(self.parse_call()?);
            self.expect(Token::Semi)?;
        }

        Ok(Block::new().with_calls(calls).with_span(self.span()))
    }

    pub fn parse_call(&mut self) -> ParseResult<Call> {
        let path = self.parse_path()?;
        self.expect(Token::LParen)?;
        self.expect(Token::RParen)?;
        Ok(Call::new().with_path(path))
    }

    pub fn parse_path(&mut self) -> ParseResult<Path> {
        let mut segments = Vec::new();
        segments.push(self.parse_ident()?);

        while self.current() == &Token::ColonColon {
            self.expect(Token::ColonColon)?;
            segments.push(self.parse_ident()?);
        }

        Ok(Path::new().with_segments(segments))
    }

    pub fn parse_item_mod(&mut self) -> ParseResult<Item> {
        // Start a new span
        self.start();

        // Consume the `fn` token
        self.expect(Token::KwMod)?;

        // Read the identifier
        let ident = self.parse_ident()?;

        // Read the brackets and function body
        self.expect(Token::LBrace)?;
        let file = self.parse_file()?;
        self.expect(Token::RBrace)?;

        Ok(Item::ItemMod(
            ItemMod::new()
                .with_ident(ident)
                .with_file(file)
                .with_span(self.span()),
        ))
    }

    fn start(&mut self) {
        let mut span = Span::new();

        if self.index < self.spans.len() {
            span = self.spans[self.index].clone();
        }

        self.starts.push(span);
    }

    fn finish(&mut self) {
        self.starts.pop();
    }

    fn current(&self) -> &Token {
        if self.index > self.input.len() {
            return &Token::EOF;
        }

        &self.input[self.index]
    }

    fn span(&mut self) -> Span {
        let mut end = self.starts[0].clone();

        if self.index > 0 && self.index <= self.spans.len() {
            end = self.spans[self.index - 1].clone();
        }

        let span = Span::new()
            .from(self.starts.last().unwrap().from.clone())
            .to(end.to.clone());

        self.finish();
        span
    }

    fn _eat(&mut self, kind: Token) -> bool {
        if self.current() == &kind {
            self.advance(1);
            return true;
        }

        false
    }

    fn advance(&mut self, times: usize) {
        self.index += times;
    }

    fn expect(&mut self, kind: Token) -> ParseResult<&Token> {
        // Start a new span
        self.start();

        if self.current().clone() == kind {
            self.advance(1);
            // Return the current token
            self.finish();
            return Ok(self.current());
        }

        let err_span = self.span();
        Err(ParseError::from(format!(
            "Expected '{}' but found '{}' ({})",
            kind,
            self.current(),
            err_span
        )))
    }
}
