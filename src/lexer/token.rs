use std::fmt;

pub enum Token<'tok> {
    VAR,
    CONST,
    PROCEDURE,
    Ident(&'tok str),
    Val(&'tok str),
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

impl<'tok> fmt::Display for Token<'tok> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Token::*;
        let s = match *self {
            VAR => "VAR",
            CONST =>"CONST",
            PROCEDURE => "PROCEDURE",
            Ident(s) => s,
            Val(i) => i,
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
