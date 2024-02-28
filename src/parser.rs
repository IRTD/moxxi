use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::*;

use std::any::TypeId;

pub struct Parser {
    lexer: Lexer,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut p = Parser {
            lexer,
            cur_token: None,
            peek_token: None,
            errors: Vec::new(),
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn errors<'a>(&'a self) -> &'a Vec<String> {
        &self.errors
    }

    pub fn peek_error(&mut self, t: TokenType) {
        let err = format!(
            "Expected next token to be {:?}, instead got {:?}",
            t, self.peek_token
        );
        self.errors.push(err);
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.next_token());
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut prog = Program::new();

        while self.cur_token.as_ref().unwrap().ty != TokenType::EOF {
            match self.parse_statement() {
                Some(stmt) => prog.statements.push(stmt),
                None => {}
            }

            self.next_token();
        }

        Some(prog)
    }

    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.as_ref()?.ty {
            TokenType::Let => return self.parse_let_stmt(),
            TokenType::Return => return self.parse_return_statement(),
            _ => None,
        }
    }

    pub fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = self.cur_token.clone()?;
        self.next_token();

        while !self
            .cur_token
            .as_ref()
            .is_some_and(|i| i.ty == TokenType::Semicolon)
        {
            self.next_token();
        }
        // TODO well, expressions

        let stmt = ReturnStmt {
            token,
            value: Box::new(DummyExpr),
        };
        Some(Box::new(stmt))
    }

    pub fn parse_let_stmt(&mut self) -> Option<Box<dyn Statement>> {
        let token = self.cur_token.clone()?;
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }
        let id_token = self.cur_token.as_ref()?;
        let ident = Identifier {
            value: id_token.literal.to_string(),
            token: id_token.clone(),
        };
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        // TODO parse Expressions aswell, currently ignored

        while !self
            .cur_token
            .as_ref()
            .is_some_and(|t| t.ty == TokenType::Semicolon)
        {
            self.next_token();
        }
        let stmt = LetStmt {
            token,
            ident,
            value: Box::new(DummyExpr),
        };
        Some(Box::new(stmt))
    }

    pub fn expect_peek(&mut self, exp: TokenType) -> bool {
        if self.peek_token.as_ref().is_some_and(|t| t.ty == exp) {
            self.next_token();
            true
        } else {
            self.peek_error(exp);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_parse_error(p: &mut Parser) {
        if p.errors().is_empty() {
            return;
        }

        eprintln!("Parser has {}, errors", p.errors().len());

        for err in p.errors() {
            eprintln!("ParseErr: {}", err);
        }
        panic!("Parser had Errors");
    }

    #[test]
    fn parses_let_statement() {
        let sample = "
            let x = 4;
            let y = 9;
            let foobar = 838383;
        ";
        let l = Lexer::new(sample);
        let mut p = Parser::new(l);
        let prog = p.parse_program();
        assert!(prog.is_some());
        let prog = prog.unwrap();
        check_parse_error(&mut p);
        assert_eq!(prog.statements.len(), 3, "Statement length too short");
        let exp_ids = vec!["x", "y", "foobar"];
        for (i, stmt) in prog.statements.into_iter().enumerate() {
            let exp = exp_ids[i];
            assert_eq!(stmt.token_literal(), "let", "No 'let' token literal");
            assert_eq!(
                stmt.type_id(),
                TypeId::of::<LetStmt>(),
                "Parsed Statement is NOT LetStmt"
            );
            assert_eq!(stmt.ident().value, exp, "Ident value does not match");
            assert_eq!(
                stmt.ident().token_literal(),
                exp,
                "Token literal of ident does not match"
            );
        }
    }

    #[test]
    #[should_panic]
    fn emits_error_on_false_syntax() {
        let sample = "
            let x = 4;
            let y = 120;
            let 3242;
        ";
        let l = Lexer::new(sample);
        let mut p = Parser::new(l);
        let prog = p.parse_program();
        assert!(prog.is_some());
        check_parse_error(&mut p);
    }

    #[test]
    fn parses_return_statement() {
        let sample = "
            return 5;
            return 53;
            return x;
        ";
        let l = Lexer::new(sample);
        let mut p = Parser::new(l);
        let prog = p.parse_program();
        assert!(prog.is_some());
        let prog = prog.unwrap();
        check_parse_error(&mut p);
        assert_eq!(prog.statements.len(), 3);
        for stmt in prog.statements {
            assert_eq!(stmt.token_literal(), "return");
            assert_eq!(stmt.type_id(), TypeId::of::<ReturnStmt>());
        }
    }
}
