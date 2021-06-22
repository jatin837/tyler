mod libs;
libs::*;
pub struct Token {
    TYPE: TokenType,
    lexeme: String,
    literal: Literal,
    line: i32, 
  
    // Token(TokenType type, String lexeme, Object literal, int line) {
    //   this.type = type;
    //   this.lexeme = lexeme;
    //   this.literal = literal;
    //   this.line = line;
    // }
  
    // public String toString() {
    //   return type + " " + lexeme + " " + literal;
    // }
  }
pub impl Token {
      to_string(&self){
          //return self.Type + " " self.lexeme + " " + self.literal
      }
  }