use crate::{ast0, ast1, names::Name};
use bumpalo::Bump;
use im_rc::HashMap;

pub type Ctx<'a> = HashMap<String, &'a Name>;

pub fn trans<'a, 'b>(ctx: Ctx<'b>, m: &'a ast0::Term<'a>, bump: &'b Bump) -> &'b ast1::Term<'b> {
    use ast0::Term::*;
    match m {
        Int(i) => ast1::int(*i, bump),
        Bool(b) => ast1::bool(*b, bump),
        Var(s) => {
            let x = ctx.get(s).unwrap();
            ast1::var(x, bump)
        }
        Op1(op1, m) => {
            let op1 = trans_op1(op1);
            let m = trans(ctx, m, bump);
            ast1::op1(op1, m, bump)
        }
        Op2(op2, m, n) => {
            let op2 = trans_op2(op2);
            let m = trans(ctx.clone(), m, bump);
            let n = trans(ctx.clone(), n, bump);
            ast1::op2(op2, m, n, bump)
        }
        Fun(f0, x0, m) => {
            let mut ctx = ctx.clone();
            let f = Name::create(f0.clone(), bump);
            let x = Name::create(x0.clone(), bump);
            ctx.insert(f0.clone(), f);
            ctx.insert(x0.clone(), x);
            let m = trans(ctx, m, bump);
            ast1::fun(f, x, m, bump)
        }
        App(m, n) => {
            let m = trans(ctx.clone(), m, bump);
            let n = trans(ctx.clone(), n, bump);
            ast1::app(m, n, bump)
        }
        LetIn(x0, m, n) => {
            let m = trans(ctx.clone(), m, bump);
            let mut ctx = ctx.clone();
            let x = Name::create(x0.clone(), bump);
            ctx.insert(x0.clone(), x);
            let n = trans(ctx, n, bump);
            ast1::letin(x, m, n, bump)
        }
        Ifte(m, n1, n2) => {
            let m = trans(ctx.clone(), m, bump);
            let n1 = trans(ctx.clone(), n1, bump);
            let n2 = trans(ctx.clone(), n2, bump);
            ast1::ifte(m, n1, n2, bump)
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
