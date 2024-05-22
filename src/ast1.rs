use crate::names::*;
use bumpalo::Bump;

#[derive(Debug, Clone)]
pub enum Op1 {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum Op2 {
    Add,
    Sub,
    Mul,
    Div,
    Lte,
    Gte,
    Lt,
    Gt,
    Eq,
    Neq,
    And,
    Or,
}

#[derive(Debug)]
pub enum Term<'a> {
    Int(i32),
    Bool(bool),
    Var(&'a Name),
    Op1(Op1, &'a Term<'a>),
    Op2(Op2, &'a Term<'a>, &'a Term<'a>),
    Fun(&'a Name, &'a Name, &'a Term<'a>),
    App(&'a Term<'a>, &'a Term<'a>),
    LetIn(&'a Name, &'a Term<'a>, &'a Term<'a>),
    Ifte(&'a Term<'a>, &'a Term<'a>, &'a Term<'a>),
}

pub fn int(i: i32, bump: &Bump) -> &Term {
    bump.alloc(Term::Int(i))
}

pub fn bool(b: bool, bump: &Bump) -> &Term {
    bump.alloc(Term::Bool(b))
}

pub fn var<'a>(x: &'a Name, bump: &'a Bump) -> &'a Term<'a> {
    bump.alloc(Term::Var(x))
}

pub fn op1<'a>(op: Op1, m: &'a Term<'a>, bump: &'a Bump) -> &'a Term<'a> {
    bump.alloc(Term::Op1(op, m))
}

pub fn op2<'a>(op: Op2, m: &'a Term<'a>, n: &'a Term<'a>, bump: &'a Bump) -> &'a Term<'a> {
    bump.alloc(Term::Op2(op, m, n))
}

pub fn fun<'a>(x: &'a Name, y: &'a Name, m: &'a Term<'a>, bump: &'a Bump) -> &'a Term<'a> {
    bump.alloc(Term::Fun(x, y, m))
}

pub fn app<'a>(m: &'a Term<'a>, n: &'a Term<'a>, bump: &'a Bump) -> &'a Term<'a> {
    bump.alloc(Term::App(m, n))
}

pub fn letin<'a>(x: &'a Name, m: &'a Term<'a>, n: &'a Term<'a>, bump: &'a Bump) -> &'a Term<'a> {
    bump.alloc(Term::LetIn(x, m, n))
}

pub fn ifte<'a>(
    m: &'a Term<'a>,
    n1: &'a Term<'a>,
    n2: &'a Term<'a>,
    bump: &'a Bump,
) -> &'a Term<'a> {
    bump.alloc(Term::Ifte(m, n1, n2))
}
