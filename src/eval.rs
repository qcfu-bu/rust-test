use crate::{ast1::*, names::Name};
use ahash::HashMap;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

type Env = Rc<RefCell<HashMap<Rc<Name>, Rc<Value>>>>;

#[derive(Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Clo(Env, Rc<Name>, Rc<Name>, Rc<Term>),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Int(i) => f.write_fmt(format_args!("Int({})", i)),
            Self::Bool(b) => f.write_fmt(format_args!("Bool({})", b)),
            Self::Clo(_, x, y, m) => f.write_fmt(format_args!("Clo({:?}, {:?}, {:?})", x, y, m)),
        }
    }
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
    // println!("eval({:?})", m0);
    match &*m0 {
        Int(i) => int(*i),
        Bool(b) => bool(*b),
        Var(x) => match env.borrow().get(x) {
            Some(v) => v.clone(),
            None => {
                println!("cannot find({:?})", x);
                panic!()
            }
        },
        Op1(op1, m) => {
            let m = eval(env, m.clone());
            eval_op1(op1, m)
        }
        Op2(op2, m, n) => {
            let m = eval(env.clone(), m.clone());
            let n = eval(env, n.clone());
            eval_op2(op2, m, n)
        }
        Fun(f, x, m) => clo(Rc::new((*env).clone()), f.clone(), x.clone(), m.clone()),
        App(m, n) => {
            let m0 = eval(env.clone(), m.clone());
            let n = eval(env, n.clone());
            match &*m0 {
                Value::Clo(env, f, x, m) => {
                    // println!("insert({:?}, {:?})", f, m0);
                    let opt1 = env.borrow_mut().insert(f.clone(), m0.clone());
                    // println!("insert({:?}, {:?})", x, n);
                    let opt2 = env.borrow_mut().insert(x.clone(), n);
                    let result = eval(env.clone(), m.clone());
                    opt1.and_then(|v| env.borrow_mut().insert(f.clone(), v));
                    opt2.and_then(|v| env.borrow_mut().insert(x.clone(), v));
                    return result;
                }
                _ => panic!("eval_App({:?})", m0),
            }
        }
        LetIn(x, m, n) => {
            let m = eval(env.clone(), m.clone());
            let opt = env.borrow_mut().insert(x.clone(), m);
            let result = eval(env.clone(), n.clone());
            opt.and_then(|v| env.borrow_mut().insert(x.clone(), v));
            return result;
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
    // println!("eval_op1({:?}, {:?})", op, m);
    match (op, &*m) {
        (Not, Bool(b)) => bool(!b),
        (Neg, Int(i)) => int(-i),
        (_, _) => panic!("eval_op1({:?}, {:?})", op, m),
    }
}

fn eval_op2(op: &Op2, m: Rc<Value>, n: Rc<Value>) -> Rc<Value> {
    use self::Op2::*;
    use Value::*;
    // println!("eval_op2({:?}, {:?}, {:?})", op, m, n);
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
