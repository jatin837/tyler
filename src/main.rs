<<<<<<< HEAD
use std::env;
mod utils;
use utils::*;
fn main() {
    let args: Vec<String> = env::args().collect();
    parse_args(&args);
=======
use path_abs::PathAbs;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

//use std::io::Read;

enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

struct Scanner {
    token_list : Vec<Token>,
}

struct Token {
    Type: TokenType,
    line: i64,
    literal: String,
    lexeme: String,
}

fn run_prompt() {
    // TOBE implemented
    loop {
        let mut line = String::new();
        println!("\x1b[0;31mtyli>>\x1b[0m");
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        println!("no of bytes read , {}", b1);
    }
}

fn to_bytes(raw_text: &String) -> &[u8] {
    raw_text.as_bytes()
}

fn run_file(fpath: &Path) {
    //reading file first
    let fp = File::open(&fpath).expect("[ERROR] opening file");
    let mut buf_reader = BufReader::new(fp);
    let mut src_code = String::new();
    buf_reader
        .read_to_string(&mut src_code)
        .expect("[ERROR] can't read file");
    let bytes = to_bytes(&src_code);
    print!("File {:?} \n-----------\n{:?}", fpath, src_code);
    print!("bytes string  {:?} \n-----------\n{:?}", fpath, bytes);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => {
            let filename: &String = &args[1];
            let abs_file_path = PathAbs::new(filename).unwrap();
            let filepath: &Path = Path::new(&abs_file_path);
            if !filepath.exists() {
                eprintln!("[ERROR] provide a file that exists")
            } else {
                run_file(&filepath)
            }
        }
        _ => eprintln!("[USAGE] tyler <file_name> (for script), tyler (for running prompt)"),
    }
>>>>>>> old
}
