use crate::names::*;
use std::rc::*;

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
pub enum Term {
    Int(i32),
    Bool(bool),
    Var(Rc<Name>),
    Op1(Op1, Rc<Term>),
    Op2(Op2, Rc<Term>, Rc<Term>),
    Fun(Rc<Name>, Rc<Name>, Rc<Term>),
    App(Rc<Term>, Rc<Term>),
    LetIn(Rc<Name>, Rc<Term>, Rc<Term>),
    Ifte(Rc<Term>, Rc<Term>, Rc<Term>),
}

pub fn int(i: i32) -> Rc<Term> {
    Rc::new(Term::Int(i))
}

pub fn bool(b: bool) -> Rc<Term> {
    Rc::new(Term::Bool(b))
}

pub fn var(x: Rc<Name>) -> Rc<Term> {
    Rc::new(Term::Var(x))
}

pub fn op1(op: Op1, m: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::Op1(op, m))
}

pub fn op2(op: Op2, m: Rc<Term>, n: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::Op2(op, m, n))
}

pub fn fun(x: Rc<Name>, y: Rc<Name>, m: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::Fun(x, y, m))
}

pub fn app<'a>(m: Rc<Term>, n: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::App(m, n))
}

pub fn letin<'a>(x: Rc<Name>, m: Rc<Term>, n: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::LetIn(x, m, n))
}

pub fn ifte<'a>(m: Rc<Term>, n1: Rc<Term>, n2: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::Ifte(m, n1, n2))
}
