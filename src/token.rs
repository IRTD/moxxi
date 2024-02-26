use std::str::FromStr;
use strum::EnumString;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub literal: String,
    pub ty: TokenType,
}

impl Token {
    pub fn new(ty: TokenType, l: impl ToString) -> Self {
        Token {
            literal: l.to_string(),
            ty,
        }
    }
}

#[derive(Debug, EnumString, Clone, Copy, PartialEq)]
pub enum TokenType {
    Illegal,
    Int,
    Ident,
    #[strum(serialize = "\0")]
    EOF,
    #[strum(serialize = "let")]
    Let,
    #[strum(serialize = "(")]
    LParen,
    #[strum(serialize = ")")]
    RParen,
    #[strum(serialize = "{")]
    LBrace,
    #[strum(serialize = "}")]
    RBrace,
    #[strum(serialize = "[")]
    LBracket,
    #[strum(serialize = "]")]
    RBracket,
    #[strum(serialize = "=")]
    Assign,
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "-")]
    Minus,
    #[strum(serialize = ";")]
    Semicolon,
    #[strum(serialize = "<")]
    LessThan,
    #[strum(serialize = ">")]
    GreaterThan,
    #[strum(serialize = "if")]
    If,
    #[strum(serialize = "else")]
    Else,
    #[strum(serialize = "#")]
    Hash,
    #[strum(serialize = "!")]
    Bang,
    #[strum(serialize = ":")]
    Colon,
    #[strum(serialize = r#"""#)]
    DoubleQuote,
    #[strum(serialize = "==")]
    Equal,
    #[strum(serialize = "!=")]
    NotEqual,
    #[strum(serialize = "::")]
    DoubleColon,
    #[strum(serialize = "<<")]
    DoubleLessThan,
    #[strum(serialize = "return")]
    Return,
    #[strum(serialize = "show")]
    Show,
    #[strum(serialize = "fn")]
    Func,
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = "*")]
    Asterisk,
    #[strum(serialize = "/")]
    Slash,
    #[strum(serialize = "true")]
    True,
    #[strum(serialize = "false")]
    False,
}
