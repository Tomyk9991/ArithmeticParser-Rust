use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum Operator {
    NOOP,
    ADD,
    SUB,
    DIV,
    MUL,
    POW,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == Operator::NOOP {
            return write!(f, "");
        }
        write!(f, "{:?}", self)
    }
}