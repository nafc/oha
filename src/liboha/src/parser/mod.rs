pub mod traveler;
pub mod ast;

pub use self::ast::{Expression, Statement, Operand, operand};
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

    #[allow(unused_must_use)]
    fn statement(&mut self) -> Statement {
        match self.traveler.current().token_type {
            TokenType::Identifier => {
                let ident = self.atom();
                self.traveler.next();

                let content = self.traveler.current_content();

                if content != "=" {
                    self.traveler.prev();
                    return Statement::Expression(Box::new(self.expression()))
                }

                self.traveler.next();
                Statement::Assignment(Box::new(ident), Box::new(self.expression()))
            },

            TokenType::Keyword => match self.traveler.current_content().as_str() {
                "let" => {
                    self.traveler.next();
                    let ident = self.atom();

                    self.traveler.next();

                    if self.traveler.current_content() == "=" {
                        self.traveler.next();

                        Statement::Declaration(Box::new(ident), Some(Box::new(self.expression())))
                    } else {
                        self.traveler.prev();
                        Statement::Declaration(Box::new(ident), None)
                    }
                }
                _ => Statement::Expression(Box::new(self.expression())),
            },
            _ => Statement::Expression(Box::new(self.expression())),
        }
    }

    fn expression(&mut self) -> Expression {
        let expr = self.atom();
        self.traveler.next();

        if self.traveler.remaining() > 0 {
            if self.traveler.current().token_type == TokenType::Operator {
                return self.operation(expr)
            }

            self.traveler.prev();
        }

        expr
    }

    fn atom(&mut self) -> Expression {
        match self.traveler.current().token_type.clone() {
            TokenType::IntLiteral    => Expression::IntLiteral(self.traveler.current_content().parse::<i32>().unwrap()),
            TokenType::FloatLiteral  => Expression::FloatLiteral(self.traveler.current_content().parse::<f32>().unwrap()),
            TokenType::BoolLiteral   => Expression::BoolLiteral(self.traveler.current_content() == "true"),
            TokenType::StringLiteral => Expression::StringLiteral(self.traveler.current_content().clone()),
            TokenType::CharLiteral   => Expression::CharLiteral(self.traveler.current_content().chars().nth(0).unwrap().clone()),
            TokenType::Identifier    => Expression::Identifier(self.traveler.current_content()),

            _ => panic!("unexpected atom: '{}'", self.traveler.current_content()),
        }
    }

    fn operation(&mut self, expression: Expression) -> Expression {

        let mut ex_stack = vec![expression];
        let mut op_stack: Vec<(Operand, u8)> = Vec::new();

        op_stack.push(operand(&self.traveler.current_content()).unwrap());

        self.traveler.next();

        ex_stack.push(self.atom());

        let mut done = false;
        while ex_stack.len() > 1 {

            if !done && self.traveler.next() {
                if self.traveler.current().token_type != TokenType::Operator {
                    self.traveler.prev();

                    done = true;

                    continue
                }

                let (op, prec) = operand(&self.traveler.current_content()).unwrap();

                if prec > op_stack.last().unwrap().1 {
                    let left = ex_stack.pop().unwrap();
                    let right = ex_stack.pop().unwrap();

                    ex_stack.push(Expression::Operation(Box::new(left),
                                                        op_stack.pop().unwrap().0,
                                                        Box::new(right)));

                    self.traveler.next();

                    ex_stack.push(self.atom());
                    op_stack.push((op, prec));

                    continue
                }

                self.traveler.next();

                ex_stack.push(self.atom());
                op_stack.push((op, prec));
            }

            let left = ex_stack.pop().unwrap();
            let right = ex_stack.pop().unwrap();

            ex_stack.push(Expression::Operation(Box::new(left),
                                                op_stack.pop().unwrap().0,
                                                Box::new(right)));
        }

        ex_stack.pop().unwrap()
    }
}