use lexer::{Tokens};
use lisp::{Type};

pub fn build(tokens:&Vec<Tokens>, i: usize) -> (Type, usize) {
    let mut elems:Vec<Type> = vec![];
    let mut current_pointer = i + 1;
    loop {
        let current_token = tokens.get(current_pointer).unwrap();
        match current_token {
            Tokens::Symbol(s) => {
                elems.push(Type::Symbol(s.clone()))
            },
            Tokens::Number(n) => {
                elems.push(Type::Number(n.clone()));
            },
            Tokens::OP => {
                let (node, p) = build(&tokens, current_pointer);
                elems.push(node);
                current_pointer = p;
            },
            Tokens::CP => {
                let node = Type::List(elems);
                return (node, current_pointer);
            }
        }
        current_pointer += 1;
    }
}