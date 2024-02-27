#![allow(unused, dead_code)]

use token::TokenType;

mod ast;
mod lexer;
mod token;

fn main() {
    let sample = ",,<<,,";
    let mut l = lexer::Lexer::new(sample);
    loop {
        let tok = l.next_token();
        println!("{tok:?}\n");
        if tok.ty == TokenType::EOF {
            break;
        }
    }
}
