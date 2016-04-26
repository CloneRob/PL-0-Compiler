use std::fmt;

pub enum Ident{
    VAR,
    CONST,
    PROCEDURE,
    IDENT(String),
}

impl Ident {
    pub fn can_continue(c: &char) -> bool {
        if c.is_numeric() && c.is_alphabetic() {
            true
        } else {
            false
        }
    }
}
pub enum Token {
    Identifier(Ident),

    Add,
    Sub,
    Mul,
    Div,

    LParen,
    RParen,

    Colon,
    SemiColon,

    If,
    Then,
    While,
    Do,

    Begin,
    End,

    Call,

    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    EqEq,
    Ne,
    And,
    AndAnd,
    Or,
    OrOr,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Token::*;
        let s = match *self {
            Identifier(..) => "Identifier(..)",
            Add => "Add",
            Sub => "Sub",
            Mul => "Mul",
            Div => "Div",

            LParen => "LParen",
            RParen => "RParen",

            Colon => "Colon",
            SemiColon => "SemiColon",

            If => "If",
            Then => "Then",
            While => "While",
            Do => "Do",

            Begin => "Begin",
            End => "End",

            Call => "Call",

            Gt => "Gt",
            Ge => "Ge",
            Lt => "Lt",
            Le => "Le",
            Eq => "Eq",
            EqEq => "EqEq",
            Ne => "Ne",
            And => "And",
            AndAnd => "AndAnd",
            Or => "Or",
            OrOr => "OrOr",

        };
        s.fmt(f)
    }
}
