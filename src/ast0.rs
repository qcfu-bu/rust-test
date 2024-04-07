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

pub type Term = Rc<TermNode>;

#[derive(Debug)]
pub enum TermNode {
    Int(i32),
    Bool(bool),
    Var(String),
    Op1(Op1, Term),
    Op2(Op2, Term, Term),
    Fun(String, String, Term),
    App(Term, Term),
    LetIn(String, Term, Term),
    Ifte(Term, Term, Term),
}
