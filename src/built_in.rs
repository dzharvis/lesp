use std::collections::HashMap;
use lisp::{Type, Context, Eval};
use std::rc::Rc;
use lisp::Applyable;

fn add(mut context: &mut Context, args:&[Type]) -> Type {
    Type::Number(args.into_iter().map(|x| {
        if let Type::Number(n) = x.eval(&mut context) {
            n
        } else { panic!()}
    }).fold(0, |acc, x| acc + x))
}

fn mult(mut context: &mut Context, args:&[Type]) -> Type {
    Type::Number(args.into_iter().map(|x| {
        if let Type::Number(n) = x.eval(&mut context) {
            n
        } else { panic!()}
    }).fold(1, |acc, x| acc * x))
}

fn def(mut context: &mut Context, args:&[Type]) -> Type {
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
fn let_special(mut context: &mut Context, args:&[Type]) -> Type {
    let bindings = args.get(0).unwrap();
    if let Type::List(elems) = bindings {
        for x in elems {
            if let Type::List(binding) = x {
                let name = if let Type::Symbol(n) = binding.get(0).unwrap() {
                    n
                } else { panic!()};
                let value= binding.get(1).unwrap().eval(&mut context);
                context.insert(name.clone(), value.clone());
            }
        }
    } else { panic!()}
    let len = args.len() - 1;
    for form in &args[1..len] {
        form.eval(&mut context);
    }
    args.get(args.len() - 1).unwrap().eval(&mut context)
}

/**
(fn name (a b c)
    (+ a b c))
*/

fn fn_special(mut context: &mut Context, args:& [Type]) -> Type {
    let name = if let Type::Symbol(name) = args.get(0).unwrap() {
        name
    } else { panic!()};
    let argument_bindings = if let Type::List(names) = args.get(1).unwrap() {
        names.clone()
    } else { panic!() };
    let body = args.get(2).unwrap().clone();

    let closure: Rc<Applyable> = Rc::new(move |mut context: &mut Context, args:&[Type]| {
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

pub fn init_context() -> Context {
    let mut hm: HashMap<String, Type> = HashMap::new();
    let mult_str = String::from("*");
    hm.insert(mult_str.clone(), Type::Function(mult_str, Rc::new(mult)));
    let add_str = String::from("+");
    hm.insert(add_str.clone(), Type::Function(add_str, Rc::new(add)));
    let def_str = String::from("def");
    hm.insert(def_str.clone(), Type::Function(def_str, Rc::new(def)));
    let let_str = String::from("let");
    hm.insert(let_str.clone(), Type::Function(let_str, Rc::new(let_special)));
    let fn_str = String::from("fn");
    hm.insert(fn_str.clone(), Type::Function(fn_str, Rc::new(fn_special)));
    return hm;
}