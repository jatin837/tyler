use std::env;
mod utils;
use utils::*;
fn main() {
    let args: Vec<String> = env::args().collect();
    parse_args(&args);
}

