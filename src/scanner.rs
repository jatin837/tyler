struct Scanner {
    source: String,
    tokens: Vec<Token> = Vec::new;
  
    Scanner(String source) {
      this.source = source;
    }
}

impl Scanner {
    new(&self, source: Token) {
        self.source = source
    }
}