use crate::ast0;
use ast0::*;
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use std::rc::Rc;

#[derive(pest_derive::Parser)]
#[grammar = "lam.pest"]
pub struct LamParser;

lazy_static::lazy_static! {
  static ref PRATT_PARSER: PrattParser<Rule> = {
    use pest::pratt_parser::{Assoc::*, Op};
    use Rule::*;
    PrattParser::new()
      .op(Op::infix(and, Left) | Op::infix(or, Left))
      .op(Op::infix(eq, Left) | Op::infix(neq, Left))
      .op(Op::infix(lte, Left) | Op::infix(gte, Left) | Op::infix(lt, Left) | Op::infix(gt, Left))
      .op(Op::infix(add, Left) | Op::infix(sub, Left))
      .op(Op::infix(mul, Left) | Op::infix(div, Left))
      .op(Op::prefix(neg) | Op::prefix(not))
      .op(Op::infix(app, Left))
  };
}

pub fn parse_term(pairs: Pairs<Rule>) -> TermNode {
    use self::Op1::*;
    use self::Op2::*;
    use TermNode::*;
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::bool => Bool(primary.as_str().parse::<bool>().unwrap()),
            Rule::integer => Int(primary.as_str().parse::<i32>().unwrap()),
            Rule::var => Var(String::from(primary.as_str())),
            Rule::letin => {
                let outer = primary.into_inner().next().unwrap();
                match outer.as_rule() {
                    Rule::decl_rec => {
                        let mut inner = outer.into_inner();
                        let f = String::from(inner.next().unwrap().as_str());
                        let x = String::from(inner.next().unwrap().as_str());
                        let args = inner.next().unwrap().into_inner();
                        let mut body = parse_term(inner.next().unwrap().into_inner());
                        let m = parse_term(inner.next().unwrap().into_inner());
                        for arg in args.rev() {
                            body = Fun(String::from(""), String::from(arg.as_str()), Rc::new(body))
                        }
                        LetIn(f.clone(), Rc::new(Fun(f, x, Rc::new(body))), Rc::new(m))
                    }
                    Rule::decl => {
                        let mut inner = outer.into_inner();
                        let x = String::from(inner.next().unwrap().as_str());
                        let args = inner.next().unwrap().into_inner();
                        let mut body = parse_term(inner.next().unwrap().into_inner());
                        let m = parse_term(inner.next().unwrap().into_inner());
                        for arg in args.rev() {
                            body = Fun(String::from(""), String::from(arg.as_str()), Rc::new(body))
                        }
                        LetIn(x, Rc::new(body), Rc::new(m))
                    }
                    _ => panic!(),
                }
            }
            Rule::lambda => {
                let mut inner = primary.into_inner();
                let args = inner.next().unwrap().into_inner();
                let mut body = parse_term(inner.next().unwrap().into_inner());
                for arg in args.rev() {
                    body = Fun(String::from(""), String::from(arg.as_str()), Rc::new(body))
                }
                body
            }
            Rule::ifte => {
                let mut inner = primary.into_inner();
                let cond = parse_term(inner.next().unwrap().into_inner());
                let m1 = parse_term(inner.next().unwrap().into_inner());
                let m2 = parse_term(inner.next().unwrap().into_inner());
                Ifte(Rc::new(cond), Rc::new(m1), Rc::new(m2))
            }
            Rule::term => parse_term(primary.into_inner()),
            _ => panic!(),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => Op2(Add, Rc::new(lhs), Rc::new(rhs)),
            Rule::sub => Op2(Sub, Rc::new(lhs), Rc::new(rhs)),
            Rule::mul => Op2(Mul, Rc::new(lhs), Rc::new(rhs)),
            Rule::div => Op2(Div, Rc::new(lhs), Rc::new(rhs)),
            Rule::lte => Op2(Lte, Rc::new(lhs), Rc::new(rhs)),
            Rule::gte => Op2(Gte, Rc::new(lhs), Rc::new(rhs)),
            Rule::lt => Op2(Lt, Rc::new(lhs), Rc::new(rhs)),
            Rule::gt => Op2(Gt, Rc::new(lhs), Rc::new(rhs)),
            Rule::eq => Op2(Eq, Rc::new(lhs), Rc::new(rhs)),
            Rule::neq => Op2(Neq, Rc::new(lhs), Rc::new(rhs)),
            Rule::and => Op2(And, Rc::new(lhs), Rc::new(rhs)),
            Rule::or => Op2(Or, Rc::new(lhs), Rc::new(rhs)),
            Rule::app => App(Rc::new(lhs), Rc::new(rhs)),
            _ => panic!(),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::not => Op1(Not, Rc::new(rhs)),
            Rule::neg => Op1(Neg, Rc::new(rhs)),
            _ => panic!(),
        })
        .parse(pairs)
}
