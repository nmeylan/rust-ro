extern crate bnf;
use crate::scanner::Scanner;

mod token;
mod scanner;
mod ast_generator;
mod ast;
mod parser_state;

fn main() {
    let code = r#"
        .@x = 2;
    "#;
    let mut scanner = Scanner::new(code.as_bytes().to_vec());
    scanner.scan();
    // ast_generator::generate()

}