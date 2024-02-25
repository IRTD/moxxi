use std::str::FromStr;
use strum::EnumString;

#[derive(Debug, Clone)]
pub struct Token {
    literal: String,
    ty: TokenType,
}

#[derive(Debug, EnumString, Clone, Copy)]
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
    Minux,
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
