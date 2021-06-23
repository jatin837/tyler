use std::env;
use tyler::token::*;
use token::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    parse_args(&args);
}

