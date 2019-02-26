use crate::lexer::{Tokens};
use crate::lisp::{Type};

pub fn build(tokens:&Vec<Tokens>, i: usize) -> (Vec<Type>, usize) {
    let mut elems:Vec<Type> = vec![];
    let mut current_pointer = i;
    loop {
        let current_token_ = tokens.get(current_pointer);
        match current_token_ {
            Some(current_token) => {
                match current_token {
                    Tokens::Symbol(s) => {
                        elems.push(Type::Symbol(s.clone()))
                    },
                    Tokens::Number(n) => {
                        elems.push(Type::Number(n.clone()));
                    },
                    Tokens::OP => {
                        let (node, p) = build(&tokens, current_pointer + 1);
                        elems.push(node.get(0).unwrap().clone());
                        current_pointer = p;
                    },
                    Tokens::CP => {
                        let node = Type::List(elems);
                        return (vec![node], current_pointer);
                    }
                }
            },
            None => return (elems, current_pointer)
        }
        current_pointer += 1;
    }
}