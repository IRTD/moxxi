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
        l
    }

    pub fn skip(&mut self) {
        while [' ', '\n', '\t', '\r'].contains(&self.ch) {
            self.read_char();
        }
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
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
}
