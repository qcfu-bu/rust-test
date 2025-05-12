use std::rc::Rc;

use crate::names::Name;

#[derive(Debug)]
pub enum Op1 {
    Neg,
    Not,
}

#[derive(Debug)]
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
    Var(u64),
    Op1(Op1, Rc<Term>),
    Op2(Op2, Rc<Term>, Rc<Term>),
    Fun(u64, u64, Rc<Term>),
    App(Rc<Term>, Rc<Term>),
    LetIn(u64, Rc<Term>, Rc<Term>),
    Ifte(Rc<Term>, Rc<Term>, Rc<Term>),
}

pub fn int(i: i32) -> Rc<Term> {
    Rc::new(Term::Int(i))
}

pub fn bool(b: bool) -> Rc<Term> {
    Rc::new(Term::Bool(b))
}

pub fn var(s: String) -> Rc<Term> {
    let x = Name::create(s);
    Rc::new(Term::Var(x.id))
}

pub fn op1(op: Op1, m: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::Op1(op, m))
}

pub fn op2<'a>(op: Op2, m: Rc<Term>, n: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::Op2(op, m, n))
}

pub fn fun(s1: String, s2: String, m: Rc<Term>) -> Rc<Term> {
    let f = Name::create(s1);
    let x = Name::create(s2);
    Rc::new(Term::Fun(f.id, x.id, m))
}

pub fn app(m: Rc<Term>, n: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::App(m, n))
}

pub fn letin(s: String, m: Rc<Term>, n: Rc<Term>) -> Rc<Term> {
    let x = Name::create(s);
    Rc::new(Term::LetIn(x.id, m, n))
}

pub fn ifte<'a>(m: Rc<Term>, n1: Rc<Term>, n2: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::Ifte(m, n1, n2))
}
