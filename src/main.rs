extern crate regex;
extern crate core;

mod lexer;
mod lisp;
mod parser;
mod built_in;

fn main() {
    let mut context = built_in::init_context();
    lisp::eval_in_context(&String::from("(def a 1)"), &mut context);
    let r = lisp::eval_in_context(&String::from("(+ a 3)"), &mut context);
    println!("{:?}", r);
}
