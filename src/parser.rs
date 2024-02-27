use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::*;

use std::any::TypeId;

pub struct Parser {
    lexer: Lexer,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut p = Parser {
            lexer,
            cur_token: None,
            peek_token: None,
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.next_token());
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut prog = Program::new();

        while self.cur_token.as_ref().unwrap().ty != TokenType::EOF {
            match self.parse_statement() {
                Some(stmt) => prog.statements.push(Box::new(stmt)),
                None => {}
            }

            self.next_token();
        }

        Some(prog)
    }

    pub fn parse_statement(&mut self) -> Option<impl Statement> {
        match self.cur_token.as_ref()?.ty {
            TokenType::Let => return self.parse_let_stmt(),
            _ => None,
        }
    }

    pub fn parse_let_stmt(&mut self) -> Option<LetStmt> {
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
        Some(stmt)
    }

    pub fn expect_peek(&mut self, exp: TokenType) -> bool {
        if self.peek_token.as_ref().is_some_and(|t| t.ty == exp) {
            self.next_token();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
