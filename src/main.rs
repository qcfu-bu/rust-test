mod ast0;
mod ast1;
mod env;
mod eval;
mod names;
mod parse;
mod trans01;

use env::nil;
use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use eval::*;
use im_rc::HashMap;
use parse::*;
use pest::Parser;
use std::fs;
use trans01::trans;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = fs::read_to_string(&args[1]).expect("cannot read file");
    match LamParser::parse(Rule::prog, &file) {
        Ok(mut pairs) => {
            let tm = parse_term(pairs.next().unwrap().into_inner());
            let tm = trans(HashMap::new(), tm);
            println!("term  : {:?}", tm);
            let val = eval(nil(), tm);
            println!("value : {:?}", val);
        }
        Err(e) => {
            eprintln!("Parse failed: {:?}", e)
        }
    }
}
