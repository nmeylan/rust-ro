#![feature(in_band_lifetimes)]

use crate::scanner::Scanner;

mod token;
mod scanner;

fn main() {
    let code = r#"
        .@x = 2;
        .@str = "i'm a string";
        // comment
        .@x = 2;
        .@array[0] = 100
        /*
        * multiline
        */
        if (.@x <= 100) {
          .@x = 99;
        }
        return .@x+1;
    "#;
    let mut scanner = Scanner::new(code.as_bytes().to_vec());
    scanner.scan();
}