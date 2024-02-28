use crate::token::*;

pub trait Node: std::any::Any {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn ident<'a>(&'a self) -> &'a Identifier;
    fn value<'a>(&'a self) -> &'a Box<dyn Expression>;
}
pub trait Expression: Node {}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
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
    pub token: Token,
    pub value: String,
}
impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
impl Expression for Identifier {}

pub struct LetStmt {
    pub token: Token,
    pub ident: Identifier,
    pub value: Box<dyn Expression>,
}
impl Node for LetStmt {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
impl Statement for LetStmt {
    fn ident<'a>(&'a self) -> &'a Identifier {
        &self.ident
    }
    fn value<'a>(&'a self) -> &'a Box<dyn Expression> {
        &self.value
    }
}

pub struct ReturnStmt {
    token: Token,
    value: Box<dyn Expression>,
}
impl Node for ReturnStmt {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
impl Statement for ReturnStmt {
    fn ident<'a>(&'a self) -> &'a Identifier {
        panic!("Called ident on ReturnStmt");
    }

    fn value<'a>(&'a self) -> &'a Box<dyn Expression> {
        &self.value
    }
}

pub struct DummyExpr;
impl Node for DummyExpr {
    fn token_literal(&self) -> String {
        "DummyExpr".to_string()
    }
}
impl Expression for DummyExpr {}
