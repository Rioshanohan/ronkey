use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT,
    INT,

    ASSIGN,
    PLUS,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
}

pub const KEYWORDS: [(&str, TokenType); 2] = [("fn", TokenType::FUNCTION), ("let", TokenType::LET)];

pub fn lookup_ident(ident: &str) -> TokenType {
    for (keyword, token_type) in KEYWORDS.iter() {
        if &ident == keyword {
            return *token_type;
        }
    }
    TokenType::IDENT
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str) -> Self {
        Self {
            token_type,
            literal: literal.to_owned(),
        }
    }
}
