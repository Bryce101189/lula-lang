#[macro_use]
extern crate lazy_static;

use std::{env, fs::File, io::Read};

use lexer::Lexer;
use parser::Parser;

pub mod error;
pub mod expr;
pub mod lexer;
pub mod parser;
pub mod statement;
pub mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for input file
    if args.len() < 2 {
        eprintln!("Fatal error: No input file provided");
        return;
    }

    let in_file_path = args.get(1).unwrap();

    // Check if input file uses the '.lla' file extension
    if !in_file_path.ends_with(".lla") {
        eprintln!("Fatal error: Input file does not use the '.lla' file extension");
        return;
    }

    // Attempt to open input file
    let mut in_file = match File::open(in_file_path) {
        Ok(file) => file,
        Err(..) => {
            eprintln!(
                "Fatal error: Failed to open file '{}'\n~ are you sure this file exists?",
                in_file_path
            );
            return;
        }
    };

    // Read file contents into string
    let mut in_file_contents = String::new();

    if in_file.read_to_string(&mut in_file_contents).is_err() {
        eprintln!("Fatal error: Failed to read file contents into string");
        return;
    }

    // Lex file contents into a vector of tokens
    let mut lexer = Lexer::new(in_file_path.to_owned(), in_file_contents);
    let tokens = match lexer.collect_tokens() {
        Some(t) => t,
        None => return,
    };

    // Parse tokens into a vector of statements
    let mut parser = Parser::new(in_file_path.to_owned(), tokens);

    let statements = match parser.collect_statements() {
        Some(s) => s,
        None => return,
    };

    // Interpret statements sequentially
    for statement in statements {
        if !statement.interpret() {
            // Return if a runtime error occurs
            return;
        }
    }
}
