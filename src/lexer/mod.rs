mod token;

pub struct Lexer {
    buf: Vec<char>,
    pos: usize,
    last_symbol: Option<char>,
}

impl Lexer {
    pub fn new(src: &'static str) -> Lexer {
        Lexer {
            buf: src.chars().collect(),
            pos: 0,
            last_symbol: None,
        }
    }

    fn is_alphabetical(&mut self) -> token::Token{
        token::Token::Add
    }

    fn is_numerical(&mut self) -> token::Token {
        token::Token::Sub
    }
}

impl Iterator for Lexer {
    type Item = token::Token;
    fn next(&mut self) -> Option<token::Token> {
        match self.buf[self.pos] {
            '+' => Some(token::Token::Add),
            '-' => Some(token::Token::Sub),
            '*' => Some(token::Token::Mul),
            '/' => Some(token::Token::Div),

            '(' => Some(token::Token::LParen),
            ')' => Some(token::Token::RParen),

            '<' => Some(token::Token::Ls),
            '>' => Some(token::Token::Gr),
            _ => None
        }
    }
}
