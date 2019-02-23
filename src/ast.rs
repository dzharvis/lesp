use lexer::{Tokens};
use std::fmt::Debug;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Type {
    Number(u32), Function(Apply)
}

pub trait Node : Debug {
    fn eval(&self, context: &Context) -> Type;
}

#[derive(Debug)]
struct EvalNode {
    args: Vec<Box<Node>>
}

impl Node for EvalNode {
    fn eval(&self, context: &Context) -> Type {
        // always expected first element to be some symbol
        let symb = self.args.get(0).unwrap();
        // lookup function for first symbol
        if let Type::Function(func) = symb.eval(context) {
            // eval every argument
            let x:Vec<Type> = self.args[1..].iter().map(|x| x.eval(&context)).collect();
            // call function on ints args
            let result = func(&x);
            return result;
        } else {
            panic!();
        }
    }
}

#[derive(Debug)]
struct SymbolNode {
    name: String
}

impl Node for SymbolNode {
    fn eval(&self, context: &Context) -> Type {
        let x = context.get(self.name.as_str()).unwrap();
        x.clone()
    }
}

#[derive(Debug)]
struct NumberNode {
    number:u32
}

impl Node for NumberNode {
    fn eval(&self, _: &Context) -> Type {
        Type::Number(self.number)
    }
}

type Apply = fn(&[Type]) -> Type;
type Context = HashMap<String, Type>;


/**
(let ((a (+ 1 2))
      (b (* 1 2)))
  (+ a b))
*/

pub fn consume_let(tokens:&Vec<Tokens>, i:usize) -> (Box<Node>, usize) {

}

pub fn consume(tokens:&Vec<Tokens>, i: usize) -> (Box<Node>, usize) {
    let mut args:Vec<Box<Node>> = vec![];
    let mut current_pointer = i + 1;
    loop {
        let current_token = tokens.get(current_pointer).unwrap();
        match current_token {
            Tokens::Symbol(s) => {
                match s.as_str() {
                    "let" => {
                        let (node, p) = consume_let(&tokens, current_pointer);
                        args.push(node);
                        current_pointer = p;
                    },
                    _ => args.push(Box::new(SymbolNode{name: s.clone()}))
                }
            },
            Tokens::Number(n) => {
                args.push(Box::new(NumberNode{number: n.clone()}));
            },
            Tokens::OP => {
                let (node, p) = consume(&tokens, current_pointer);
                args.push(node);
                current_pointer = p;
            },
            Tokens::CP => {
                let node = EvalNode{args};
                return (Box::new(node), current_pointer);
            }
        }
        current_pointer += 1;
    }
}