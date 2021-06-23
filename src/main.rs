use std::env;
mod libs;
use token::*;
use libs::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    parse_args(&args);
}

