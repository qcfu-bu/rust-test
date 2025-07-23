use crate::{ast1::*, names::Name};
use ahash::HashMap;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

type Env<'a> = RefCell<HashMap<Name, Value<'a>>>;

pub struct Closure<'a> {
    pub env: Env<'a>,
    pub func: Name,
    pub param: Name,
    pub body: &'a Term,
}

impl Debug for Closure<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Closure({:?}, {:?})", self.func, self.param)
    }
}

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Int(i32),
    Bool(bool),
    Clo(Rc<Closure<'a>>),
}

pub fn eval<'a>(env: &Env<'a>, m0: &'a Term) -> Value<'a> {
    use Term::*;
    match m0 {
        Int(i) => Value::Int(*i),
        Bool(b) => Value::Bool(*b),
        Var(x) => match env.borrow().get(x) {
            Some(v) => v.clone(),
            None => {
                println!("cannot find({:?})", x);
                panic!()
            }
        },
        Op1(op1, m) => {
            let m = eval(env, m);
            eval_op1(op1, &m)
        }
        Op2(op2, m, n) => {
            let m = eval(env, m);
            let n = eval(env, n);
            eval_op2(op2, &m, &n)
        }
        Fun(f, x, m) => Value::Clo(Rc::new(Closure {
            env: env.clone(),
            func: f.clone(),
            param: x.clone(),
            body: m,
        })),
        App(m, n) => {
            let m0 = eval(env, m);
            let n0 = eval(env, n);
            match &m0 {
                Value::Clo(clo) => {
                    let local = &clo.env;
                    let opt1 = local.borrow_mut().insert(clo.func.clone(), m0.clone());
                    let opt2 = local.borrow_mut().insert(clo.param.clone(), n0);
                    let result = eval(local, clo.body);
                    if let Some(v) = opt1 {
                        local.borrow_mut().insert(clo.func.clone(), v);
                    } else {
                        local.borrow_mut().remove(&clo.func);
                    };
                    if let Some(v) = opt2 {
                        local.borrow_mut().insert(clo.param.clone(), v);
                    } else {
                        local.borrow_mut().remove(&clo.param);
                    };
                    return result;
                }
                _ => panic!("eval_App({:?})", m0),
            }
        }
        LetIn(x, m, n) => {
            let m = eval(env, m);
            let opt = env.borrow_mut().insert(x.clone(), m);
            let result = eval(env, n);
            opt.and_then(|v| env.borrow_mut().insert(x.clone(), v));
            return result;
        }
        Ifte(m, n1, n2) => {
            let m = eval(env, m);
            match m {
                Value::Bool(true) => eval(env, n1),
                Value::Bool(false) => eval(env, n2),
                _ => panic!("eval_Ifte({:?})", m0),
            }
        }
    }
}

fn eval_op1<'a>(op: &'a Op1, m: &Value<'a>) -> Value<'a> {
    use self::Op1::*;
    use Value::*;
    match (op, m) {
        (Not, Bool(b)) => Value::Bool(!b),
        (Neg, Int(i)) => Value::Int(-i),
        (_, _) => panic!("eval_op1({:?}, {:?})", op, m),
    }
}

fn eval_op2<'a>(op: &'a Op2, m: &Value<'a>, n: &Value<'a>) -> Value<'a> {
    use self::Op2::*;
    use Value::*;
    match (op, m, n) {
        (Add, Int(i), Int(j)) => Value::Int(i + j),
        (Sub, Int(i), Int(j)) => Value::Int(i - j),
        (Mul, Int(i), Int(j)) => Value::Int(i * j),
        (Div, Int(i), Int(j)) => Value::Int(i / j),
        (Lte, Int(i), Int(j)) => Value::Bool(i <= j),
        (Gte, Int(i), Int(j)) => Value::Bool(i >= j),
        (Lt, Int(i), Int(j)) => Value::Bool(i < j),
        (Gt, Int(i), Int(j)) => Value::Bool(i > j),
        (Eq, Int(i), Int(j)) => Value::Bool(i == j),
        (Neq, Int(i), Int(j)) => Value::Bool(i != j),
        (And, Bool(i), Bool(j)) => Value::Bool(*i && *j),
        (Or, Bool(i), Bool(j)) => Value::Bool(*i || *j),
        (_, _, _) => panic!("eval_op2({:?}, {:?}, {:?})", op, m, n),
    }
}
