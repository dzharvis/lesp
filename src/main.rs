extern crate lazy_static;
extern crate regex;
extern crate core;

mod lexer;
mod ast;
mod built_in;

fn main() {
    let input = String::from("(* (* 1 2 (+ 3 4 (+ 0 0))) 1 )");
    println!("input -> {}", input);

    let res = lexer::parse_fsm(&input);
    println!("lexer -> {:?}", res);

    let (n, _) = ast::consume(&res, 0);
    println!("tree -> {:?}", n);

    let context = built_in::init_context();
    let result = n.eval(&context);
    if let ast::Type::Number(n) = result {
        println!("result -> {:?}", n);
    } else {
        println!("error ");
    }
}
