use std::collections::HashMap;
use ast::{Type};

fn add(args:&[Type]) -> Type {
    Type::Number(args.into_iter().map(|x| {
        if let Type::Number(n) = x {
            n
        } else { panic!()}
    }).fold(0, |acc, x| acc + x))
}

fn mult(args:&[Type]) -> Type {
    Type::Number(args.into_iter().map(|x| {
        if let Type::Number(n) = x {
            n
        } else { panic!()}
    }).fold(1, |acc, x| acc * x))
}

pub fn init_context() -> HashMap<String, Type> {
    let mut hm: HashMap<String, Type> = HashMap::new();
    hm.insert(String::from("*"), Type::Function(mult));
    hm.insert(String::from("+"), Type::Function(add));
    hm
}