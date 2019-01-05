#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    Ident(String),
    Int(String),

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
    If,
    Else,
    Return,
    True,
    False,
}
