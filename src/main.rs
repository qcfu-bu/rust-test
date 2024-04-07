mod ast0;
mod ast1;
mod eval;
mod names;
mod parse;
mod trans01;
use eval::*;
use im::HashMap;
use parse::*;
use pest::Parser;
use std::env;
use std::fs;
use std::rc::Rc;

use crate::trans01::trans;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).expect("cannot read file");
    match LamParser::parse(Rule::prog, &file) {
        Ok(mut pairs) => {
            let tm = parse_term(pairs.next().unwrap().into_inner());
            let tm = trans(HashMap::new(), Rc::new(tm));
            println!("term  : {:?}", tm);
            println!("value : {:?}", eval(HashMap::new(), tm));
        }
        Err(e) => {
            eprintln!("Parse failed: {:?}", e)
        }
    }
}
