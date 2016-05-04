#![allow(dead_code)]
use lexer::token::Token;


pub enum Block {
    ConstAssign(Vec<(Token, Token)>),
    VarDeclare(Vec<(Token, Token)>),
    Procedure(Box<(Token, Block, Statement)>),
}

pub enum Statement {
    Assign(Token, Box<Expr>),
    Not(Box<Expr>),
    Begin(Vec<Statement>),
    IfThen(Box<(Condition, Statement)>),
    DoWhile(Box<(Condition, Statement)>),
}

pub enum Condition {
    Odd(Box<Expr>),
    Eq(Box<(Expr, Expr)>),
    Num(Box<(Expr, Expr)>),
    Le(Box<(Expr, Expr)>),
    Lt(Box<(Expr, Expr)>),
    Ge(Box<(Expr, Expr)>),
    Gt(Box<(Expr, Expr)>),
}

pub enum Expr {
    OneTerm(Box<(Option<Token>, Term)>),
}

pub enum Term {
    Fact(Box<Factor>),
    Mul(Box<(Factor, Factor)>),
    Div(Box<(Factor, Factor)>),

}

pub enum Factor {
    Ident(Token),
    Numb(Token),
    ParenExpr(Box<Expr>),
}
