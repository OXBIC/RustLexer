#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Let,
    Ident(String),
    Int(i64),
    Assign,
    Plus,
    Semicolon,
    LParen,
    RParen,
    EOF,
    Illegal(char),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub linea: usize,
    pub columna: usize,
}
