mod libs;
pub struct Token {
    TYPE: libs::TokenType,
    lexeme: String,
    literal: Literal,
    line: i32, 
  
  }
impl Token {
      pub fn to_string(&self){
          //return self.Type + " " self.lexeme + " " + self.literal
      }
  }