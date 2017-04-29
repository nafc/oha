use parser::lexer;

use lexer::{Token, TokenType};
use lexer::block_tree;

#[derive(Debug, Clone)]
pub struct Traveler {
    pub tokens: Vec<Token>,
    top: usize,
}

impl<'a> Traveler {
    pub fn from(tokens: Vec<Token>) -> Traveler {
        Traveler {
            tokens: tokens,
            top: 0,
        }
    }

    pub fn next(&mut self) -> bool {
        if self.top < self.tokens.len() {
            self.top += 1;

            return true;
        }

        false
    }

    pub fn prev(&mut self) -> bool {
        if self.top > 0 {
            self.top -= 1;

            return true;
        }

        false
    }

    pub fn remaining(&self) -> usize {
        self.tokens.len() - self.top
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
        match self.current().token_type == token {
            true => Ok(self.current()),
            false => Err(format!("expected {:?} but found {:#?}", token, self.current())),
        }
    }
}