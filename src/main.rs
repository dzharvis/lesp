extern crate lazy_static;
extern crate regex;
extern crate core;

mod lexer;
mod lisp;
mod builder;
mod built_in;

use lisp::Eval;

fn main() {
//    let input = String::from("(let ((add (fn add () (fn a (a b) (+ a b))))\
//                                               (apply (fn apply (f a) (f a)))
//                                                (fib (fn fib (n) \
//                                                (if (> 3 n)\
//                                                    (- n 1)\
//                                                    (+ (fib (- n 1)) \
//                                                       (fib (- n 2)))))))\
//                                          (fib 6)\
//                                          ((add) 2 4)\
//                                          (apply (fn _ (a) (* a 10) ) 123))");

    let input = String::from("(let ((apply (fn apply (f a) (f a))))\
                                          (apply (fn _ (a) (* a a) ) ((fn _ (a) (+ a 5)) 5)))");
//    println!("input -> {}", input);

    let res = lexer::parse_fsm(&input);
//    println!("lexer -> {:?}", res);

    let (n, _) = builder::build(&res, 0);
    println!("tree -> {:?}", n);

    let mut context = built_in::init_context();
    let result = n.eval(&mut context);
    println!("result -> {:?}", result);
}
