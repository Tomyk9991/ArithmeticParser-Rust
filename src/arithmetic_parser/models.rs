extern crate string_builder;
use std::{fmt};
use std::borrow::{Borrow, BorrowMut};

use string_builder::Builder;


#[derive(Debug, Clone)]
pub struct Expression {
    LHS: Option<Box<Expression>>,
    RHS: Option<Box<Expression>>,
    Operator: Operator,
    pub Value: Option<f64>,
    Func: String
}

impl Default for Expression {
    fn default() -> Self {
        Expression {
            LHS: None,
            RHS: None,
            Operator: Operator::NOOP,
            Value: None,
            Func: String::new()
        }
    }
}

impl Expression {
    pub fn new_e(e: Expression) -> Self {
        Expression {
            Value: Some(e.evaluate()),
            ..Default::default()
        }
    }
    
    pub fn flip_value(&mut self) -> () {
        let v: &mut f64 = &mut f64::NAN;
        *self.Value.as_mut().unwrap_or(v) *= -1.0;
    }

    pub fn new_f64(value: f64) -> Self {
        Expression {
            Value: Some(value),
            ..Default::default()
        }
    }

    pub fn new_func(func: String, lhs: Option<Box<Expression>>, value: f64) -> Self {
        Expression { 
            LHS: lhs, 
            Value: Some(value),
            Func: func,
            ..Default::default()
        }
    }

    pub fn new(lhs: Option<Box<Expression>>, operation: Operator, rhs: Option<Box<Expression>>, value: f64) -> Self {
        Expression { 
            LHS: lhs, 
            RHS: rhs, 
            Operator: operation, 
            Value: Some(value),
            Func: String::new()
        }
    } 

    pub fn defined(&self) -> bool {
        self.Value.is_some() && 
        self.Operator != Operator::NOOP &&
        self.LHS.is_some() && self.RHS.is_some()
    }

    pub fn evaluate(&self) -> f64 {
        return self.Value.unwrap();
    }

    pub fn add(&mut self, other: Option<Box<Expression>>) -> Option<Box<Expression>> {
        let boxed_self: Option<Box<Expression>> = Some(Box::new(self.clone()));
        let other_result = other.as_ref().unwrap().evaluate();
        
        let ex: Expression = Expression::new(
            boxed_self, 
            Operator::ADD, 
            other, 
            self.Value.unwrap_or(f64::NAN) + other_result
        );

        return Some(Box::new(ex));
    }

    pub fn sub(&self, other: Option<Box<Expression>>) -> Option<Box<Expression>> {
        let boxed_self: Option<Box<Expression>> = Some(Box::new(self.clone()));
        let other_result = other.as_ref().unwrap().evaluate();
        let ex: Expression = Expression::new(
            boxed_self, 
            Operator::SUB, 
            other, 
            self.Value.unwrap_or(f64::NAN) - other_result
        );

        return Some(Box::new(ex));
    }

    pub fn mul(&self, other: Option<Box<Expression>>) -> Option<Box<Expression>> {
        let boxed_self: Option<Box<Expression>> = Option::Some(Box::new(self.clone()));
        let other_result = other.as_ref().unwrap().evaluate();
        let ex: Expression = Expression::new(
            boxed_self, 
            Operator::MUL, 
            other, 
            self.Value.unwrap_or(f64::NAN) * other_result
        );

        return Some(Box::new(ex));
    }

    pub fn div(&self, other: Option<Box<Expression>>) -> Option<Box<Expression>> {
        let boxed_self: Option<Box<Expression>> = Option::Some(Box::new(self.clone()));
        let other_value: f64 = other.as_ref().unwrap().evaluate();
        
        if other_value == 0.0 {
            panic!("Division by zero");
        }


        let ex: Expression = Expression::new(
            boxed_self, 
            Operator::DIV, 
            other, 
            self.Value.unwrap_or(f64::NAN) / other_value
        );

        return Option::Some(Box::new(ex));
    }

    pub fn pow(&self, other: Option<Box<Expression>>) -> Option<Box<Expression>> {
        let boxed_self: Option<Box<Expression>> = Option::Some(Box::new(self.clone()));
        let other_result = other.as_ref().unwrap().evaluate();
        let ex: Expression = Expression::new(
            boxed_self, 
            Operator::POW, 
            other,
            self.Value.unwrap_or(f64::NAN).powf(other_result)
        );

        return Some(Box::new(ex));
    }

    pub fn tree_view(&self, mut indent: String, last: bool) -> String {
        let mut string_builder = Builder::default();
        if last {
            string_builder.append("└─");
            indent += "  ";
        }
        else {
            string_builder.append("├─");
            indent += "| ";
        }

        let inner: String = self.to_inner_string();
        string_builder.append(inner);
        string_builder.append('\n');
        
        let mut children = Vec::new();
        
        if self.LHS.is_some() { children.push(self.LHS.as_ref().unwrap()); }
        if self.RHS.is_some() { children.push(self.RHS.as_ref().unwrap()); }

        let child_len = children.len();

        for i in 0..children.len() {
            string_builder.append(children[i].tree_view(indent.clone(), i == child_len - 1));
        }

        return string_builder.string().unwrap();
    }

    pub fn to_string(&self) -> String {
        let lhs: String = self.LHS.as_ref().map_or_else(
            || "NULL".to_owned(), 
            |lhs| lhs.to_string()
        );

        let rhs: String = self.RHS.as_ref().map_or_else(
            || "NULL".to_owned(), 
            |rhs| rhs.to_string()
        );

        if self.Value.is_some() && lhs == "NULL" && rhs == "NULL" {
            return self.Value.unwrap().to_string();
        }

        if self.Operator != Operator::NOOP && lhs == "NULL" && rhs == "NULL" {
            return self.Operator.to_string();
        }

        let list = vec![lhs, self.Operator.to_string(), rhs];
        return list.join(" ");
    }

    pub fn to_inner_string(&self) -> String {
        if !self.Func.is_empty() {
            return self.Value.unwrap().to_string() + ", " + self.Func.to_uppercase().as_str();
        }

        if self.Value.is_some() && self.LHS.is_none() && self.RHS.is_none() {
            self.Value.unwrap().to_string();
        }

        if self.Operator != Operator::NOOP && self.LHS.is_none() && self.RHS.is_none() {
            return self.Operator.to_string();
        }

        let list = vec![self.Value.unwrap().to_string(), self.Operator.to_string()];
        return list.join(", ");
    }

}

#[derive(PartialEq, Clone, Debug)]
pub enum Operator {
    ADD,
    SUB,
    DIV,
    MUL,
    POW,
    NOOP
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}