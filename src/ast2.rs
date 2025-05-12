use crate::names::*;
use derivative::Derivative;
use std::rc::*;

#[derive(Debug, Clone, Copy)]
pub enum Op1 {
    Neg,
    Not,
}

#[derive(Debug, Clone, Copy)]
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

pub type Binder1<'a> = Rc<dyn Fn(Term<'a>) -> Term<'a> + 'a>;
pub type Binder2<'a> = Rc<dyn Fn(Term<'a>, Term<'a>) -> Term<'a> + 'a>;
pub type Term<'a> = Rc<TermNode<'a>>;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum TermNode<'a> {
    Int(i32),
    Bool(bool),
    Var(Name),
    Op1(Op1, Term<'a>),
    Op2(Op2, Term<'a>, Term<'a>),
    Fun(#[derivative(Debug = "ignore")] Binder2<'a>),
    App(Term<'a>, Term<'a>),
    LetIn(Term<'a>, #[derivative(Debug = "ignore")] Binder1<'a>),
    Ifte(Term<'a>, Term<'a>, Term<'a>),
}

pub fn int<'a>(i: i32) -> Term<'a> {
    Rc::new(TermNode::Int(i))
}

pub fn bool<'a>(b: bool) -> Term<'a> {
    Rc::new(TermNode::Bool(b))
}

pub fn var<'a>(x: Name) -> Term<'a> {
    Rc::new(TermNode::Var(x))
}

pub fn op1<'a>(op: Op1, m: Term<'a>) -> Term<'a> {
    Rc::new(TermNode::Op1(op, m))
}

pub fn op2<'a>(op: Op2, m: Term<'a>, n: Term<'a>) -> Term<'a> {
    Rc::new(TermNode::Op2(op, m, n))
}

pub fn fun<'a>(bnd: Binder2<'a>) -> Term<'a> {
    Rc::new(TermNode::Fun(bnd))
}

pub fn app<'a>(m: Term<'a>, n: Term<'a>) -> Term<'a> {
    Rc::new(TermNode::App(m, n))
}

pub fn letin<'a>(m: Term<'a>, bnd: Binder1<'a>) -> Term<'a> {
    Rc::new(TermNode::LetIn(m, bnd))
}

pub fn ifte<'a>(m: Term<'a>, n1: Term<'a>, n2: Term<'a>) -> Term<'a> {
    Rc::new(TermNode::Ifte(m, n1, n2))
}

pub fn reduce<'a>(m0: Term<'a>) -> Term<'a> {
    use TermNode::*;
    match &*m0 {
        Int(_) => m0.clone(),
        Bool(_) => m0.clone(),
        Var(_) => m0.clone(),
        Op1(op1, m) => {
            let m = reduce(m0.clone());
            reduce_op1(&op1, m)
        }
        Op2(op2, m, n) => {
            let m = reduce(m.clone());
            let n = reduce(n.clone());
            reduce_op2(&op2, m, n)
        }
        Fun(_) => m0.clone(),
        App(m, n) => {
            let m1 = reduce(m.clone());
            let n = reduce(n.clone());
            match &*m1 {
                Fun(bnd) => reduce(bnd(m1.clone(), n)),
                _ => panic!("eval_App"),
            }
        }
        LetIn(m, bnd) => {
            let m = reduce(m.clone());
            reduce(bnd(m))
        }
        Ifte(m, n1, n2) => {
            let m = reduce(m.clone());
            match &*m {
                Bool(true) => reduce(n1.clone()),
                Bool(false) => reduce(n2.clone()),
                _ => panic!("eval_Ifte"),
            }
        }
    }
}

fn reduce_op1<'a>(op: &Op1, m: Term<'a>) -> Term<'a> {
    use self::Op1::*;
    use TermNode::*;
    match (op, &*m) {
        (Not, Bool(b)) => bool(!b),
        (Neg, Int(i)) => int(-i),
        (_, _) => panic!("eval_op1"),
    }
}

fn reduce_op2<'a>(op: &Op2, m: Term<'a>, n: Term<'a>) -> Term<'a> {
    use self::Op2::*;
    use TermNode::*;
    match (op, &*m, &*n) {
        (Add, Int(i), Int(j)) => int(i + j),
        (Sub, Int(i), Int(j)) => int(i - j),
        (Mul, Int(i), Int(j)) => int(i * j),
        (Div, Int(i), Int(j)) => int(i / j),
        (Lte, Int(i), Int(j)) => bool(i <= j),
        (Gte, Int(i), Int(j)) => bool(i >= j),
        (Lt, Int(i), Int(j)) => bool(i < j),
        (Gt, Int(i), Int(j)) => bool(i > j),
        (Eq, Int(i), Int(j)) => bool(i == j),
        (Neq, Int(i), Int(j)) => bool(i != j),
        (And, Bool(i), Bool(j)) => bool(*i && *j),
        (Or, Bool(i), Bool(j)) => bool(*i || *j),
        (_, _, _) => panic!("eval_op2"),
    }
}
