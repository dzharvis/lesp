extern crate regex;
extern crate core;

mod lexer;
mod lisp;
mod builder;
mod built_in;

use lisp::Eval;

fn main() {
    println!("result -> {:?}", lisp::eval(&String::from("(+ 1 2)")));
}
