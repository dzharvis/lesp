use std::collections::HashMap;
use crate::lisp::{Type, Context, Eval};
use std::rc::Rc;
use crate::lisp::Function;

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

fn car(mut context: &mut Context, args:&[Type]) -> Type {
    if args.len() == 0 {
        return Type::List(vec![]);
    }
    if let Type::List(elems) = args.get(0).unwrap().eval(&mut context) {
        if elems.len() == 0 {
            return Type::List(vec![]);
        }
        elems.get(0).unwrap().clone()
    } else { panic!() }
}

fn cdr(mut context: &mut Context, args:&[Type]) -> Type {
    if args.len() == 0 {
        return Type::List(vec![]);
    }
    if let Type::List(elems) = args.get(0).unwrap().eval(&mut context) {
        if elems.len() == 0 {
            return Type::List(vec![]);
        }
        Type::List(elems[1..].to_vec())
    } else { panic!() }
}

fn cons(mut context: &mut Context, args:&[Type]) -> Type {
    let first = args.get(0).unwrap().eval(&mut context);
    if let Type::List(elems) = args.get(1).unwrap().eval(&mut context) {
        let mut new_list = vec![first];
        new_list.extend(elems);
        Type::List(new_list)
    } else { panic!() }
}

/**
 * (quote (a 2 3))
 * -> (a 2 3)
 * (quote 1)
 * -> 1
 */
fn quote(_context: &mut Context, args:&[Type]) -> Type {
    args.get(0).unwrap().clone()
}

/**
 * (list a 2 3)
 * -> (a 2 3)
 * (list 1 (+ 0 1))
 * -> (1 2)
 */
fn list(mut context: &mut Context, args:&[Type]) -> Type {
    let elems = args.into_iter().map(|x| x.eval(&mut context)).collect();
    Type::List(elems)
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
                new_context.insert(name.clone(), value);
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
fn fn_special(context: &mut Context, args:& [Type]) -> Type {
    let name = if let Type::Symbol(name) = args.get(0).unwrap() {
        name
    } else { panic!()};
    let argument_bindings = if let Type::List(names) = args.get(1).unwrap() {
        names.clone()
    } else { panic!() };
    let body = args.get(2).unwrap().clone();
    let captured_context = context.clone(); // clone because of lifetime inside closure
    let closure_name = name.clone();

    let closure: Rc<Function> = Rc::new(move |mut lexical_context: &mut Context, args:&[Type]| {
        let mut captured_context = captured_context.clone(); // rust forces either mutex or clone for thread safety
        assert_eq!(args.len(), argument_bindings.len(), "argument size mismatch {:?} -> {:?}", &args, &argument_bindings);
        for i in 0..args.len() {
            let name = if let Type::Symbol(name) = argument_bindings.get(i).unwrap() {
                name
            } else { panic!() };
            let arg = args.get(i).unwrap().eval(&mut lexical_context); //eval function args first with current lexical scope
            captured_context.insert(name.clone(), arg);
        }

        // TODO find better solution for this hack
        // hack for named lambdas - get current closure from lexical scope if exists
        match lexical_context.get(&closure_name) {
            Some(c) => {
                // force add current closure to captured scope
                captured_context.insert(closure_name.clone(), c.clone());
                ()
            },
            None => () //ignore
        };
        return body.eval(&mut captured_context); // eval body with enclosed scope
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

fn eq(mut context: &mut Context, args:& [Type]) -> Type {
    let left = args.get(0).unwrap().eval(&mut context);
    let right = args.get(1).unwrap().eval(&mut context);

    Type::Bool(left.eq(&right))
}

fn and(mut context: &mut Context, args:& [Type]) -> Type {
    let left = args.get(0).unwrap().eval(&mut context);
    let right = args.get(1).unwrap().eval(&mut context);
    match (left, right) {
        (Type::Bool(true), Type::Bool(true)) => Type::Bool(true),
        (_,_) => Type::Bool(false)
    }
}

fn or(mut context: &mut Context, args:& [Type]) -> Type {
    // no short circuit - i'm too lazy
    let left = args.get(0).unwrap().eval(&mut context);
    let right = args.get(1).unwrap().eval(&mut context);
    match (left, right) {
        (Type::Bool(left), Type::Bool(right)) => Type::Bool(left || right),
        (_,_) => Type::Bool(false)
    }
}

fn not(mut context: &mut Context, args:& [Type]) -> Type {
    let arg = args.get(0).unwrap().eval(&mut context);
    match arg {
        Type::Bool(arg) => Type::Bool(!arg),
        _ => panic!()
    }
}

// Almost the same as function but evals twice and uses lexical context.
// Find a way to remove code duplication
fn macro_scpecial(_context: &mut Context, args:& [Type]) -> Type {
    let name = if let Type::Symbol(name) = args.get(0).unwrap() {
        name
    } else { panic!()};
    let argument_bindings = if let Type::List(names) = args.get(1).unwrap() {
        names.clone()
    } else { panic!() };
    let body = args.get(2).unwrap().clone();
    let closure_name = name.clone();

    let closure: Rc<Function> = Rc::new(move |mut lexical_context: &mut Context, args:&[Type]| {
        // let mut captured_context = captured_context.clone(); // rust forces either mutex or clone for thread safety
        assert_eq!(args.len(), argument_bindings.len(), "argument size mismatch {:?} -> {:?}", &args, &argument_bindings);
        for i in 0..args.len() {
            let name = if let Type::Symbol(name) = argument_bindings.get(i).unwrap() {
                name
            } else { panic!() };
            let arg = args.get(i).unwrap().clone(); // don't eval args for macro
            lexical_context.insert(name.clone(), arg);
        }

        // TODO find better solution for this hack
        match lexical_context.get(&closure_name) {
            Some(c) => {
                // force add current closure to captured scope
                lexical_context.insert(closure_name.clone(), c.clone());
                ()
            },
            None => () //ignore
        };
        return body.eval(&mut lexical_context).eval(&mut lexical_context);
    });
    Type::Function(name.clone(), closure)
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
    add_to_context("quote", &mut context,Rc::new(quote));
    add_to_context("list", &mut context,Rc::new(list));
    add_to_context("car", &mut context,Rc::new(car));
    add_to_context("cdr", &mut context,Rc::new(cdr));
    add_to_context("cons", &mut context,Rc::new(cons));
    add_to_context("eq", &mut context,Rc::new(eq));
    add_to_context("and", &mut context,Rc::new(and));
    add_to_context("or", &mut context,Rc::new(or));
    add_to_context("not", &mut context,Rc::new(not));
    add_to_context("macro", &mut context,Rc::new(macro_scpecial));
    return context;
}