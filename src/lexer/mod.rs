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
}
