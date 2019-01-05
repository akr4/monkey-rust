use crate::lexer::Lexer;
use crate::token::Token;

#[test]
fn test_next_token() {
    let input = r"
let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
";

    let tests = [
        Token::Let,
        Token::Ident("five".to_string()),
        Token::Assign,
        Token::Int("5".to_string()),
        Token::Semicolon,
        Token::Let,
        Token::Ident("ten".to_string()),
        Token::Assign,
        Token::Int("10".to_string()),
        Token::Semicolon,
        Token::Let,
        Token::Ident("add".to_string()),
        Token::Assign,
        Token::Function,
        Token::LParen,
        Token::Ident("x".to_string()),
        Token::Comma,
        Token::Ident("y".to_string()),
        Token::RParen,
        Token::LBrace,
        Token::Ident("x".to_string()),
        Token::Plus,
        Token::Ident("y".to_string()),
        Token::Semicolon,
        Token::RBrace,
        Token::Semicolon,
        Token::Let,
        Token::Ident("result".to_string()),
        Token::Assign,
        Token::Ident("add".to_string()),
        Token::LParen,
        Token::Ident("five".to_string()),
        Token::Comma,
        Token::Ident("ten".to_string()),
        Token::RParen,
        Token::Semicolon,
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Int("5".to_string()),
        Token::Semicolon,
        Token::Int("5".to_string()),
        Token::Lt,
        Token::Int("10".to_string()),
        Token::Gt,
        Token::Int("5".to_string()),
        Token::Semicolon,
        Token::If,
        Token::LParen,
        Token::Int("5".to_string()),
        Token::Lt,
        Token::Int("10".to_string()),
        Token::RParen,
        Token::LBrace,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::RBrace,
        Token::Else,
        Token::LBrace,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::RBrace,
        Token::Int("10".to_string()),
        Token::Eq,
        Token::Int("10".to_string()),
        Token::Semicolon,
        Token::Int("10".to_string()),
        Token::NotEq,
        Token::Int("9".to_string()),
        Token::Semicolon,
        Token::Eof,
    ];

    for (actual, expected) in Lexer::new(input).zip(tests.iter()) {
        assert_eq!(actual, *expected);
    }
}
