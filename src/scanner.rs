pub mod Scanner{
    pub struct Scanner {
        source: String,
        tokens: Vec<Token>,
    }

    impl Scanner {
        pub fn new(&self, source: Token) {
            self.source = source;
        }
    }
}