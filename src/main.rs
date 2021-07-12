use path_abs::PathAbs;
use std::env;
use std::path::Path;
//use std::io::Read;

pub mod helper {

    use crate::scanner;
    use std::io::prelude::*;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;
    pub fn run_prompt() {
        // TOBE implemented
        loop {
            let mut line = String::new();
            println!("\x1b[0;31mtyli>>\x1b[0m");
            let b1 = std::io::stdin().read_line(&mut line).unwrap();
            println!("no of bytes read , {}", b1);
        }
    }

    pub fn to_bytes(raw_text: &String) -> Vec<u8> {
        let mut res = Vec::new();
        res = raw_text.as_bytes().to_vec();
        res
    }

    pub fn run_file(fpath: &Path) {
        //reading file first
        let fp = File::open(&fpath).expect("[ERROR] opening file");
        let mut buf_reader = BufReader::new(fp);
        let mut src_code = String::new();
        buf_reader
            .read_to_string(&mut src_code)
            .expect("[ERROR] can't read file");
        let bytes = to_bytes(&src_code);
        let mut scanner = scanner::Scanner::new(bytes);
        scanner.scan();
        println!("------------------------------------");
        println!("{:?}", scanner);
    }
}

pub mod scanner {
    use crate::token::{
        Token,
        TokenType,
    };

    #[derive(Debug)]
    pub struct Scanner {
        token_list : Vec<Token>,
        source: Vec<u8>,
        curr_pos: usize,
    }

    impl Scanner {
        pub fn new(source: Vec<u8>) -> Scanner {
            Scanner {
                token_list: Vec::new(),
                source : source,
                curr_pos : 0,
            }
        }

        pub fn scan(&mut self) -> () {
            while self.curr_pos < self.source.len(){
                self.dump(self.curr_pos);
                match self.source[self.curr_pos] as char{
                    '(' => {
                        let temp = Token::new(TokenType::LEFT_PAREN, 0, String::from(""), String::from(""));
                        self.token_list.push(temp);
                    },

                    ')' => {
                        let temp = Token::new(TokenType::RIGHT_PAREN, 0, String::from(""), String::from(""));
                        self.token_list.push(temp);
                    },

                    '{' => {
                        let temp = Token::new(TokenType::LEFT_BRACE, 0, String::from(""), String::from(""));
                        self.token_list.push(temp);
                    },

                    '}' => {
                        let temp = Token::new(TokenType::RIGHT_BRACE, 0, String::from(""), String::from(""));
                        self.token_list.push(temp);
                    },
 
                    ',' => {
                        let temp = Token::new(TokenType::COMMA, 0, String::from(""), String::from(""));
                        self.token_list.push(temp);
                    },
 
                    '.' => {
                        let temp = Token::new(TokenType::DOT, 0, String::from(""), String::from(""));
                        self.token_list.push(temp);
                    },
 
                    '-' => {
                        let temp = Token::new(TokenType::MINUS, 0, String::from(""), String::from(""));
                        self.token_list.push(temp);
                    },
 
                    '+' => {
                        let temp = Token::new(TokenType::PLUS, 0, String::from(""), String::from(""));
                        self.token_list.push(temp);
                    },
 
                    ';' => {
                        let temp = Token::new(TokenType::SEMICOLON, 0, String::from(""), String::from(""));
                        self.token_list.push(temp);
                    },
 
                    '/' => {
                        let temp = Token::new(TokenType::SLASH, 0, String::from(""), String::from(""));
                        self.token_list.push(temp);
                    },
 
                    '*' => {
                        let temp = Token::new(TokenType::STAR, 0, String::from(""), String::from(""));
                        self.token_list.push(temp);
                    },
 
                    _ => { println!("TO BE IMPLEMENTED");},
                }
                self.curr_pos += 1;
            }
            println!("EOF");
        }
        
        pub fn dump(&self, indx: usize) -> () {
            println!("{:?} at {:}", self.source[indx] as char, indx);
        }
    }
}

pub mod token {
    #[derive(Debug)]
    pub enum TokenType {
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
    #[derive(Debug)]
    pub struct Token {
        Type: TokenType,
        line: usize,
        literal: String,
        lexeme: String,
    }
    
    impl Token {
        pub fn new(Type: TokenType, line: usize, literal: String, lexeme: String) -> Token {
            Token {
                Type: Type,
                line: line,
                literal: literal,
                lexeme: lexeme,
            }
        }
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => helper::run_prompt(),
        2 => {
            let filename: &String = &args[1];
            let abs_file_path = PathAbs::new(filename).unwrap();
            let filepath: &Path = Path::new(&abs_file_path);
            if !filepath.exists() {
                eprintln!("[ERROR] provide a file that exists")
            } else {
                helper::run_file(&filepath)
            }
        }
        _ => eprintln!("[USAGE] tyler <file_name> (for script), tyler (for running prompt)"),
    }
}
