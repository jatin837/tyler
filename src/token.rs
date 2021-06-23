use std::env;
use std::path::Path;
use path_abs::PathAbs;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

pub mod token {
    pub enum TokenType {
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

    pub fn tokenize(raw: &String) -> Vec<&str>{
        /*
        tokenize based on 
        */
        let tokens: Vec<&str> = raw.split(" ").collect();
        tokens
    }
} 
