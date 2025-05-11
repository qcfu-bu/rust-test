use crate::names::*;
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

#[derive(Debug)]
pub enum Term {
    Int(i32),
    Bool(bool),
    Var(Name),
    Op1(Op1, Rc<Term>),
    Op2(Op2, Rc<Term>, Rc<Term>),
    Fun(Name, Name, Rc<Term>),
    App(Rc<Term>, Rc<Term>),
    LetIn(Name, Rc<Term>, Rc<Term>),
    Ifte(Rc<Term>, Rc<Term>, Rc<Term>),
}

pub fn int(i: i32) -> Rc<Term> {
    Rc::new(Term::Int(i))
}

pub fn bool(b: bool) -> Rc<Term> {
    Rc::new(Term::Bool(b))
}

pub fn var(x: Name) -> Rc<Term> {
    Rc::new(Term::Var(x))
}

pub fn op1(op: Op1, m: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::Op1(op, m))
}

pub fn op2(op: Op2, m: Rc<Term>, n: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::Op2(op, m, n))
}

pub fn fun(x: Name, y: Name, m: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::Fun(x, y, m))
}

pub fn app(m: Rc<Term>, n: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::App(m, n))
}

pub fn letin(x: Name, m: Rc<Term>, n: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::LetIn(x, m, n))
}

pub fn ifte(m: Rc<Term>, n1: Rc<Term>, n2: Rc<Term>) -> Rc<Term> {
    Rc::new(Term::Ifte(m, n1, n2))
}

pub fn subst(m0: Rc<Term>, x0: Name, n: Rc<Term>) -> Rc<Term> {
    use Term::*;
    match &*m0 {
        Int(_) => m0.clone(),
        Bool(_) => m0.clone(),
        Var(x) => {
            if x0 == *x {
                n
            } else {
                m0.clone()
            }
        }
        Op1(op, m1) => op1(*op, subst(m1.clone(), x0, n)),
        Op2(op, m1, m2) => {
            let m1 = subst(m1.clone(), x0.clone(), n.clone());
            let m2 = subst(m2.clone(), x0, n);
            op2(*op, m1, m2)
        }
        Fun(f, x, m1) => {
            if x0 != *f && x0 != *x {
                fun(f.clone(), x.clone(), subst(m1.clone(), x0, n))
            } else {
                m0.clone()
            }
        }
        App(m1, m2) => {
            let m1 = subst(m1.clone(), x0.clone(), n.clone());
            let m2 = subst(m2.clone(), x0, n);
            app(m1, m2)
        }
        LetIn(x, m1, m2) => {
            let m1 = subst(m1.clone(), x0.clone(), n.clone());
            let m2 = subst(m2.clone(), x0, n);
            letin(x.clone(), m1, m2)
        }
        Ifte(m1, m2, m3) => {
            let m1 = subst(m1.clone(), x0.clone(), n.clone());
            let m2 = subst(m2.clone(), x0.clone(), n.clone());
            let m3 = subst(m3.clone(), x0.clone(), n.clone());
            ifte(m1, m2, m3)
        }
    }
}

pub fn reduce(m0: Rc<Term>) -> Rc<Term> {
    use Term::*;
    match &*m0 {
        Int(i) => m0.clone(),
        Bool(b) => m0.clone(),
        Var(x) => m0.clone(),
        Op1(op1, m) => {
            let m = reduce(m0.clone());
            reduce_op1(&op1, m)
        }
        Op2(op2, m, n) => {
            let m = reduce(m.clone());
            let n = reduce(n.clone());
            reduce_op2(&op2, m, n)
        }
        Fun(f, x, m) => m0.clone(),
        App(m, n) => {
            let m1 = reduce(m.clone());
            let n = reduce(n.clone());
            match &*m1 {
                Fun(f, x, m) => {
                    let m = subst(m.clone(), f.clone(), m1.clone());
                    let m = subst(m.clone(), x.clone(), n);
                    reduce(m)
                }
                _ => panic!("eval_App({:?})", m0),
            }
        }
        LetIn(x, m, n) => {
            let m = reduce(m.clone());
            let n = subst(n.clone(), x.clone(), m);
            reduce(n)
        }
        Ifte(m, n1, n2) => {
            let m = reduce(m.clone());
            match &*m {
                Bool(true) => reduce(n1.clone()),
                Bool(false) => reduce(n2.clone()),
                _ => panic!("eval_Ifte({:?})", m0),
            }
        }
    }
}

fn reduce_op1(op: &Op1, m: Rc<Term>) -> Rc<Term> {
    use self::Op1::*;
    use Term::*;
    match (op, &*m) {
        (Not, Bool(b)) => bool(!b),
        (Neg, Int(i)) => int(-i),
        (_, _) => panic!("eval_op1({:?}, {:?})", op, m),
    }
}

fn reduce_op2(op: &Op2, m: Rc<Term>, n: Rc<Term>) -> Rc<Term> {
    use self::Op2::*;
    use Term::*;
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
        (_, _, _) => panic!("eval_op2({:?}, {:?}, {:?})", op, m, n),
    }
}
