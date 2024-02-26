use std::str::FromStr;

use crate::token::*;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(i: impl ToString) -> Self {
        let mut l = Lexer {
            input: i.to_string().chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\n',
        };
        l.read_char();
        l.read_char();
        l
    }

    pub fn skip(&mut self) {
        while [' ', '\n', '\t', '\r'].contains(&self.ch) {
            self.read_char();
        }
    }

    pub fn peek(&self) -> char {
        println!(
            "self.pos = {}; self.read_pos = {}",
            self.position, self.read_position
        );
        if self.read_position >= self.input.len() {
            return '\0';
        }
        self.input[self.read_position]
    }

    pub fn read_ident(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() {
            self.read_char();
        }
        self.input[pos..self.position]
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn read_num(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        self.input[pos..self.position]
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn read_char(&mut self) {
        if self.position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn read_double(&mut self) -> Option<String> {
        println!("self.ch = {:#?}", self.ch);
        println!("self.peek() = {:#?}", self.peek());
        match self.ch {
            ':' => {
                if self.peek() == ':' {
                    Some(())
                } else {
                    None
                }
            }
            '<' => {
                if self.peek() == '<' {
                    Some(())
                } else {
                    None
                }
            }
            _ => None,
        }?;
        let prev = self.ch;
        // self.read_char();
        Some(format!("{}{}", prev, self.ch))
    }

    pub fn next_token(&mut self) -> Token {
        self.skip();
        if self.ch.is_digit(10) {
            let dig = self.read_num();
            return Token::new(TokenType::Int, dig);
        } else if self.ch.is_ascii_alphabetic() {
            let id = self.read_ident();
            return Token::new(TokenType::Ident, id);
        }
        let (ty, ch) = match self.read_double() {
            Some(double) => {
                println!("Double: {double}");
                let ty = TokenType::from_str(&double).unwrap_or(TokenType::Illegal);
                (ty, double)
            }
            None => {
                let ty = TokenType::from_str(&self.ch.to_string()).unwrap_or(TokenType::Illegal);
                (ty, self.ch.to_string())
            }
        };
        self.read_char();
        Token::new(ty, ch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval_tokens(expected: Vec<Token>, sample: impl ToString) {
        let mut l = Lexer::new(sample);
        for (i, exp) in expected.iter().enumerate() {
            let tok = l.next_token();
            assert_eq!(*exp, tok, "At expected index {i} :: Expected, Sample");
        }
    }

    #[test]
    fn read_singles() {
        let sample = r#";[]"<>{}()+-#!:=,*/"#;
        let exp = vec![
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::LBracket, "["),
            Token::new(TokenType::RBracket, "]"),
            Token::new(TokenType::DoubleQuote, r#"""#),
            Token::new(TokenType::LessThan, "<"),
            Token::new(TokenType::GreaterThan, ">"),
            Token::new(TokenType::LBrace, "{"),
            Token::new(TokenType::RBrace, "}"),
            Token::new(TokenType::LParen, "("),
            Token::new(TokenType::RParen, ")"),
            Token::new(TokenType::Plus, "+"),
            Token::new(TokenType::Minus, "-"),
            Token::new(TokenType::Hash, "#"),
            Token::new(TokenType::Bang, "!"),
            Token::new(TokenType::Colon, ":"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Asterisk, "*"),
            Token::new(TokenType::Slash, "/"),
            Token::new(TokenType::EOF, "\0"),
        ];
        eval_tokens(exp, sample);
    }

    #[test]
    fn read_doubles() {
        let sample = "<< :: == !=";
        let exp = vec![
            Token::new(TokenType::DoubleLessThan, "<<"),
            Token::new(TokenType::DoubleColon, "::"),
            Token::new(TokenType::Equal, "=="),
            Token::new(TokenType::NotEqual, "!="),
        ];
        eval_tokens(exp, sample);
    }
}
