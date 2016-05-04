pub mod token;

use self::token::Token;

pub struct Lex {
    buf: String,
    pos: usize,
    last_symbol: Option<char>,
}

impl Lex {
    pub fn new(src: &'static str) -> Lex {
        Lex {
            buf: src.to_string(),
            pos: 0,
            last_symbol: None,
        }
    }

    fn current_position(&self) {
        println!("Current position: {:?}", self.pos);
        println!("Current char: {:?}", &self.buf[self.pos..].chars().next());
        println!("Last symbol: {:?}", self.last_symbol);
    }


    fn bump(&mut self) {
        self.last_symbol = self.buf[self.pos..].chars().next();
        self.pos += 1;
    }

    fn lex_assign(&mut self) -> Option<Token> {
        let current_char = self.buf[self.pos + 1..].chars().next();
        if let Some(c) = current_char {
            match c {
                '=' => {
                    self.bump();
                    self.bump();
                    Some(Token::Assign)
                },
                _ => {
                    self.bump();
                    None
                }
            }
        } else {
            None
        }
    }
    fn lex_eq(&mut self) -> Option<Token> {
        let current_char = self.buf[self.pos + 1 ..].chars().next();
        if let Some(c) = current_char {
            match c {
                '=' => {
                    self.bump();
                    self.bump();
                    Some(Token::EqEq)
                },
                _ => {
                    self.bump();
                    Some(Token::Eq)
                }
            }
        } else {
            None
        }
    }

    fn lex_lt(&mut self) -> Option<Token> {
        let current_char = self.buf[self.pos + 1 ..].chars().next();
        if let Some(c) = current_char {
            match c {
                '=' => {
                    self.bump();
                    self.bump();
                    Some(Token::Le)
                },
                _ => {
                    self.bump();
                    Some(Token::Lt)
                }
            }
        } else {
            None
        }
    }

    fn lex_gt(&mut self) -> Option<Token> {
        let current_char = self.buf[self.pos + 1 ..].chars().next();
        if let Some(c) = current_char {
            match c {
                '=' => {
                    self.bump();
                    self.bump();
                    Some(Token::Ge)
                },
                _ => {
                    self.bump();
                    Some(Token::Gt)
                }
            }
        } else {
            None
        }
    }
    fn lex_num(&mut self) -> Option<Token> {
        if let Some((_, span)) = (regex!(r"[^\s]+")).find(&self.buf[self.pos..]) {
            let word = self.buf[self.pos .. self.pos + span].chars().as_str();
            self.pos += word.len();
            Some(Token::Val(word.to_string()))
        } else {
            self.bump();
            None
        }

    }
    fn lex_word(&mut self) -> Option<Token> {
        if let Some((_, span)) = (regex!(r"[^\s]+")).find(&self.buf[self.pos..]) {
            let word = self.buf[self.pos .. self.pos + span].chars().as_str();
            let x = match word {
                "VAR" => {
                    self.pos += word.len();
                    Token::VAR
                },
                "CONST" => {
                    self.pos += word.len();
                    Token::CONST
                },
                "PROCEDURE" => {
                    self.pos += word.len();
                    Token::PROCEDURE
                },
                "CALL" => {
                    self.pos += word.len();
                    Token::Call
                },
                "DO" => {
                    self.pos += word.len();
                    Token::Do
                },
                "WHILE" => {
                    self.pos += word.len();
                    Token::While
                },
                "IF" => {
                    self.pos += word.len();
                    Token::If
                },
                "THEN" => {
                    self.pos += word.len();
                    Token::Then
                },
                "BEGIN" => {
                    self.pos += word.len();
                    Token::Begin
                },
                "END" => {
                    self.pos += word.len();
                    Token::End
                },
                "ODD" => {
                    self.pos += word.len();
                    Token::Odd
                },
                _ => {
                    if regex!(r"\b(,|;)+").is_match(word) {
                        self.pos += word.len() - 1;
                        Token::Ident(word[.. word.len() - 1].to_string())
                    } else {
                        self.pos += word.len();
                        Token::Ident(word[..].to_string())
                    }
                }
            };
            //self.current_position();
            Some(x)
        } else {
            self.bump();
            None
        }
    }
}

impl Iterator for Lex {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        if let Some((s, _)) = (regex!(r"[^\s]+")).find(&self.buf[self.pos..]) {
            self.pos += s;
        }

        let current_char = self.buf[self.pos..].chars().next();
        if let Some(c) = current_char {
            match c {
                '+' => {
                    self.bump();
                    Some(Token::Add)
                }
                '-' => {
                    self.bump();
                    Some(Token::Sub)
                }
                '*' => {
                    self.bump();
                    Some(Token::Mul)
                }
                '/' => {
                    self.bump();
                    Some(Token::Div)
                }
                '(' => {
                    self.bump();
                    Some(Token::LParen)
                }
                ')' => {
                    self.bump();
                    Some(Token::RParen)
                }
                ':' => {
                    self.lex_assign()
                }
                '<' => {
                    self.lex_lt()
                }
                '>' => {
                    self.lex_gt()
                }
                '=' => {
                    self.lex_eq()
                }
                ',' => {
                    self.bump();
                    Some(Token::Colon)
                }
                ';' => {
                    self.bump();
                    Some(Token::SemiColon)
                }
                ' ' => {
                    self.bump();
                    None
                }
                '\n' => {
                    self.bump();
                    None
                }
                x if x.is_alphabetic() => self.lex_word(),
                x if x.is_numeric() => self.lex_num(),
                _ => None,
            }
        } else {
            println!("None");
            None
        }
    }
}
