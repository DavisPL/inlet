use std::{
    collections::HashMap,
    fs,
    iter::successors,
    panic,
    path::{Path, PathBuf},
    sync::Arc,
};

use serde::Deserialize;

use lexer::Lexer;
use parser::Parser;
use semantics::{Analysis, FunctionAnalysis, FunctionData, IdentAnalysis, OriginAnalysis};

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

use clap::Parser as ClapParser;
use symbol::SymbolTable;

/// The Inlet Compiler.
#[derive(ClapParser)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// Specify the root directory of your project.
    #[arg(short, long)]
    path: PathBuf,
}

#[derive(Deserialize)]
struct Manifest {
    package: Package,
    dependencies: HashMap<String, Dependency>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
}

#[derive(Deserialize)]
struct Dependency {
    path: PathBuf,
}

struct ProcessContext {
    dependencies: HashMap<String, PathBuf>,
    functions: SymbolTable<FunctionData>,
    binary_exists: bool,
}

impl ProcessContext {
    pub fn new() -> Self {
        ProcessContext {
            dependencies: HashMap::new(),
            functions: SymbolTable::new(),
            binary_exists: false,
        }
    }
}

fn process_crate(path: &Path, ctx: &mut ProcessContext) {
    // First, let's read the manifest to see if there are any dependencies we should look at first
    let manifest = &fs::read_to_string(path.join("Inlet.toml")).expect(&format!(
        "Could not find `Inlet.toml` in {}. Does it exist?",
        path.to_str().unwrap()
    ));

    let manifest: Manifest = toml::from_str(manifest).expect(&format!(
        "Failed to parse `Inlet.toml` in {}. Is it formatted correctly?",
        path.to_str().unwrap()
    ));

    // Next, let's figure out whether this is a binary or library crate
    let binary = fs::exists(path.join("main.inlet")).unwrap();
    let library = fs::exists(path.join("lib.inlet")).unwrap();

    if !binary && !library {
        panic!("Crate '{}' has neither a 'main.inlet' (binary entrypoint) or 'lib.rs' (library entrypoint).", manifest.package.name);
    }

    if ctx.binary_exists && binary {
        panic!(
            "Binary crate '{}' cannot serve as a dependency for another binary crate.",
            manifest.package.name
        );
    }

    for (dep_name, dep) in manifest.dependencies {
        if ctx.dependencies.contains_key(&dep_name) {
            if ctx.dependencies[&dep_name] != dep.path {
                panic!(
                    "Dependency '{}' of crate '{}' has already been analyzed at a different path.",
                    dep_name, manifest.package.name
                )
            }
        } else {
            // This dependency hasn't been processed yet... let's do it!
            process_crate(&path.join(&dep.path), ctx);
        }
    }

    // Now we can process this crate! Right now, every crate only has ONE file
    let file = if binary { "main.inlet" } else { "lib.inlet" };
    let source = fs::read_to_string(path.join(file)).expect(&format!(
        "Couldn't find file '{}' in crate '{}'",
        path.join(file).to_str().unwrap(),
        manifest.package.name
    ));

    // We'll begin by lexing the source
    let slice = source.chars().collect::<Vec<char>>();

    // Next, we'll run the lexer
    let mut lexer = Lexer::new(&slice);
    let (tokens, spans) = lexer.lex().unwrap();

    // Then, we'll run the parser
    let mut parser = Parser::new(&tokens, &spans);
    let ast = parser.parse_file().unwrap(); // TODO: Figure out a better way to handle errors

    // Next, we'll perform some simple semantic analysis
    // For starters, let's collect all function definitions and then make sure all identifiers are defined
    let functions = FunctionAnalysis::new(&ast, manifest.package.name, &ctx.functions)
        .analyze()
        .expect("TODO: Handle function collection errors properly.");

    ctx.functions = functions;

    let ident_analysis = IdentAnalysis::new(&ast, &ctx.functions).analyze();
    if let Err(errors) = ident_analysis {
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
            let col_num_len = (error.span.from.line).to_string().len();

            println!("[ERROR] {}\n", error.message);
            println!("{}:{}", error.span.from.line, line);
            println!("{} {}\n", " ".repeat(col_num_len), marker);
        }
    }

    // That's out of the way! Now, let's run the origin analysis
    let origin_analysis = OriginAnalysis::new(&ast, &ctx.functions, "example".to_string()).analyze();

    if let Err(errors) = origin_analysis {
        // We encountered one or more semantic errors... print them
        for error in errors {
            let mut r = error.span.from.line;
            let mut c = error.span.from.column;

            if r > 0 {
                r -= 1;
            }

            if c > 0 {
                c -= 1;
            }

            // Find the source line with this error
            let line = source
                .split('\n')
                .nth(r)
                .expect("Could not locate original source code line. This is a bug!");

            let length = if error.span.to.line > error.span.from.line {
                line.len() - error.span.from.column // TODO: Test this
            } else {
                error.span.to.column - error.span.from.column + 1
            };

            let marker = " ".repeat(c) + &"~".repeat(length);
            let col_num_len = (error.span.from.line).to_string().len();

            println!("[ERROR] {}\n", error.message);
            println!("{}:{}", error.span.from.line, line);
            println!("{} {}\n", " ".repeat(col_num_len), marker);
        }
    }
}

fn main() {
    let arguments = Arguments::parse();
    let root = arguments.path;

    let mut ctx = ProcessContext::new();
    process_crate(&root, &mut ctx);

    return;
    // let path = PathBuf::from("./example/main.inlet");

    // // First, we'll read the entire file as a string
    // let source = fs::read_to_string(path)
    //     .expect("Could not read file. Does it exist, and is it formatted correctly?");
    // let slice = source.chars().collect::<Vec<char>>();

    // // Next, we'll run the lexer
    // let mut lexer = Lexer::new(&slice);
    // let (tokens, spans) = lexer.lex().unwrap(); // TODO: Figure out a better way to handle errors

    // // Next, we'll run the parser
    // let mut parser = Parser::new(&tokens, &spans);
    // let ast = parser.parse_file().unwrap(); // TODO: Figure out a better way to handle errors

    // // Next, we'll run some semantic analysis
    // // We'll begin by collecting all the function declarations
    // let functions = FunctionAnalysis::new(&ast, "example".to_string())
    //     .analyze()
    //     .expect("TODO: Handle function collection errors properly.");

    // let ident_analysis = IdentAnalysis::new(&ast, &functions).analyze();
    // if let Err(errors) = ident_analysis {
    //     // We encountered one or more semantic errors... print them
    //     for error in errors {
    //         // Find the source line with this error
    //         let line = source
    //             .split('\n')
    //             .nth(error.span.from.line - 1)
    //             .expect("Could not locate original source code line. This is a bug!");

    //         let length = if error.span.to.line > error.span.from.line {
    //             line.len() - error.span.from.column // TODO: Test this
    //         } else {
    //             error.span.to.column - error.span.from.column + 1
    //         };

    //         let marker = " ".repeat(error.span.from.column - 1) + &"~".repeat(length);
    //         let col_num_len = (error.span.from.line).to_string().len();

    //         println!("[ERROR] {}\n", error.message);
    //         println!("{}:{}", error.span.from.line, line);
    //         println!("{} {}\n", " ".repeat(col_num_len), marker);
    //     }
    // }

    // let origin_analysis = OriginAnalysis::new(&ast, &functions, "example".to_string()).analyze();

    // if let Err(errors) = origin_analysis {
    //     // We encountered one or more semantic errors... print them
    //     for error in errors {
    //         let mut r = error.span.from.line;
    //         let mut c = error.span.from.column;

    //         if r > 0 {
    //             r -= 1;
    //         }

    //         if c > 0 {
    //             c -= 1;
    //         }

    //         // Find the source line with this error
    //         let line = source
    //             .split('\n')
    //             .nth(r)
    //             .expect("Could not locate original source code line. This is a bug!");

    //         let length = if error.span.to.line > error.span.from.line {
    //             line.len() - error.span.from.column // TODO: Test this
    //         } else {
    //             error.span.to.column - error.span.from.column + 1
    //         };

    //         let marker = " ".repeat(c) + &"~".repeat(length);
    //         let col_num_len = (error.span.from.line).to_string().len();

    //         println!("[ERROR] {}\n", error.message);
    //         println!("{}:{}", error.span.from.line, line);
    //         println!("{} {}\n", " ".repeat(col_num_len), marker);
    //     }
    // }
}
