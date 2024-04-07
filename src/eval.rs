use crate::{ast1::*, names::Name};
use im::HashMap;
use std::rc::*;

type Env = HashMap<i32, Val>;

type Val = Rc<ValNode>;

#[derive(Debug)]
pub enum ValNode {
    Int(i32),
    Bool(bool),
    Clo(Env, Name, Name, Term),
}

pub fn eval(env: Env, m0: Term) -> Val {
    use TermNode::*;
    match &*m0 {
        Int(i) => Rc::new(ValNode::Int(*i)),
        Bool(b) => Rc::new(ValNode::Bool(*b)),
        Var(x) => match env.get(&x.id) {
            Some(v) => v.clone(),
            None => {
                println!("cannot find({:?})", x);
                panic!()
            }
        },
        Op1(op1, m) => {
            let m = eval(env.clone(), m.clone());
            eval_op1(op1, m)
        }
        Op2(op2, m, n) => {
            let m = eval(env.clone(), m.clone());
            let n = eval(env.clone(), n.clone());
            eval_op2(op2, m, n)
        }
        Fun(f, x, m) => {
            let f = f.clone();
            let x = x.clone();
            let m = m.clone();
            let clo = ValNode::Clo(env, f, x, m);
            Rc::new(clo)
        }
        App(m, n) => {
            let m0 = eval(env.clone(), m.clone());
            let n = eval(env.clone(), n.clone());
            match &*m0 {
                ValNode::Clo(env, f, x, m) => {
                    let mut env = env.clone();
                    env.insert(f.id, m0.clone());
                    env.insert(x.id, n.clone());
                    eval(env, m.clone())
                }
                _ => panic!("eval_App({:?})", m0),
            }
        }
        LetIn(x, m, n) => {
            let m = eval(env.clone(), m.clone());
            let mut env = env.clone();
            env.insert(x.clone().id, m);
            eval(env, n.clone())
        }
        Ifte(m, n1, n2) => {
            let m = eval(env.clone(), m.clone());
            match &*m {
                ValNode::Bool(true) => eval(env, n1.clone()),
                ValNode::Bool(false) => eval(env, n2.clone()),
                _ => panic!("eval_Ifte({:?})", m0),
            }
        }
    }
}

fn eval_op1(op: &Op1, m: Val) -> Val {
    use self::Op1::*;
    use ValNode::*;
    match (op, &*m) {
        (Not, Bool(b)) => Rc::new(Bool(!b)),
        (Neg, Int(i)) => Rc::new(Int(-i)),
        (_, _) => panic!("eval_op1({:?}, {:?})", op, m),
    }
}

fn eval_op2(op: &Op2, m: Val, n: Val) -> Val {
    use self::Op2::*;
    use ValNode::*;
    match (op, &*m, &*n) {
        (Add, Int(i), Int(j)) => Rc::new(Int(i + j)),
        (Sub, Int(i), Int(j)) => Rc::new(Int(i - j)),
        (Mul, Int(i), Int(j)) => Rc::new(Int(i * j)),
        (Div, Int(i), Int(j)) => Rc::new(Int(i / j)),
        (Lte, Int(i), Int(j)) => Rc::new(Bool(i <= j)),
        (Gte, Int(i), Int(j)) => Rc::new(Bool(i >= j)),
        (Lt, Int(i), Int(j)) => Rc::new(Bool(i < j)),
        (Gt, Int(i), Int(j)) => Rc::new(Bool(i > j)),
        (Eq, Int(i), Int(j)) => Rc::new(Bool(i == j)),
        (Neq, Int(i), Int(j)) => Rc::new(Bool(i != j)),
        (And, Bool(i), Bool(j)) => Rc::new(Bool(*i && *j)),
        (Or, Bool(i), Bool(j)) => Rc::new(Bool(*i || *j)),
        (_, _, _) => panic!("eval_op2({:?}, {:?}, {:?})", op, m, n),
    }
}
