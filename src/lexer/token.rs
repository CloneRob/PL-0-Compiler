use std::fmt;

pub enum Token<'a> {
    VAR,
    CONST,
    PROCEDURE,
    IDENT(&'a str),

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

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Token::*;
        let s = match *self {
            VAR => "VAR",
            CONST =>"CONST",
            PROCEDURE => "PROCEDURE",
            IDENT(s) => s,
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

            Call => "Call",

            Gt => ">",
            Ge => ">=",
            Lt => "<",
            Le => "<=",
            Eq => "=",
            EqEq => "==",
            Ne => "!=",
            And => "&",
            AndAnd => "&&",
            Or => "|",
            OrOr => "||",

        };
        s.fmt(f)
    }
}
