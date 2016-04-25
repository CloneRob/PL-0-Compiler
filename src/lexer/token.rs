pub enum Ident{
    VAR(String),
    CONST(String),
    PROCEDURE(String),
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
