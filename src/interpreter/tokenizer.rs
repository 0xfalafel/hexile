enum Token {
    Integer,
    Char,
    Variable(String),
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
}