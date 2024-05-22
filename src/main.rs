mod ast0;
mod ast1;
mod eval;
mod names;
mod parse;
mod trans01;

use bumpalo::Bump;
use eval::*;
use im_rc::HashMap;
use parse::*;
use pest::Parser;
use std::env;
use std::fs;
use trans01::trans;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).expect("cannot read file");
    match LamParser::parse(Rule::prog, &file) {
        Ok(mut pairs) => {
            let mut bump1 = Bump::new();
            let bump2 = Bump::new();
            let tm = parse_term(pairs.next().unwrap().into_inner(), &bump1);
            let tm = trans(HashMap::new(), tm, &bump2);
            bump1.reset();
            let val = eval(HashMap::new(), tm, &bump2);
            println!("term  : {:?}", tm);
            println!("value : {:?}", val);
        }
        Err(e) => {
            eprintln!("Parse failed: {:?}", e)
        }
    }
}
