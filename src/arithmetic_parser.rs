use crate::arithmetic_parser::models::Expression;

pub mod models;

pub struct Interpreter {
    source_code: String,
    pub syntax_tree: Option<Box<Expression>>,
    pos: i32,
    ch: i32
}

impl Interpreter {
    pub fn new(source_code: String) -> Self {
        let interpreter = Interpreter {
            source_code,
            pos: -1,
            syntax_tree: None,
            ch: -1
        };


        return interpreter;
    }

    pub fn evaluate(&mut self) -> f64 {
        if self.source_code.is_empty() {
            return 0.0;
        }

        return self.parse().evaluate();
    }

    fn next_char(&mut self) {
        self.pos += 1;
        if self.pos < self.source_code.len() as i32 {
            self.ch = self.source_code.as_bytes()[self.pos as usize] as i32;
        } else {
            self.ch = -1;
        }
    }

    fn eat(&mut self, char_to_eat: char) -> bool {
        while self.ch == ' ' as i32 {
            self.next_char();
        }

        if self.ch == char_to_eat as i32 {
            self.next_char();
            return true;
        }


        return false;
}

    fn parse(&mut self) -> &Box<Expression> {
        self.source_code = self.source_code.replace('.', ",");
        self.next_char();
        self.syntax_tree = self.parse_expression();

        if self.pos < self.source_code.len() as i32 {
            panic!("Unexpected: \"{}\"", self.ch);
        }

        return self.syntax_tree.as_ref().unwrap();
    }

    fn parse_expression(&mut self) -> Option<Box<Expression>> {
        let mut x: Box<Expression> = self.parse_term().unwrap();
        loop {
            if self.eat('+') { x = x.add(self.parse_term()).unwrap(); }
            else if self.eat('-') { x = x.sub(self.parse_term()).unwrap(); }
            else { return Some(x); }
        }
    }

    fn parse_term(&mut self) -> Option<Box<Expression>> {
        let mut x: Box<Expression> = self.parse_factor().unwrap();
        loop {
            if self.eat('*') { x = x.mul(self.parse_term()).unwrap(); }
            else if self.eat('/') { x = x.div(self.parse_term()).unwrap(); }
            else { return Some(x); }
        }
    }

    fn parse_factor(&mut self) -> Option<Box<Expression>> {
        let mut x: Option<Box<Expression>>;
        
        if self.eat('+') {
            x = self.parse_factor();
            return x;
        } else if self.eat('-') {
            x = self.parse_factor();
            x.as_mut().unwrap().flip_value();
            return x;
        }
        
        let start_pos: i32 = self.pos;
        if self.eat('(') {
            x = self.parse_expression();
            
            if !self.eat(')') {
                panic!("Expected ')'");
            }
            
        } else if self.ch >= '0' as i32 && self.ch <= '9' as i32 || self.ch == '.' as i32 || self.ch == ',' as i32 {
            while self.ch >= '0' as i32 && self.ch <= '9' as i32 || self.ch == ',' as i32 || self.ch == '.' as i32 { 
                self.next_char() 
            }
            
            let sub_string:&str = &self.source_code[start_pos as usize..self.pos as usize];
            let value: f64 = sub_string.parse::<f64>().unwrap();
            x = Some(Box::new(Expression::new_f64(value)));
        } else if self.ch >= 'a' as i32 && self.ch <= 'z' as i32 {
            while self.ch >= 'a' as i32 && self.ch <= 'z' as i32 {
                self.next_char();
            }

            let func: String = self.source_code[start_pos as usize..self.pos as usize].to_string();
            x = self.parse_factor();

            x = match func.as_str() {
                "sqrt" => {
                    let result = x.as_ref().unwrap().evaluate().sqrt();
                    Some(Box::new(Expression::new_func("Sqrt".to_string(), x, result)))
                },
                "sin" => {
                    let result = x.as_ref().unwrap().evaluate().sin();
                    Some(Box::new(Expression::new_func("Sin".to_string(), x, result)))
                },
                "cos" => {
                    let result = x.as_ref().unwrap().evaluate().cos();
                    Some(Box::new(Expression::new_func("Cos".to_string(), x, result)))
                },
                "tan" => {
                    let result = x.as_ref().unwrap().evaluate().tan();
                    Some(Box::new(Expression::new_func("Tan".to_string(), x, result)))
                },
                _ => panic!("Unknown function: {}", func)
            }
        } else {
            panic!("Unexpected: \"{}\"", self.ch);
        }

        if self.eat('^') {
            x = x.unwrap().pow(self.parse_factor());
        }

        return x;
    }
}
