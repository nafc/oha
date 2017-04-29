use lexer::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct Traveler {
    pub tokens: Vec<Token>,
    top: usize,
}

impl<'a> Traveler {
    pub fn new(tokens: Vec<Token>) -> Traveler {
        Traveler {
            tokens: tokens,
            top: 0,
        }
    }

    pub fn next(&mut self) -> bool {
        if self.top < self.tokens.len() {
            self.top += 1;
            return true
        }
        false
    }

    pub fn prev(&mut self) -> bool {
        if self.top > 0 {
            self.top -= 1;
            return true
        }
        false
    }

    pub fn remaining(&self) -> usize {
        self.tokens.len() - self.top + 1
    }

    pub fn current(&self) -> &Token {
        if self.top > self.tokens.len() - 1 {
            return &self.tokens[self.tokens.len() - 1];
        }
        &self.tokens[self.top]
    }

    pub fn current_content(&self) -> String {
        self.current().content().clone()
    }

    pub fn expect(&self, token: TokenType) -> Result<&Token, String> {
        if self.current().token_type == token {
            Ok(self.current())
        } else {
            Err(format!("expected {:?} but found {:#?}", token, self.current()))
        }
    }

    pub fn expect_content(&self, content: String) -> Result<&Token, String> {
        if self.current_content() == content {
            Ok(self.current())
        } else {
            Err(format!("expected {} but found {:#?}", content, self.current()))
        }
    }
}