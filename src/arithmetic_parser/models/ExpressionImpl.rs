use crate::arithmetic_parser::models::{Expression, Operator};

impl Default for Expression {
    fn default() -> Self {
        Expression {
            lhs: None,
            rhs: None,
            operator: Operator::NOOP,
            value: None,
            func: String::new()
        }
    }
}

impl Expression {
    pub fn flip_value(&mut self) -> () {
        let v: &mut f64 = &mut (0.0_f64 / 0.0_f64);
        *self.value.as_mut().unwrap_or(v) *= -1.0;
    }

    pub fn new_f64(value: f64) -> Self {
        Expression {
            value: Some(value),
            ..Default::default()
        }
    }

    pub fn new_func(func: String, lhs: Option<Box<Expression>>, value: f64) -> Self {
        Expression {
            lhs,
            value: Some(value),
            func,
            ..Default::default()
        }
    }

    pub fn new(lhs: Option<Box<Expression>>, operation: Operator, rhs: Option<Box<Expression>>, value: f64) -> Self {
        Expression {
            lhs,
            rhs,
            operator: operation,
            value: Some(value),
            func: String::new()
        }
    }

    pub fn evaluate(&self) -> f64 {
        return self.value.unwrap();
    }

    pub fn add(&mut self, other: Option<Box<Expression>>) -> Option<Box<Expression>> {
        let boxed_self: Option<Box<Expression>> = Some(Box::new(self.clone()));
        let other_result = other.as_ref().unwrap().evaluate();

        let ex: Expression = Expression::new(
            boxed_self,
            Operator::ADD,
            other,
            self.value.unwrap_or(f64::NAN) + other_result
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
            self.value.unwrap_or(f64::NAN) - other_result
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
            self.value.unwrap_or(f64::NAN) * other_result
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
            self.value.unwrap_or(f64::NAN) / other_value
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
            self.value.unwrap_or(f64::NAN).powf(other_result)
        );

        return Some(Box::new(ex));
    }

    pub fn tree_view(&self, mut indent: String, last: bool) -> String {
        let mut string_builder: String = String::new();
        string_builder += &indent;

        if last {
            string_builder += "└─";
            indent += "  ";
        }
        else {
            string_builder += "├─";
            indent += "| ";
        }

        let inner: String = self.to_inner_string();
        string_builder += inner.as_str();
        string_builder += "\n";

        let mut children = Vec::new();

        if self.lhs.is_some() { children.push(self.lhs.as_ref().unwrap()); }
        if self.rhs.is_some() { children.push(self.rhs.as_ref().unwrap()); }

        let child_len = children.len();

        for i in 0..children.len() {
            string_builder += children[i].tree_view(indent.clone(), i == child_len - 1).as_str();
        }

        return string_builder;
    }


    pub fn to_inner_string(&self) -> String {
        if !self.func.is_empty() {
            return self.value.unwrap().to_string() + ", " + self.func.to_uppercase().as_str();
        }

        if self.value.is_some() && self.lhs.is_none() && self.rhs.is_none() {
            self.value.unwrap().to_string();
        }

        if self.operator != Operator::NOOP && self.lhs.is_none() && self.rhs.is_none() {
            return self.operator.to_string();
        }

        let list = vec![self.value.unwrap().to_string(), self.operator.to_string()];

        return if list[1] == "" {
            list.join("")
        } else {
            list.join(", ")
        };
    }
}
