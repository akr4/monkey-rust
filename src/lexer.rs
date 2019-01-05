use std::collections::HashMap;
#[cfg(test)]
use std::fmt;

use crate::token::Token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

#[cfg(test)]
fn escape(s: &str) -> String {
    s.replace("\n", "\\n")
        .replace("\r", "\\r")
        .replace("\t", "\\t")
}

#[cfg(test)]
impl fmt::Debug for Lexer {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let input = self
            .input
            .get(self.position..self.input.len().min(self.position + 10));
        write!(
            f,
            r#"Lexer {{ input: "...{}...", position: {:?}, read_position: {:?}, ch: {:?} }}"#,
            input.map(|x| escape(x)).unwrap_or("".to_string()),
            self.position,
            self.read_position,
            self.ch
        )
    }
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut x = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: None,
        };
        (&mut x).read_char();
        x
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else if let Some(cs) = self.input.get(self.read_position..=self.read_position) {
            let c = cs.chars().next().unwrap();
            self.ch = Some(c);
            self.position = self.read_position;
            self.read_position += 1;
        } else {
            panic!("read_position exceeds input length.")
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else if let Some(cs) = self.input.get(self.read_position..=self.read_position) {
            let c = cs.chars().next().unwrap();
            Some(c)
        } else {
            panic!("read_position exceeds input length.")
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            Some('=') => match self.peek_char() {
                Some(c) if c == '=' => {
                    self.read_char();
                    Token::Eq
                }
                _ => Token::Assign,
            },
            Some(';') => Token::Semicolon,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some(',') => Token::Comma,
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('!') => match self.peek_char() {
                Some(c) if c == '=' => {
                    self.read_char();
                    Token::NotEq
                }
                _ => Token::Bang,
            },
            Some('*') => Token::Asterisk,
            Some('/') => Token::Slash,
            Some('<') => Token::Lt,
            Some('>') => Token::Gt,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some(c) => {
                if is_letter(c) {
                    let ident = self.read_identifier();
                    return lookup_ident(ident).unwrap_or_else(|| Token::Ident(ident.to_string()));
                } else if is_digit(c) {
                    return Token::Int(self.read_number().to_string());
                } else {
                    Token::Illegal
                }
            }
            _ => Token::Eof,
        };

        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> &str {
        let start = self.position;
        while let Some(c) = self.ch {
            if is_letter(c) {
                self.read_char();
            } else {
                break;
            }
        }
        &self.input[start..self.position]
    }

    fn read_number(&mut self) -> &str {
        let start = self.position;
        while let Some(c) = self.ch {
            if is_digit(c) {
                self.read_char();
            } else {
                break;
            }
        }
        &self.input[start..self.position]
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.ch {
            if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
                self.read_char();
            } else {
                break;
            }
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self.next_token() {
            Token::Eof => None,
            token => Some(token),
        }
    }
}

fn is_letter(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' => true,
        _ => false,
    }
}

fn is_digit(c: char) -> bool {
    match c {
        '0'..='9' => true,
        _ => false,
    }
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Token> = {
        let mut m = HashMap::new();
        m.insert("fn", Token::Function);
        m.insert("let", Token::Let);
        m.insert("if", Token::If);
        m.insert("else", Token::Else);
        m.insert("return", Token::Return);
        m.insert("true", Token::True);
        m.insert("false", Token::False);
        m
    };
}

fn lookup_ident(ident: &str) -> Option<Token> {
    KEYWORDS.get(ident).cloned()
}
