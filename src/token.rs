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

   pub struct Token {
       token_type: TokenType,
       lexeme: String,
       literal: Object,
       line: int,
   } 
   impl Token {
       pub fn new(self, token_type: TokenType, lexeme: String, literal: Object, line: int){
        self.token_type = token_type;
        self.lexeme = lexeme;
        self.literal = literal;
        self.line = line;
       }

       pub fn to_string(self) -> String {
           concat!(self.token_type, " ", self.lexeme, " ", self.literal, " ", self.line)
       }
   }
} 
