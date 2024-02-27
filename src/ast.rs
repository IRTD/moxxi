use crate::token::*;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {}
pub trait Expression: Node {}

pub struct Program {
    statements: Vec<Box<dyn Node>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        match self.statements.first() {
            Some(s) => s.token_literal(),
            None => String::new(),
        }
    }
}

pub struct Identifier {
    token: Token,
    value: String,
}

pub struct LetStmt {
    token: Token,
    ident: Identifier,
    value: Box<dyn Expression>,
}
