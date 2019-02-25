extern crate regex;
extern crate core;

mod lexer;
mod lisp;
mod parser;
mod built_in;

fn main() {
    println!("result -> {:?}", lisp::eval(&String::from("(+ 1 2)")));
}
