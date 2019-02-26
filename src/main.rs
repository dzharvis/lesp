extern crate regex;
extern crate core;

mod lexer;
mod lisp;
mod parser;
mod built_in;

use std::fs;
use std::io::{self, BufRead, Write};

fn main() {
    let mut context = built_in::init_context();

    init_context_from_file(&mut context);

    let stdin = io::stdin();
    print(">> ");
    
    for line in stdin.lock().lines() {
        let r = lisp::eval_in_context(&String::from(line.unwrap()), &mut context);
        println!("<< {:?}", r);
        print(">> ");
    }
}

fn print(s: &str) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write(&s.as_bytes());
    handle.flush();
}

fn init_context_from_file(mut context: &mut lisp::Context) {
    let contents = fs::read_to_string("C:\\Users\\dzharvis\\projects\\lesp\\res\\init.lisp")
        .expect("Something went wrong reading the file");
    lisp::eval_in_context(&String::from(contents), &mut context);
}
