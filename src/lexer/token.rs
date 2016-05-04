use std::fmt;

pub enum Token {
    VAR,
    CONST,
    PROCEDURE,
    Ident(String),
    Val(String),
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

    Odd,

    Begin,
    End,

    Call,
    Assign,

    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    EqEq,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Token::*;
        let s = match *self {
            VAR => "VAR",
            CONST =>"CONST",
            PROCEDURE => "PROCEDURE",
            Ident(ref s) => s,
            Val(ref i) => i,
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",

            LParen => "(",
            RParen => ")",

            Colon => ",",
            SemiColon => ";",

            If => "If",
            Then => "Then",
            While => "While",
            Do => "Do",

            Begin => "Begin",
            End => "End",

            Odd => "Odd",

            Call => "Call",

            Assign => ":=",
            Gt => ">",
            Ge => ">=",
            Lt => "<",
            Le => "<=",
            Eq => "=",
            EqEq => "==",
        };
        s.fmt(f)
    }
}
