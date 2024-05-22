use crate::ast0;
use ast0::*;
use bumpalo::Bump;
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;

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

pub fn parse_term<'a>(pairs: Pairs<Rule>, bump: &'a Bump) -> &'a Term<'a> {
    use self::Op1::*;
    use self::Op2::*;
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::bool => bool(primary.as_str().parse::<bool>().unwrap(), bump),
            Rule::integer => int(primary.as_str().parse::<i32>().unwrap(), bump),
            Rule::var => var(String::from(primary.as_str()), bump),
            Rule::letin => {
                let outer = primary.into_inner().next().unwrap();
                match outer.as_rule() {
                    Rule::decl_rec => {
                        let mut inner = outer.into_inner();
                        let f = String::from(inner.next().unwrap().as_str());
                        let x = String::from(inner.next().unwrap().as_str());
                        let args = inner.next().unwrap().into_inner();
                        let mut body = parse_term(inner.next().unwrap().into_inner(), bump);
                        let m = parse_term(inner.next().unwrap().into_inner(), bump);
                        for arg in args.rev() {
                            body = fun(String::from(""), String::from(arg.as_str()), body, bump)
                        }
                        letin(f.clone(), fun(f, x, body, bump), m, bump)
                    }
                    Rule::decl => {
                        let mut inner = outer.into_inner();
                        let x = String::from(inner.next().unwrap().as_str());
                        let args = inner.next().unwrap().into_inner();
                        let mut body = parse_term(inner.next().unwrap().into_inner(), bump);
                        let m = parse_term(inner.next().unwrap().into_inner(), bump);
                        for arg in args.rev() {
                            body = fun(String::from(""), String::from(arg.as_str()), body, bump)
                        }
                        letin(x, body, m, bump)
                    }
                    _ => panic!(),
                }
            }
            Rule::lambda => {
                let mut inner = primary.into_inner();
                let args = inner.next().unwrap().into_inner();
                let mut body = parse_term(inner.next().unwrap().into_inner(), bump);
                for arg in args.rev() {
                    body = fun(String::from(""), String::from(arg.as_str()), body, bump)
                }
                body
            }
            Rule::ifte => {
                let mut inner = primary.into_inner();
                let cond = parse_term(inner.next().unwrap().into_inner(), bump);
                let m1 = parse_term(inner.next().unwrap().into_inner(), bump);
                let m2 = parse_term(inner.next().unwrap().into_inner(), bump);
                ifte(cond, m1, m2, bump)
            }
            Rule::term => parse_term(primary.into_inner(), bump),
            _ => panic!(),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => op2(Add, lhs, rhs, bump),
            Rule::sub => op2(Sub, lhs, rhs, bump),
            Rule::mul => op2(Mul, lhs, rhs, bump),
            Rule::div => op2(Div, lhs, rhs, bump),
            Rule::lte => op2(Lte, lhs, rhs, bump),
            Rule::gte => op2(Gte, lhs, rhs, bump),
            Rule::lt => op2(Lt, lhs, rhs, bump),
            Rule::gt => op2(Gt, lhs, rhs, bump),
            Rule::eq => op2(Eq, lhs, rhs, bump),
            Rule::neq => op2(Neq, lhs, rhs, bump),
            Rule::and => op2(And, lhs, rhs, bump),
            Rule::or => op2(Or, lhs, rhs, bump),
            Rule::app => app(lhs, rhs, bump),
            _ => panic!(),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::not => op1(Not, rhs, bump),
            Rule::neg => op1(Neg, rhs, bump),
            _ => panic!(),
        })
        .parse(pairs)
}
