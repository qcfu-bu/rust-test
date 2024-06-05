use crate::env::*;
use crate::{ast1::*, names::Name};
use std::rc::Rc;

type Env = Rc<List<(Rc<Name>, Rc<Value>)>>;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Clo(Env, Rc<Name>, Rc<Name>, Rc<Term>),
}

fn int(i: i32) -> Rc<Value> {
    Rc::new(Value::Int(i))
}

fn bool(b: bool) -> Rc<Value> {
    Rc::new(Value::Bool(b))
}

fn clo(env: Env, x: Rc<Name>, y: Rc<Name>, m: Rc<Term>) -> Rc<Value> {
    Rc::new(Value::Clo(env, x, y, m))
}

pub fn eval(env: Env, m0: Rc<Term>) -> Rc<Value> {
    use Term::*;
    match &*m0 {
        Int(i) => int(*i),
        Bool(b) => bool(*b),
        Var(x) => match find(x.clone(), env) {
            Some(v) => v.clone(),
            None => {
                println!("cannot find({:?})", x);
                panic!()
            }
        },
        Op1(op1, m) => {
            let m = eval(env, m.clone());
            eval_op1(&op1, m)
        }
        Op2(op2, m, n) => {
            let m = eval(env.clone(), m.clone());
            let n = eval(env, n.clone());
            eval_op2(&op2, m, n)
        }
        Fun(f, x, m) => clo(env, f.clone(), x.clone(), m.clone()),
        App(m, n) => {
            let m0 = eval(env.clone(), m.clone());
            let n = eval(env, n.clone());
            match &*m0 {
                Value::Clo(env, f, x, m) => {
                    let env = cons((f.clone(), m0.clone()), env.clone());
                    let env = cons((x.clone(), n), env);
                    eval(env, m.clone())
                }
                _ => panic!("eval_App({:?})", m0),
            }
        }
        LetIn(x, m, n) => {
            let m = eval(env.clone(), m.clone());
            let env = cons((x.clone(), m), env);
            eval(env, n.clone())
        }
        Ifte(m, n1, n2) => {
            let m = eval(env.clone(), m.clone());
            match &*m {
                Value::Bool(true) => eval(env, n1.clone()),
                Value::Bool(false) => eval(env, n2.clone()),
                _ => panic!("eval_Ifte({:?})", m0),
            }
        }
    }
}

fn eval_op1(op: &Op1, m: Rc<Value>) -> Rc<Value> {
    use self::Op1::*;
    use Value::*;
    match (op, &*m) {
        (Not, Bool(b)) => bool(!b),
        (Neg, Int(i)) => int(-i),
        (_, _) => panic!("eval_op1({:?}, {:?})", op, m),
    }
}

fn eval_op2(op: &Op2, m: Rc<Value>, n: Rc<Value>) -> Rc<Value> {
    use self::Op2::*;
    use Value::*;
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
