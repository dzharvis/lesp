extern crate lazy_static;
extern crate regex;
extern crate core;

mod lexer;
mod lisp;
mod builder;
mod built_in;

use lisp::Eval;

fn main() {
    let input = String::from("(let ((add (fn add (aa bb) (+ aa bb)))\
                                              (a 1)\
                                              (b (+ 10 20))) \
                                          (* 0 0) \
                                          (add a a))");
    println!("input -> {}", input);

    let res = lexer::parse_fsm(&input);
    println!("lexer -> {:?}", res);

    let (n, _) = builder::build(&res, 0);
    println!("tree -> {:?}", n);

    let mut context = built_in::init_context();
    let result = n.eval(&mut context);
    if let lisp::Type::Number(n) = result {
        println!("result -> {:?}", n);
    } else {
        println!("error ");
    }
}
