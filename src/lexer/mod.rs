mod token;

use regex;

pub struct Lexer {
    buf: String,
    pos: usize,
    last_symbol: Option<char>,
    last_token: Option<token::Token>,
}

impl Lexer {
    pub fn new(src: &'static str) -> Lexer {
        Lexer {
            buf: src.to_string(),
            pos: 0,
            last_symbol: None,
            last_token: None,
        }
    }

    fn lex_word(&mut self) -> Option<token::Token> {
        unimplemented!()
    }

    fn lex_ident(&mut self) -> Option<token::Token> {
        unimplemented!()
    }

    fn bump(&mut self) {
        self.last_symbol = self.buf[self.pos..].chars().next();
        self.pos += 1;
    }
}

impl Iterator for Lexer {
    type Item = token::Token;
    fn next(&mut self) -> Option<token::Token> {
        if let Some((_, o)) = (regex!(r"^\s*")).find(&self.buf[self.pos..]) {
            self.pos += o;
        }

        if self.pos >= self.buf.len() {
            return None;
        }

        match self.buf[self.pos..].chars().next().unwrap() {
            '+' => {
                self.bump();
                Some(token::Token::Add)
            }
            '-' => {
                self.bump();
                Some(token::Token::Sub)
            }

            '*' => {
                self.bump();
                Some(token::Token::Mul)
            }
            '/' => {
                self.bump();
                Some(token::Token::Div)
            }
            '(' => {
                self.bump();
                Some(token::Token::LParen)
            }
            ')' => {
                self.bump();
                Some(token::Token::RParen)
            }
            '<' => {
                self.bump();
                Some(token::Token::Lt)
            }
            '>' => {
                self.bump();
                Some(token::Token::Gt)
            }

            x if x.is_alphabetic() => None,
            x if x.is_numeric() => None,

            _ => None,
        }
    }
}
