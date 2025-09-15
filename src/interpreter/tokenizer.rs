enum Token {
    Integer,
    Char,
    Variable(String),
    EOF,
}

#[derive(Debug, Clone)]
pub struct Tokenizer {
    text: String,
    pos: usize,
    // variables: Variables,
}

/// The Tokenizer is in charge of spliting the input in a bunch of tokens.
impl Tokenizer {
    pub fn new(text: String) -> Tokenizer {
        Tokenizer {
            text: text,
            pos: 0,
            // variables: variables,
        }
    }

    /// Advance the `pos` pointer and set the `current_char` variable.
    fn advance(&mut self) {
        self.pos += 1
    }

    /// Return the char at the `pos` position
    fn get_char(&self) -> Option<char> {
        self.text.chars().nth(self.pos)
    }

    /// advance `self.pos` until the next non-whitespace character
    fn skip_whitespace(&mut self) {

        while self.pos < self.text.len() && self.get_char().unwrap_or_default().is_whitespace() {
            self.pos += 1;
        }
    }

    /// This method is responsible for breaking a sentence appart into tokens
    pub fn get_next_token(&mut self) -> Result<Token, Error> {
        
        self.skip_whitespace();

        let char = match self.get_char() {
            None => return Token::EOF,
            Some(char) => char,
        };
    }

}