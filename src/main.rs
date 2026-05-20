mod lexer;
mod ast;
mod parser;
mod sema;
mod codegen;
mod formatter;
mod interpreter;
mod cli;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    cli::execute_subcommand(&args);
}