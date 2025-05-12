mod ast0;
// mod ast1;
mod ast2;
// mod eval;
mod names;
mod parse;
// mod trans01;
mod trans02;

use ahash::HashMap;
use ast2::reduce;
use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

// use eval::*;
use parse::*;
use pest::Parser;
use rpds::{HashTrieMap, RedBlackTreeMap};
use std::{cell::RefCell, fs, rc::Rc};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = fs::read_to_string(&args[1]).expect("cannot read file");
    match LamParser::parse(Rule::prog, &file) {
        Ok(mut pairs) => {
            let tm = parse_term(pairs.next().unwrap().into_inner());
            let tm = trans02::trans(Rc::new(HashMap::default()), tm);
            let val = reduce(tm);
            // let val = eval(HashTrieMap::default(), tm);
            println!("value : {:?}", val);
        }
        Err(e) => {
            eprintln!("Parse failed: {:?}", e)
        }
    }
}
