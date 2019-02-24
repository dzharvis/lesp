use std::collections::HashMap;
use lisp::{Type, Context, Eval};
use std::rc::Rc;
use lisp::Function;

fn add(mut context: &mut Context, args:&[Type]) -> Type {
    Type::Number(args.into_iter().map(|x| {
        if let Type::Number(n) = x.eval(&mut context) {
            n
        } else { panic!()}
    }).fold(0, |acc, x| acc + x))
}

fn sub(mut context: &mut Context, args:&[Type]) -> Type {
    let first = if let Type::Number(n) = args.get(0).unwrap().eval(&mut context) {
        n
    } else {panic!()};
    Type::Number(args[1..].into_iter().map(|x| {
        if let Type::Number(n) = x.eval(&mut context) {
            n
        } else { panic!()}
    }).fold(first, |acc, x| {
        acc - x
    }))
}

fn mult(mut context: &mut Context, args:&[Type]) -> Type {
    Type::Number(args.into_iter().map(|x| {
        if let Type::Number(n) = x.eval(&mut context) {
            n
        } else { panic!()}
    }).fold(1, |acc, x| acc * x))
}

/**
(def a (+ 1 2 ))
*/
fn def_special(mut context: &mut Context, args:&[Type]) -> Type {
    let name = if let Type::Symbol(s) = args.get(0).unwrap() {
        s
    } else { panic!()};
    let value = args.get(1).unwrap().eval(&mut context);
    context.insert(name.clone(), value.clone());
    value
}

/**
(let ((a (+ 1 2))
      (b (* 1 2)))
  (+ a b))
*/
fn let_special(context: &mut Context, args:&[Type]) -> Type {
    let bindings = args.get(0).unwrap();
    let mut new_context = context.clone();
    if let Type::List(elems) = bindings {
        for x in elems {
            if let Type::List(binding) = x {
                let name = if let Type::Symbol(n) = binding.get(0).unwrap() {
                    n
                } else { panic!()};
                let value= binding.get(1).unwrap().eval(&mut new_context);
                new_context.insert(name.clone(), value.clone());
            }
        }
    } else { panic!()}
    let len = args.len() - 1;
    for form in &args[1..len] {
        form.eval(&mut new_context);
    }
    args.get(args.len() - 1).unwrap().eval(&mut new_context)
}

/**
(fn name (a b c)
    (+ a b c))
*/
fn fn_special(mut _context: &mut Context, args:& [Type]) -> Type {

    let name = if let Type::Symbol(name) = args.get(0).unwrap() {
        name
    } else { panic!()};
    let argument_bindings = if let Type::List(names) = args.get(1).unwrap() {
        names.clone()
    } else { panic!() };
    let body = args.get(2).unwrap().clone();

    let closure: Rc<Function> = Rc::new(move |context: &mut Context, args:&[Type]| {
        let mut context = context.clone();
        assert_eq!(args.len(), argument_bindings.len());
        for i in 0..args.len() {
            let name = if let Type::Symbol(name) = argument_bindings.get(i).unwrap() {
                name
            } else { panic!() };
            let x = args.get(i).unwrap().eval(&mut context);
            context.insert(name.clone(), x);
        }
        return body.eval(&mut context);
    });
    Type::Function(name.clone(), closure)
}

/**
(if form
    then
    else)
*/
fn if_special(mut context: &mut Context, args:& [Type]) -> Type {
    let test = args.get(0).unwrap().eval(&mut context);
    if let Type::Bool(t) = test {
        if t {
            args.get(1).unwrap().eval(&mut context)
        } else {
            args.get(2).unwrap().eval(&mut context)
        }
    } else { panic!() }
}

/**
(> 10 20)
-> false
*/
fn gt(mut context: &mut Context, args:& [Type]) -> Type {
    let left = args.get(0).unwrap().eval(&mut context);
    let right = args.get(1).unwrap().eval(&mut context);

    if let (Type::Number(l), Type::Number(r)) = (left, right) {
        Type::Bool(l > r)
    } else {
        panic!()
    }
}

fn add_to_context(name: &str, context: &mut Context, value: Rc<Function>) {
    let name = String::from(name);
    context.insert(name.clone(), Type::Function(name, value));
}

pub fn init_context() -> Context {
    let mut context: HashMap<String, Type> = HashMap::new();
    add_to_context("*", &mut context,Rc::new(mult));
    add_to_context("+", &mut context,Rc::new(add));
    add_to_context("-", &mut context,Rc::new(sub));
    add_to_context("def", &mut context,Rc::new(def_special));
    add_to_context("let", &mut context,Rc::new(let_special));
    add_to_context("fn", &mut context,Rc::new(fn_special));
    add_to_context("if", &mut context,Rc::new(if_special));
    add_to_context(">", &mut context,Rc::new(gt));
    return context;
}