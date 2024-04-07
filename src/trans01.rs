use crate::{
    ast0, ast1,
    names::{Name, NameNode},
};
use im::HashMap;
use std::rc::Rc;

pub type Ctx = HashMap<String, Name>;

pub fn trans(ctx: Ctx, m: ast0::Term) -> ast1::Term {
    use ast0::TermNode::*;
    match &*m {
        Int(i) => Rc::new(ast1::TermNode::Int(*i)),
        Bool(b) => Rc::new(ast1::TermNode::Bool(*b)),
        Var(s) => {
            let x = ctx.get(s).unwrap();
            Rc::new(ast1::TermNode::Var(x.clone()))
        }
        Op1(op1, m) => {
            let op1 = trans_op1(op1);
            let m = trans(ctx, m.clone());
            Rc::new(ast1::TermNode::Op1(op1, m))
        }
        Op2(op2, m, n) => {
            let op2 = trans_op2(op2);
            let m = trans(ctx.clone(), m.clone());
            let n = trans(ctx.clone(), n.clone());
            Rc::new(ast1::TermNode::Op2(op2, m, n))
        }
        Fun(f0, x0, m) => {
            let mut ctx = ctx.clone();
            let f = Rc::new(NameNode::create(f0.clone()));
            let x = Rc::new(NameNode::create(x0.clone()));
            ctx.insert(f0.clone(), f.clone());
            ctx.insert(x0.clone(), x.clone());
            let m = trans(ctx, m.clone());
            Rc::new(ast1::TermNode::Fun(f, x, m))
        }
        App(m, n) => {
            let m = trans(ctx.clone(), m.clone());
            let n = trans(ctx.clone(), n.clone());
            Rc::new(ast1::TermNode::App(m, n))
        }
        LetIn(x0, m, n) => {
            let m = trans(ctx.clone(), m.clone());
            let mut ctx = ctx.clone();
            let x = Rc::new(NameNode::create(x0.clone()));
            ctx.insert(x0.clone(), x.clone());
            let n = trans(ctx, n.clone());
            Rc::new(ast1::TermNode::LetIn(x, m, n))
        }
        Ifte(m, n1, n2) => {
            let m = trans(ctx.clone(), m.clone());
            let n1 = trans(ctx.clone(), n1.clone());
            let n2 = trans(ctx.clone(), n2.clone());
            Rc::new(ast1::TermNode::Ifte(m, n1, n2))
        }
    }
}

fn trans_op1(op1: &ast0::Op1) -> ast1::Op1 {
    use ast0::Op1::*;
    match op1 {
        Neg => ast1::Op1::Neg,
        Not => ast1::Op1::Not,
    }
}

fn trans_op2(op2: &ast0::Op2) -> ast1::Op2 {
    use ast0::Op2::*;
    match op2 {
        Add => ast1::Op2::Add,
        Sub => ast1::Op2::Sub,
        Mul => ast1::Op2::Mul,
        Div => ast1::Op2::Div,
        Lte => ast1::Op2::Lte,
        Gte => ast1::Op2::Gte,
        Lt => ast1::Op2::Lt,
        Gt => ast1::Op2::Gt,
        Eq => ast1::Op2::Eq,
        Neq => ast1::Op2::Neq,
        And => ast1::Op2::And,
        Or => ast1::Op2::Or,
    }
}
