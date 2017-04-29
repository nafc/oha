#[derive(Debug)]
pub enum Expression {
    IntLiteral(i32),
    FloatLiteral(f32),
    StringLiteral(String),
    CharLiteral(char),
    BoolLiteral(bool),
    Identifier(String),
    Operation(Box<Expression>, Operand, Box<Expression>),
}

#[derive(Debug)]
pub enum Statement {
    Block(Box<Vec<Statement>>),
    Assignment(Box<Expression>, Box<Expression>),
    Declaration(Box<Expression>, Option<Box<Expression>>),
    Expression(Box<Expression>),
}

#[derive(Debug)]
pub enum Operand {
    Mul,
    Div,
    Plus,
    Minus,
    Equal,
    Lt,
    LtEqual,
    Gt,
    GtEqual,
}

pub fn operand(v: &str) -> Option<(Operand, u8)> {
    match v {
        "*"  => Some((Operand::Mul, 1)),
        "/"  => Some((Operand::Div, 1)),
        "+"  => Some((Operand::Plus, 2)),
        "-"  => Some((Operand::Minus, 2)),
        "==" => Some((Operand::Equal, 3)),
        "<"  => Some((Operand::Lt, 4)),
        ">"  => Some((Operand::Gt, 4)),
        "<=" => Some((Operand::LtEqual, 4)),
        ">=" => Some((Operand::GtEqual, 4)),
        _ => None,
    }
}