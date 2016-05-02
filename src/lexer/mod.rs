mod token;

pub struct Lexer<'a> {
    buf: String,
    pos: usize,
    last_symbol: Option<char>,
    last_token: Option<token::Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'static str) -> Lexer {
        Lexer {
            buf: src.to_string(),
            pos: 0,
            last_symbol: None,
            last_token: None,
        }
    }

    fn lex_word(&mut self) -> Option<token::Token> {
        if let Some((_, span)) = (regex!(r"^\s*")).find(&self.buf[self.pos..]) {
            let word = self.buf[self.pos .. self.pos + span].chars().as_str();
            let x = match word {
                "VAR" => token::Token::VAR,
                "CONST" => token::Token::CONST,
                "PROCEDURE" => token::Token::PROCEDURE,
                _ => token::Token::IDENT(word),
            };
            return Some(x)
        } else {
            None
        }
    }

    fn lex_ident(&mut self) -> Option<token::Token> {
        unimplemented!()
    }

    fn bump(&mut self) {
        self.last_symbol = self.buf[self.pos..].chars().next();
        self.pos += 1;
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = token::Token<'a>;
    fn next(& mut self) -> Option<token::Token> {
        if let Some((_, o)) = (regex!(r"^\s*")).find(&self.buf[self.pos..]) {
            self.pos += o;
        }

        let current_char = self.buf[self.pos..].chars().next();
        if let Some(c) = current_char {
            match c {
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
                x if x.is_alphabetic() => self.lex_word(),
                x if x.is_numeric() => None,

                _ => None,
            }
        } else {
            None
        }
    }
}
