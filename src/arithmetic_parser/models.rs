mod ExpressionImpl;
mod OperatorImpl;

#[derive(Debug, Clone)]
pub struct Expression {
    lhs: Option<Box<Expression>>,
    rhs: Option<Box<Expression>>,
    operator: Operator,
    pub value: Option<f64>,
    func: String
}

#[derive(PartialEq, Clone, Debug)]
pub enum Operator {
    NOOP,
    ADD,
    SUB,
    DIV,
    MUL,
    POW,
}