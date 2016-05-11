use std::fmt;

#[derive(Debug, Clone)]
pub enum Delimiter {
    LParen,
    RParen,
}

impl fmt::Display for Delimiter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Delimiter::*;
        let x = match *self {
            LParen => "(",
            RParen => ")",
        };
        x.fmt(f)
    }
}

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,

    Tilde,
    Equals,
    EqEq,
    NotEquals,


    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,

    Assign,

    Not,
    Qmark,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Operator::*;
        let x = match *self {
            Plus => "+",
            Minus => "-",
            Star => "*",
            Slash => "/",

            Tilde => "#",
            Equals => "=",
            EqEq => "==",
            NotEquals => "!=",
            Greater => ">",
            GreaterOrEqual => ">=",
            Less => "<",
            LessOrEqual => "<=",

            Assign => ":=",

            Not => "!",
            Qmark => "?",
        };
        x.fmt(f)
    }
}


#[derive(Debug,Clone)]
pub enum Keyword {
    Var,
    Const,
    Procedure,

    Call,
    Odd,

    If,
    Then,
    Begin,
    End,
    While,
    Do,

}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Keyword::*;
        let x = match *self {
            Var => "var",
            Const => "const",
            Procedure => "procedure",

            Call => "call",
            Odd => "odd",

            If => "if",
            Then => "then",

            Begin => "Begin",
            End => "End",

            While => "while",
            Do => "so",
        };
        x.fmt(f)
    }
}

pub enum Token {
    Ident(String),
    Value(String),
    Key(Keyword),
    Op(Operator),
    Delim(Delimiter),
    Comma,
    SemiColon,
    Dot,
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Token::*;

        match *self {
            Ident(ref i) => i.fmt(f),
            Value(ref v) => v.fmt(f),
            Key(ref k) => k.fmt(f),
            Op(ref o) => o.fmt(f),
            Delim(ref d) => d.fmt(f),
            Comma => ",".fmt(f),
            SemiColon => ";".fmt(f),
            Dot => ".".fmt(f),
        }
    }
}
