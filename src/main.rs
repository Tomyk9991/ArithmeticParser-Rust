use arithmetic_parser::Interpreter;

mod arithmetic_parser;
fn main() {
    let mut interpreter: Interpreter = Interpreter::new("4 + 5 * 6".to_string());
    println!("4 + 5 * 6 = {}", interpreter.evaluate());
    println!("{}", interpreter.syntax_tree.unwrap().tree_view("".to_string(), true));
}
