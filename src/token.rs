use std::str::FromStr;
use strum::EnumString;

#[derive(Debug, Clone)]
pub struct Token {
    literal: String,
    ty: TokenType,
}

#[derive(Debug, EnumString, Clone, Copy)]
pub enum TokenType {
    #[strum(serialize = "\0")]
    EOF,
}
