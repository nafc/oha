#[derive(Debug)]
pub enum Expression {
    IntLiteral(i32),
    FloatLiteral(f32),
    StringLiteral(String),
    BoolLiteral(bool),
    Identifier(String),
}

#[derive(Debug)]
pub enum Statement {
    Block(Box<Vec<Statement>>),
    Expression(Box<Expression>),
}
