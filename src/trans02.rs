use crate::{
    ast0,
    ast2::{self, Binder1, Binder2},
};
use ahash::HashMap;
use std::rc::Rc;

pub type Ctx = Rc<HashMap<u64, ast2::Term>>;

pub fn trans(ctx: Ctx, m: Rc<ast0::Term>) -> ast2::Term {
    use ast0::Term::*;
    match &*m {
        Int(i) => ast2::int(*i),
        Bool(b) => ast2::bool(*b),
        Var(s) => ctx.get(s).unwrap().clone(),
        Op1(op1, m) => {
            let op1 = trans_op1(op1);
            let m = trans(ctx, m.clone());
            ast2::op1(op1, m)
        }
        Op2(op2, m, n) => {
            let op2 = trans_op2(op2);
            let m = trans(ctx.clone(), m.clone());
            let n = trans(ctx.clone(), n.clone());
            ast2::op2(op2, m, n)
        }
        Fun(f, x, m0) => {
            let f = f.clone();
            let x = x.clone();
            let m0 = m0.clone();
            let bnd: Binder2 = Rc::new(move |fv, xv| {
                let mut ctx = (*ctx).clone();
                ctx.insert(f, fv);
                ctx.insert(x, xv);
                trans(Rc::new(ctx), m0.clone())
            });
            ast2::fun(bnd)
        }
        App(m, n) => {
            let m = trans(ctx.clone(), m.clone());
            let n = trans(ctx.clone(), n.clone());
            ast2::app(m, n)
        }
        LetIn(x, m, n) => {
            let m = trans(ctx.clone(), m.clone());
            let x = x.clone();
            let n = n.clone();
            let bnd: Binder1 = Rc::new(move |xv| {
                let mut ctx = (*ctx).clone();
                ctx.insert(x, xv);
                trans(Rc::new(ctx), n.clone())
            });
            ast2::letin(m, bnd)
        }
        Ifte(m, n1, n2) => {
            let m = trans(ctx.clone(), m.clone());
            let n1 = trans(ctx.clone(), n1.clone());
            let n2 = trans(ctx.clone(), n2.clone());
            ast2::ifte(m, n1, n2)
        }
        _ => todo!(),
    }
}

fn trans_op1(op1: &ast0::Op1) -> ast2::Op1 {
    use ast0::Op1::*;
    match op1 {
        Neg => ast2::Op1::Neg,
        Not => ast2::Op1::Not,
    }
}

fn trans_op2(op2: &ast0::Op2) -> ast2::Op2 {
    use ast0::Op2::*;
    match op2 {
        Add => ast2::Op2::Add,
        Sub => ast2::Op2::Sub,
        Mul => ast2::Op2::Mul,
        Div => ast2::Op2::Div,
        Lte => ast2::Op2::Lte,
        Gte => ast2::Op2::Gte,
        Lt => ast2::Op2::Lt,
        Gt => ast2::Op2::Gt,
        Eq => ast2::Op2::Eq,
        Neq => ast2::Op2::Neq,
        And => ast2::Op2::And,
        Or => ast2::Op2::Or,
    }
}
