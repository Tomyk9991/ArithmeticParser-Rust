use arithmetic_parser::Interpreter;
mod arithmetic_parser;

fn main() {
    let target: String = "((4 - 2^3 + 1) * -sqrt(3*3+4*4)) / 2".to_string();

    let mut interpreter: Interpreter = Interpreter::new(target.clone().to_string());
    println!("{} = {}", target ,interpreter.evaluate());
    println!("{}", interpreter.syntax_tree.unwrap().tree_view("".to_string(), true));
}
