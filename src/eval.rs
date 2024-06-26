use crate::env::*;
use crate::{ast1::*, names::Name};
use bumpalo::Bump;

type Env<'a> = List<'a, (&'a Name, &'a Value<'a>)>;

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Int(i32),
    Bool(bool),
    Clo(&'a Env<'a>, &'a Name, &'a Name, &'a Term<'a>),
}

fn int<'a>(i: i32, bump: &'a Bump) -> &'a Value<'a> {
    bump.alloc(Value::Int(i))
}

fn bool<'a>(b: bool, bump: &'a Bump) -> &'a Value<'a> {
    bump.alloc(Value::Bool(b))
}

fn clo<'a>(
    env: &'a Env<'a>,
    x: &'a Name,
    y: &'a Name,
    m: &'a Term<'a>,
    bump: &'a Bump,
) -> &'a Value<'a> {
    bump.alloc(Value::Clo(env, x, y, m))
}

pub fn eval<'a>(env: &'a Env<'a>, m0: &'a Term, bump: &'a Bump) -> &'a Value<'a> {
    use Term::*;
    match m0 {
        Int(i) => int(*i, bump),
        Bool(b) => bool(*b, bump),
        Var(x) => match find(*x, env) {
            Some(v) => v,
            None => {
                println!("cannot find({:?})", x);
                panic!()
            }
        },
        Op1(op1, m) => {
            let m = eval(env, m, bump);
            eval_op1(&op1, m, bump)
        }
        Op2(op2, m, n) => {
            let m = eval(env, m, bump);
            let n = eval(env, n, bump);
            eval_op2(&op2, m, n, bump)
        }
        Fun(f, x, m) => clo(env, f, x, m, bump),
        App(m, n) => {
            let m0 = eval(env, m, bump);
            let n = eval(env, n, bump);
            match *m0 {
                Value::Clo(env, f, x, m) => {
                    let env = cons((f, m0), env, bump);
                    let env = cons((x, n), env, bump);
                    eval(env, m, bump)
                }
                _ => panic!("eval_App({:?})", m0),
            }
        }
        LetIn(x, m, n) => {
            let m = eval(env, m, bump);
            let env = cons((*x, m), env, bump);
            eval(env, n, bump)
        }
        Ifte(m, n1, n2) => {
            let m = eval(env, m, bump);
            match m {
                Value::Bool(true) => eval(env, n1, bump),
                Value::Bool(false) => eval(env, n2, bump),
                _ => panic!("eval_Ifte({:?})", m0),
            }
        }
    }
}

fn eval_op1<'a>(op: &Op1, m: &'a Value<'a>, bump: &'a Bump) -> &'a Value<'a> {
    use self::Op1::*;
    use Value::*;
    match (op, &*m) {
        (Not, Bool(b)) => bool(!b, bump),
        (Neg, Int(i)) => int(-i, bump),
        (_, _) => panic!("eval_op1({:?}, {:?})", op, m),
    }
}

fn eval_op2<'a>(op: &Op2, m: &'a Value<'a>, n: &'a Value<'a>, bump: &'a Bump) -> &'a Value<'a> {
    use self::Op2::*;
    use Value::*;
    match (op, &*m, &*n) {
        (Add, Int(i), Int(j)) => int(i + j, bump),
        (Sub, Int(i), Int(j)) => int(i - j, bump),
        (Mul, Int(i), Int(j)) => int(i * j, bump),
        (Div, Int(i), Int(j)) => int(i / j, bump),
        (Lte, Int(i), Int(j)) => bool(i <= j, bump),
        (Gte, Int(i), Int(j)) => bool(i >= j, bump),
        (Lt, Int(i), Int(j)) => bool(i < j, bump),
        (Gt, Int(i), Int(j)) => bool(i > j, bump),
        (Eq, Int(i), Int(j)) => bool(i == j, bump),
        (Neq, Int(i), Int(j)) => bool(i != j, bump),
        (And, Bool(i), Bool(j)) => bool(*i && *j, bump),
        (Or, Bool(i), Bool(j)) => bool(*i || *j, bump),
        (_, _, _) => panic!("eval_op2({:?}, {:?}, {:?})", op, m, n),
    }
}
