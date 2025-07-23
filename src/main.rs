mod ast0;
mod ast1;
mod eval;
mod names;
mod parse;
mod trans01;

use ahash::HashMap;
use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use eval::*;
use parse::*;
use pest::Parser;
use std::{cell::RefCell, fs};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = fs::read_to_string(&args[1]).expect("cannot read file");
    match LamParser::parse(Rule::prog, &file) {
        Ok(mut pairs) => {
            let tm = parse_term(pairs.next().unwrap().into_inner());
            let tm = trans01::trans(&mut HashMap::default(), tm.as_ref());
            let val = eval(&RefCell::new(HashMap::default()), tm.as_ref());
            println!("value : {:?}", val);
        }
        Err(e) => {
            eprintln!("Parse failed: {:?}", e)
        }
    }
}
