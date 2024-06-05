use std::rc::Rc;

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
    Var(String),
    Op1(Op1, Rc<Term>),
    Op2(Op2, Rc<Term>, Rc<Term>),
    Fun(String, String, Rc<Term>),
    App(Rc<Term>, Rc<Term>),
    LetIn(String, Rc<Term>, Rc<Term>),
    Ifte(Rc<Term>, Rc<Term>, Rc<Term>),
}

pub fn int(i: i32) -> Rc<Term> {
    Rc::new(Term::Int(i))
}

pub fn bool(b: bool) -> Rc<Term> {
    Rc::new(Term::Bool(b))
}

pub fn var(s: String) -> Rc<Term> {
    Rc::new(Term::Var(s))
}

pub fn op1(op: Op1, m: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::Op1(op, m))
}

pub fn op2<'a>(op: Op2, m: Rc<Term>, n: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::Op2(op, m, n))
}

pub fn fun(s1: String, s2: String, m: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::Fun(s1, s2, m))
}

pub fn app(m: Rc<Term>, n: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::App(m, n))
}

pub fn letin(s: String, m: Rc<Term>, n: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::LetIn(s, m, n))
}

pub fn ifte<'a>(m: Rc<Term>, n1: Rc<Term>, n2: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::Ifte(m, n1, n2))
}
