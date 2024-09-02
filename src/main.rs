use std::{fs, path::PathBuf};

use lexer::Lexer;
use parser::Parser;

mod ast;
mod diagnostic;
mod error;
mod lexer;
mod parser;
mod span;
mod token;

fn main() {
    let path = PathBuf::from("./c/main.inlet");

    // First, we'll read the entire file as a string
    let source = fs::read_to_string(path)
        .expect("Could not read file. Does it exist, and is it formatted correctly?");
    let slice = source.chars().collect::<Vec<char>>();

    // Next, we'll run the lexer
    let mut lexer = Lexer::new(&slice);
    let (tokens, spans) = lexer.lex().unwrap(); // TODO: Figure out a better way to handle errors

    println!("{:#?}", tokens);
    println!("{:#?}", spans);

    // Next, we'll run the parser
    let mut parser = Parser::new(&tokens, &spans);
    let ast = parser.parse_file().unwrap(); // TODO: Figure out a better way to handle errors

    println!("{:#?}", ast);
}
