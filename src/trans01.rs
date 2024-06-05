use crate::{ast0, ast1, names::Name};
use im_rc::HashMap;
use std::rc::*;

pub type Ctx = HashMap<String, Rc<Name>>;

pub fn trans(ctx: Ctx, m: Rc<ast0::Term>) -> Rc<ast1::Term> {
    use ast0::Term::*;
    match &*m {
        Int(i) => ast1::int(*i),
        Bool(b) => ast1::bool(*b),
        Var(s) => {
            let x = ctx.get(s).unwrap();
            ast1::var(x.clone())
        }
        Op1(op1, m) => {
            let op1 = trans_op1(op1);
            let m = trans(ctx, m.clone());
            ast1::op1(op1, m)
        }
        Op2(op2, m, n) => {
            let op2 = trans_op2(op2);
            let m = trans(ctx.clone(), m.clone());
            let n = trans(ctx.clone(), n.clone());
            ast1::op2(op2, m, n)
        }
        Fun(f0, x0, m) => {
            let mut ctx = ctx.clone();
            let f = Name::create(f0.clone());
            let x = Name::create(x0.clone());
            ctx.insert(f0.clone(), f.clone());
            ctx.insert(x0.clone(), x.clone());
            let m = trans(ctx, m.clone());
            ast1::fun(f, x, m)
        }
        App(m, n) => {
            let m = trans(ctx.clone(), m.clone());
            let n = trans(ctx.clone(), n.clone());
            ast1::app(m, n)
        }
        LetIn(x0, m, n) => {
            let m = trans(ctx.clone(), m.clone());
            let mut ctx = ctx.clone();
            let x = Name::create(x0.clone());
            ctx.insert(x0.clone(), x.clone());
            let n = trans(ctx, n.clone());
            ast1::letin(x, m, n)
        }
        Ifte(m, n1, n2) => {
            let m = trans(ctx.clone(), m.clone());
            let n1 = trans(ctx.clone(), n1.clone());
            let n2 = trans(ctx.clone(), n2.clone());
            ast1::ifte(m, n1, n2)
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
