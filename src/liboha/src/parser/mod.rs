pub mod traveler;
pub mod ast;

pub use self::ast::{Expression, Statement};
pub use self::traveler::Traveler;

pub use super::lexer;
pub use lexer::TokenType;

pub struct Parser {
    traveler: Traveler,
}

impl Parser {
    pub fn new(traveler: Traveler) -> Parser {
        Parser {
            traveler: traveler,
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut stack = Vec::new();

        while self.traveler.remaining() > 1 {
            stack.push(self.statement());
            self.traveler.next();
        }

        stack
    }

    fn statement(&mut self) -> Statement {
        match self.traveler.current().token_type {
            _ => Statement::Expression(Box::new(self.expression())),
        }
    }

    fn expression(&mut self) -> Expression {
        let expr = self.atom();

        self.traveler.next();

        if self.traveler.remaining() > 0 {
            self.traveler.prev();
        }

        expr
    }

    fn atom(&mut self) -> Expression {
        match self.traveler.current().token_type {
            TokenType::IntLiteral   => Expression::IntLiteral(self.traveler.current_content().parse::<i32>().unwrap()),
            TokenType::FloatLiteral => Expression::FloatLiteral(self.traveler.current_content().parse::<f32>().unwrap()),
            ref t => panic!("unexpected term: {:?}", t),
        }
    }
}