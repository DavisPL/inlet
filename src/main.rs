use std::{fs, iter::successors, path::PathBuf};

use lexer::Lexer;
use parser::Parser;
use semantics::{FunctionAnalysis, IdentAnalysis, SemanticEngine};

mod ast;
mod diagnostic;
mod error;
mod lexer;
mod parser;
mod semantics;
mod span;
mod symbol;
mod token;
mod visit;

fn main() {
    let path = PathBuf::from("./example/main.inlet");

    // First, we'll read the entire file as a string
    let source = fs::read_to_string(path)
        .expect("Could not read file. Does it exist, and is it formatted correctly?");
    let slice = source.chars().collect::<Vec<char>>();

    // Next, we'll run the lexer
    let mut lexer = Lexer::new(&slice);
    let (tokens, spans) = lexer.lex().unwrap(); // TODO: Figure out a better way to handle errors

    // println!("{:#?}", tokens);
    // println!("{:#?}", spans);

    // Next, we'll run the parser
    let mut parser = Parser::new(&tokens, &spans);
    let ast = parser.parse_file().unwrap(); // TODO: Figure out a better way to handle errors

    // println!("{:#?}", ast);

    // Next, we'll run some semantic analysis
    // We'll begin by collecting all the function declarations
    let functions = FunctionAnalysis::new(&ast, "example".to_string())
        .analyze()
        .expect("TODO: Handle function collection errors properly.");

    let mut sema_engine = SemanticEngine::new().with_analysis(IdentAnalysis::new(&ast, &functions));

    let sema_result = sema_engine.run();
    if let Err(errors) = sema_result {
        // We encountered one or more semantic errors... print them
        for error in errors {
            // Find the source line with this error
            let line = source
                .split('\n')
                .nth(error.span.from.line - 1)
                .expect("Could not locate original source code line. This is a bug!");

            let length = if error.span.to.line > error.span.from.line {
                line.len() - error.span.from.column // TODO: Test this
            } else {
                error.span.to.column - error.span.from.column + 1
            };

            let marker = " ".repeat(error.span.from.column - 1) + &"~".repeat(length);
            let col_num_len = (error.span.from.line - 1).to_string().len();

            println!("[ERROR] {}\n", error.message);
            println!("{}:{}", error.span.from.line, line);
            println!("{} {}\n", " ".repeat(col_num_len), marker);
        }
    }
}
