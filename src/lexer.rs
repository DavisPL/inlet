use std::{fmt::format, str::Chars};

use crate::{
    ast::{Ident, NumLit},
    error::LexError,
    span::{Location, Span},
    token::Token,
};

type LexResult<T> = Result<T, LexError>;

pub struct Lexer<'a> {
    source: &'a [char],
    spans: Vec<Span>,
    index: usize,
    line: usize,
    column: usize,
    start: Location,
    end: Location,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [char]) -> Self {
        Lexer {
            source: input,
            spans: vec![],
            index: 0,
            line: 1,
            column: 1,
            start: Location::new(),
            end: Location::new(),
        }
    }

    pub fn next(&mut self) -> LexResult<Token> {
        while self.index < self.source.len() && self.current().is_whitespace() {
            self.step(1);
        }

        if self.index >= self.source.len() {
            return Ok(Token::EOF);
        }

        let current = self.current();
        if current.is_alphabetic() {
            // Mark the beginning of this token
            let i = self.index;
            self.start();
            self.step(1);

            while self.current().is_alphanumeric() {
                self.step(1);
            }

            self.complete();
            let raw: String = self.source[i..self.index].iter().collect();

            match raw.as_str() {
                "fn" => Ok(Token::KwFn),
                "let" => Ok(Token::KwLet),
                "mod" => Ok(Token::KwMod),
                _ => Ok(Token::Ident(Ident::new().with_raw(raw))),
            }
        } else if current.is_numeric() {
            // Mark the beginning of this token
            let i = self.index;
            self.start();
            self.step(1);

            while self.current().is_numeric() {
                self.step(1);
            }

            self.complete();
            let value: String = self.source[i..self.index].iter().collect();
            let value: i32 = value.parse().map_err(|_| {
                LexError::from(format!("Could not convert '{}' into an `i32`", value))
            })?;

            return Ok(Token::NumLit(NumLit::new().with_value(value)));
        } else {
            // Must be a symbol of some kind... mark the beginning
            let i = self.index;
            self.start();

            match current {
                // Single character lookahead
                '+' => {
                    self.step(1);
                    self.complete();
                    return Ok(Token::Plus);
                }

                '(' => {
                    self.step(1);
                    self.complete();
                    return Ok(Token::LParen);
                }

                ')' => {
                    self.step(1);
                    self.complete();
                    return Ok(Token::RParen);
                }

                '[' => {
                    self.step(1);
                    self.complete();
                    return Ok(Token::LBracket);
                }

                ']' => {
                    self.step(1);
                    self.complete();
                    return Ok(Token::RBracket);
                }

                '{' => {
                    self.step(1);
                    self.complete();
                    return Ok(Token::LBrace);
                }

                '}' => {
                    self.step(1);
                    self.complete();
                    return Ok(Token::RBrace);
                }

                '*' => {
                    self.step(1);
                    self.complete();
                    return Ok(Token::Star);
                }

                '-' => {
                    self.step(1);
                    if self.current() == '>' {
                        self.step(1);
                        self.complete();
                        return Ok(Token::RArrow);
                    } else {
                        return Err(LexError::from(format!(
                            "Expected '->' but found {}",
                            self.current()
                        )));
                    }
                }

                // Two character lokahead
                ':' => {
                    if self.lookahead(1) == ':' {
                        self.step(2);
                        self.complete();
                        return Ok(Token::ColonColon);
                    } else {
                        self.step(1);
                        self.complete();
                        return Ok(Token::Colon);
                    }
                }

                _ => {
                    self.step(1);
                    self.complete();
                    return Err(LexError::from(format!(
                        "Unexpected character '{}'",
                        current
                    )));
                }
            }
        }
    }

    pub fn lex(&mut self) -> Result<(Vec<Token>, Vec<Span>), LexError> {
        let mut tokens = Vec::new();
        let mut token = self.next()?;

        while token != Token::EOF {
            tokens.push(token.clone());
            token = self.next()?;
        }

        let loc = Location::new()
            .with_column(self.column)
            .with_line(self.line);
        self.spans.push(Span::new().from(loc.clone()).to(loc));
        tokens.push(Token::EOF);

        Ok((tokens, self.spans.clone())) // TODO: Find a way to avoid cloning here
    }

    fn start(&mut self) {
        self.start.set_column(self.column);
        self.start.set_line(self.line);
    }

    fn end(&mut self) {
        self.end.set_column(self.column);
        self.end.set_line(self.line);
    }

    /// Completes a span, using `self.marker` for the `from` location. Should be called every time a new token is collected.
    fn complete(&mut self) {
        self.spans
            .push(Span::new().from(self.start.clone()).to(self.end.clone()));
    }

    fn current(&self) -> char {
        self.lookahead(0)
    }

    fn lookahead(&self, n: usize) -> char {
        if self.index + n >= self.source.len() {
            return '\0';
        }

        self.source[self.index + n]
    }

    fn step(&mut self, n: usize) {
        for _ in 0..n {
            // Mark the end of the current span
            self.end();

            self.index += 1;

            if self.index >= self.source.len() {
                break;
            }

            self.column += 1;
            if self.source[self.index] == '\n' {
                while self.index <= self.source.len() && self.source[self.index] == '\n' {
                    self.index += 1;
                    self.line += 1;
                }
                self.column = 1;
            }
        }
    }
}
