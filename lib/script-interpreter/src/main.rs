extern crate bnf;
extern crate core;

use crate::scanner::Scanner;

mod token;
mod scanner;
mod generator;
mod ast;
mod parser_state;
pub mod ast_node;
mod parser;

fn main() {
    let code = r#"
        .@x = 2;
    "#;
    let mut scanner = Scanner::new(code.as_bytes().to_vec());
    let tokens = scanner.scan().unwrap();
    generator::generate();

}
