use std::env;
use std::path::Path;
use path_abs::PathAbs;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

pub enum token_type {
    // Single-character tokens.
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

//use std::io::Read;



pub fn tokenize(raw: &String) -> Vec<String>{
    vec!{String::from("ab"), String::from("dev")}
}

pub fn run_prompt(){
    // TOBE implemented
    loop {
        let mut line = String::new();
        println!("\x1b[0;31mtyli>>\x1b[0m");
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        println!("no of bytes read , {}", b1);
    }
}

pub fn to_bytes(raw_text: &String) -> &[u8] {
    raw_text.as_bytes()
}

pub fn run_file(fpath: &Path) {
//reading file first
    let fp = File::open(&fpath).expect("[ERROR] opening file");
    let mut buf_reader = BufReader::new(fp);
    let mut src_code = String::new();
    buf_reader.read_to_string(&mut src_code).expect("[ERROR] can't read file");
    let bytes = to_bytes(&src_code);
    println!("File {:?} \n-----------\n{:?}", fpath, src_code);
    println!("bytes string  {:?} \n-----------\n{:?}", fpath, bytes);
}