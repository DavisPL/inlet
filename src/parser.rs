use std::collections::VecDeque;

use crate::ast::{
    BinExp, Block, Expr, File, FnCall, FnParam, Ident, Item, ItemFn, ItemMod, Lit, Local, Op,
    Origin, Path, Return, Stmt,
};

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
        let params = self.parse_param_list()?;
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
                .with_params(params)
                .with_body(body)
                .with_ret_origin(ret_origin)
                .with_span(self.span()),
        ))
    }

    pub fn parse_param_list(&mut self) -> ParseResult<Vec<FnParam>> {
        let mut args = Vec::new();

        if self.current() == &Token::RParen {
            // There are no parameters
            Ok(args)
        } else {
            args.push(self.parse_param()?);
            while self.current() == &Token::Comma {
                self.advance(1);
                args.push(self.parse_param()?);
            }
            Ok(args)
        }
    }

    pub fn parse_param(&mut self) -> ParseResult<FnParam> {
        self.start();

        let ident = self.parse_ident()?;
        self.expect(Token::Colon)?;
        let origin = self.parse_origin()?;

        Ok(FnParam::new()
            .with_ident(ident)
            .with_origin(origin)
            .with_span(self.span()))
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
            calls.push(self.parse_stmt()?);
            self.expect(Token::Semi)?;
        }

        Ok(Block::new().with_calls(calls).with_span(self.span()))
    }

    pub fn parse_stmt(&mut self) -> ParseResult<Stmt> {
        let current = self.current();

        if current == &Token::KwLet {
            return Ok(Stmt::Local(self.parse_local()?));
        } else if current == &Token::KwReturn {
            return Ok(Stmt::Return(self.parse_return()?));
        }

        Err(ParseError::from(format!(
            "Unknown statement beginning with '{}'",
            current
        )))
    }

    pub fn parse_local(&mut self) -> ParseResult<Local> {
        self.start();
        self.expect(Token::KwLet)?;

        let ident = self.parse_ident()?;

        self.expect(Token::Equal)?;

        let expr = self.parse_expr()?;

        Ok(Local::new()
            .with_ident(ident)
            .with_expr(expr)
            .with_span(self.span()))
    }

    pub fn parse_return(&mut self) -> ParseResult<Return> {
        self.start();
        self.expect(Token::KwReturn)?;

        let expr = self.parse_expr()?;

        Ok(Return::new().with_expr(expr).with_span(self.span()))
    }

    pub fn parse_expr(&mut self) -> ParseResult<Expr> {
        // Custom handling for span creation
        let mut expr = self.parse_term()?;
        let start = expr.span().clone();

        while self.current() == &Token::Plus {
            self.expect(Token::Plus)?;
            let rhs = self.parse_term()?;
            expr = Expr::Bin(
                BinExp::new(expr, Op::Add, rhs).with_span(
                    Span::new()
                        .from(start.from.clone())
                        .to(self.previous_span().to),
                ),
            );
        }

        Ok(expr)
    }

    pub fn parse_term(&mut self) -> ParseResult<Expr> {
        // Custom handling for span creation
        let mut expr = self.parse_factor()?;
        let start = expr.span().clone();

        while self.current() == &Token::Star {
            self.expect(Token::Star)?;
            let rhs = self.parse_factor()?;
            expr = Expr::Bin(
                BinExp::new(expr, Op::Multiply, rhs).with_span(
                    Span::new()
                        .from(start.from.clone())
                        .to(self.previous_span().to),
                ),
            );
        }

        Ok(expr)
    }

    pub fn parse_factor(&mut self) -> ParseResult<Expr> {
        let current = self.current().clone();
        match current {
            Token::NumLit(num_lit) => {
                self.start();
                self.advance(1);
                Ok(Expr::Lit(Lit::NumLit(
                    num_lit.clone().with_span(self.span()),
                )))
            }

            Token::Ident(_) => {
                self.start();
                let path = self.parse_path()?;

                if self._eat(Token::LParen) {
                    let args = self.parse_arg_list()?;

                    self.expect(Token::RParen)?;
                    return Ok(Expr::FunCall(
                        FnCall::new()
                            .with_path(path)
                            .with_args(args)
                            .with_span(self.span()),
                    ));
                }

                Ok(Expr::Path(path.with_span(self.span())))
            }

            Token::LParen => {
                self.advance(1);
                let expr = self.parse_expr();
                self.expect(Token::RParen)?;
                expr
            }

            _ => Err(ParseError::from(format!(
                "Expected number or identifier, found '{}'",
                current
            ))),
        }
    }

    pub fn parse_arg_list(&mut self) -> ParseResult<Vec<Expr>> {
        let mut args = Vec::new();

        if self.current() == &Token::RParen {
            // There are no arguments
            Ok(args)
        } else {
            args.push(self.parse_expr()?);
            while self.current() == &Token::Comma {
                self.advance(1);
                args.push(self.parse_expr()?);
            }
            Ok(args)
        }
    }

    pub fn parse_path(&mut self) -> ParseResult<Path> {
        // Start a new span
        self.start();

        let mut segments = Vec::new();
        segments.push(self.parse_ident()?);

        while self.current() == &Token::ColonColon {
            self.expect(Token::ColonColon)?;
            segments.push(self.parse_ident()?);
        }

        Ok(Path::new().with_segments(segments).with_span(self.span()))
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

    fn current_span(&self) -> Span {
        if self.index > self.input.len() {
            return Span::new();
        }

        self.spans[self.index].clone()
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

    fn previous_span(&mut self) -> Span {
        let mut end = self.starts[0].clone();

        if self.index > 0 && self.index <= self.spans.len() {
            end = self.spans[self.index - 1].clone();
        }

        let span = Span::new()
            .from(self.starts.last().unwrap().from.clone())
            .to(end.to.clone());

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
