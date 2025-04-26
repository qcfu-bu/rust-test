use crate::{ast0, ast2};
use ahash::HashMap;
use std::rc::*;

pub type Ctx<'a> = Rc<HashMap<String, ast2::Term<'a>>>;

pub fn trans<'a>(ctx: Ctx<'a>, m: &'a ast0::Term) -> ast2::Term<'a> {
    use ast0::Term::*;
    match m {
        Int(i) => ast2::int(*i),
        Bool(b) => ast2::bool(*b),
        Var(s) => ctx.get(s).unwrap().clone(),
        Op1(op1, m) => {
            let op1 = trans_op1(op1);
            let m = trans(ctx, &*m);
            ast2::op1(op1, m)
        }
        Op2(op2, m, n) => {
            let op2 = trans_op2(op2);
            let m = trans(ctx.clone(), &*m);
            let n = trans(ctx.clone(), &*n);
            ast2::op2(op2, m, n)
        }
        Fun(f0, x0, m0) => {
            let bnd = Rc::new(move |fv, xv| {
                let mut ctx = (*ctx).clone();
                ctx.insert(f0.clone(), fv);
                ctx.insert(x0.clone(), xv);
                trans(Rc::new(ctx), &*m0)
            });
            ast2::fun(bnd)
        }
        App(m, n) => {
            let m = trans(ctx.clone(), &*m);
            let n = trans(ctx.clone(), &*n);
            ast2::app(m, n)
        }
        LetIn(x0, m, n) => {
            let m = trans(ctx.clone(), &*m);
            let bnd = Rc::new(move |xv| {
                let mut ctx = (*ctx).clone();
                ctx.insert(x0.clone(), xv);
                trans(Rc::new(ctx), &*n)
            });
            ast2::letin(m, bnd)
        }
        Ifte(m, n1, n2) => {
            let m = trans(ctx.clone(), &*m);
            let n1 = trans(ctx.clone(), &*n1);
            let n2 = trans(ctx.clone(), &*n2);
            ast2::ifte(m, n1, n2)
        }
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
