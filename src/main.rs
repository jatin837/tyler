#![allow(dead_code)]
#![allow(missing_copy_implementations)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]


use path_abs::PathAbs;
use std::env;
use std::path::Path;

#[macro_use] extern crate maplit;

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
        raw_text.as_bytes().to_vec()
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
    use std::collections::HashMap;
    use crate::token::{
        Token,
        TokenType,
    };

    #[derive(Debug)]
    pub struct Scanner {
        token_list                 :   Vec<Token>,
        buff                       :   Vec<u8>,
        source                     :   Vec<u8>,
        curr_pos                   :   usize,
        start_pos                  :   usize,
        line_loc                   :   usize,
        single_char_type_tok       :   HashMap<char, TokenType>,
        potential_double_char_tok  :   HashMap<char, TokenType>,
        reserved_type_tok          :   HashMap<String, TokenType>,
    }

    impl Scanner {
        pub fn new(source: Vec<u8>) -> Scanner {
            Scanner {
                token_list: Vec::new(),
                buff: Vec::new(),
                source : source,
                curr_pos : 0,
                start_pos : 0,
                line_loc : 1,
                reserved_type_tok : hashmap!{
                    "or"   .to_string()  =>  TokenType::OR,
		    "log"  .to_string()  =>  TokenType::LOG,
                    "and"  .to_string()  =>  TokenType::AND,
                    "class".to_string()  =>  TokenType::CLASS,
                    "else" .to_string()  =>  TokenType::ELSE,
                    "false".to_string()  =>  TokenType::FALSE,
                    "fun"  .to_string()  =>  TokenType::FUN,
                    "for"  .to_string()  =>  TokenType::FOR,
                    "if"   .to_string()  =>  TokenType::IF,
                    "nil"  .to_string()  =>  TokenType::NIL,
                    "or"   .to_string()  =>  TokenType::OR,
                    "ret"  .to_string()  =>  TokenType::RETURN,
                    "super".to_string()  =>  TokenType::SUPER,
                    "this" .to_string()  =>  TokenType::THIS,
                    "true" .to_string()  =>  TokenType::TRUE,
                    "var"  .to_string()  =>  TokenType::VAR,
                    "while".to_string()  =>  TokenType::WHILE,
                },
                single_char_type_tok: hashmap!{
                      '('     =>     TokenType::LEFT_PAREN,  
                      ')'     =>     TokenType::RIGHT_PAREN, 
                      '{'     =>     TokenType::LEFT_BRACE,  
                      '}'     =>     TokenType::RIGHT_BRACE, 
                      ','     =>     TokenType::COMMA,       
                      '.'     =>     TokenType::DOT,         
                      '-'     =>     TokenType::MINUS,       
                      '+'     =>     TokenType::PLUS,        
                      ';'     =>     TokenType::SEMICOLON,   
                      '/'     =>     TokenType::SLASH,       
                      '*'     =>     TokenType::STAR,        
                },
                potential_double_char_tok: hashmap!{
                      '!'     =>     TokenType::BANG,    
                      '='     =>     TokenType::EQUAL,   
                      '>'     =>     TokenType::GREATER, 
                      '<'     =>     TokenType::LESS,    
                },
            }
        }

        pub fn add_tok(&mut self, T: Token) -> (){
            self.token_list.push(T);
        }

        pub fn incr_curr_pos(&mut self) -> () {
            self.curr_pos += 1;
        }

        fn is_at_end(&self) -> bool {
            self.curr_pos >= self.source.len() - 1
        }

        fn is_buff_empty(&self) -> bool {
            self.buff.len() == 0
        }

        pub fn get_tok_from_buff(&mut self) -> Token {
            let temp = String::from_utf8(self.buff.clone()).unwrap();
            let reserved_type = hashmap!{
                "or"    =>     Token::new(TokenType::OR,        self.line_loc,     "".to_string(),     "".to_string()),
                "and"   =>     Token::new(TokenType::AND,       self.line_loc,     "".to_string(),     "".to_string()),
                "class" =>     Token::new(TokenType::CLASS,     self.line_loc,     "".to_string(),     "".to_string()),
                "else"  =>     Token::new(TokenType::ELSE,      self.line_loc,     "".to_string(),     "".to_string()),
                "false" =>     Token::new(TokenType::FALSE,     self.line_loc,     "".to_string(),     "".to_string()),
                "fun"   =>     Token::new(TokenType::FUN,       self.line_loc,     "".to_string(),     "".to_string()),
                "for"   =>     Token::new(TokenType::FOR,       self.line_loc,     "".to_string(),     "".to_string()),
                "if"    =>     Token::new(TokenType::IF,        self.line_loc,     "".to_string(),     "".to_string()),
                "nil"   =>     Token::new(TokenType::NIL,       self.line_loc,     "".to_string(),     "".to_string()),
                "or"    =>     Token::new(TokenType::OR,        self.line_loc,     "".to_string(),     "".to_string()),
                "ret"   =>     Token::new(TokenType::RETURN,    self.line_loc,     "".to_string(),     "".to_string()),
                "super" =>     Token::new(TokenType::SUPER,     self.line_loc,     "".to_string(),     "".to_string()),
                "this"  =>     Token::new(TokenType::THIS,      self.line_loc,     "".to_string(),     "".to_string()),
                "true"  =>     Token::new(TokenType::TRUE,      self.line_loc,     "".to_string(),     "".to_string()),
                "var"   =>     Token::new(TokenType::VAR,       self.line_loc,     "".to_string(),     "".to_string()),
                "while" =>     Token::new(TokenType::WHILE,     self.line_loc,     "".to_string(),     "".to_string()),
            };
            let tok = Token::new(TokenType::IDENTIFIER, self.line_loc, String::from_utf8(self.buff.clone()).unwrap(), "".to_string());
            self.buff.clear();
            tok
        }


        pub fn get_tok(&mut self) -> Token {
            if self.is_at_end() {
                if self.is_buff_empty(){
                    return Token::new(TokenType::EOF, self.line_loc, "".to_string(), "".to_string());
                }
                else { 
                    return self.get_tok_from_buff();
                }
            }

            let a = self.source[self.curr_pos] as char;

            if self.single_char_type_tok.contains_key(&a){
               if !self.is_buff_empty() {
                   let tok = self.get_tok_from_buff();
                   return tok;
               }
               else {
                   self.incr_curr_pos();
                   let tok_t = self.single_char_type_tok[&a].clone();
                   return Token::new(tok_t, self.curr_pos, "".to_string(), "".to_string());
               }
            }

            if self.potential_double_char_tok.contains_key(&a){
		match a {
                    '!'    =>    {
                        self.incr_curr_pos();
                        if self.source[self.curr_pos] == '=' as u8 {
                            self.incr_curr_pos();
                            return Token::new(TokenType::BANG_EQUAL, self.line_loc, "".to_string(), "".to_string());
                        }
                        else {
                            return Token::new(TokenType::BANG, self.line_loc, "".to_string(), "".to_string());
                        }
                    }, 

                    '>'    =>    {
                        self.incr_curr_pos();
                        if self.source[self.curr_pos] == '=' as u8 {
                            self.incr_curr_pos();
                            return Token::new(TokenType::GREATER_EQUAL, self.line_loc, "".to_string(), "".to_string());
                        }
                        else {
                            return Token::new(TokenType::EQUAL, self.line_loc, "".to_string(), "".to_string());
                        }
                    }, 
                    '<'    =>     {
                        self.incr_curr_pos();
                        if self.source[self.curr_pos] == '=' as u8 {
                            self.incr_curr_pos();
                            return Token::new(TokenType::LESS_EQUAL, self.line_loc, "".to_string(), "".to_string());
                        }
                        else {
                            return Token::new(TokenType::EQUAL, self.line_loc, "".to_string(), "".to_string());
                        }
                    }, 
                    
                    '='    =>     {
                        self.incr_curr_pos();
                        if self.source[self.curr_pos] == '=' as u8 {
                            self.incr_curr_pos();
                            return Token::new(TokenType::EQUAL_EQUAL, self.line_loc, "".to_string(), "".to_string());
                        }
                        else {
                            return Token::new(TokenType::EQUAL, self.line_loc, "".to_string(), "".to_string());
                        }
                    },
                    
                    _    =>    {},
                }
            }

            match a {
                ' ' | '\t' => {
                       if self.buff.len() == 0 {
                           self.incr_curr_pos();
                           return self.get_tok();
                    }
                    else {
                       self.incr_curr_pos();
                       return self.get_tok_from_buff();
                    }
                }
                '\n' => {
                    self.incr_curr_pos();
                    self.line_loc += 1;
                    return self.get_tok();
                }

                _ => {
                   self.incr_curr_pos();
                   self.buff.push(a as u8);
                   return self.get_tok();
                }
            }
        }

        pub fn scan(&mut self) -> () {
            loop {
                let tok = self.get_tok();
                match tok.Type {
                    TokenType::EOF => {self.add_tok(tok);break;},
                    _ => {self.add_tok(tok);},
                }
            }
        }
        
        pub fn dump(&self, indx: usize) -> () {
            println!("{:?} at {:}, line = {:?}", self.source[indx] as char, indx, self.line_loc);
        }
    }
}

pub mod token {
    #[derive(Debug, Clone)]
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
        LOG,
        RETURN,
        SUPER,
        THIS,
        TRUE,
        VAR,
        WHILE,

        EOF,
    }  
    #[derive(Debug, Clone)]
    pub struct Token {
        pub Type: TokenType,
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
