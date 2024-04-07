use crate::names::*;
use std::rc::Rc;

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
pub enum TermNode {
    Int(i32),
    Bool(bool),
    Var(Name),
    Op1(Op1, Term),
    Op2(Op2, Term, Term),
    Fun(Name, Name, Term),
    App(Term, Term),
    LetIn(Name, Term, Term),
    Ifte(Term, Term, Term),
}

pub type Term = Rc<TermNode>;
