use std::fmt;
use crate::arithmetic_parser::models::Operator;

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == Operator::NOOP {
            return write!(f, "");
        }
        write!(f, "{:?}", self)
    }
}