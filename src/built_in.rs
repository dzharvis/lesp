use std::collections::HashMap;
use std::rc::Rc;
use crate::lisp::{Type, Context, FunctionType, Function, NativeFunction};

fn add(mut context: &mut Context, args:&[Type]) -> Type {
    Type::Number(args.into_iter().map(|x| {
        if let Type::Number(n) = x.eval(&mut context) {
            n
        } else { panic!()}
    }).fold(0, |acc, x| acc + x))
}

fn is_list(_context: &mut Context, args:&[Type]) -> Type {
    if let Type::List(_l) = args.get(0).unwrap() {
        Type::Bool(true)
    } else {
        Type::Bool(false)
    }
}

fn prn(mut context: &mut Context, args:&[Type]) -> Type {
    let arg = args.get(0).unwrap();
    let result = arg.eval(&mut context);
    println!("{:?}", &result);
    result
}

fn dbg(mut context: &mut Context, args:&[Type]) -> Type {
    let arg = args.get(0).unwrap();
    let result = arg.eval(&mut context);
    println!("{:?} -> {:?}", &arg, &result);
    result
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

fn apply(mut context: &mut Context, args:&[Type]) -> Type {
    if let Type::List(elems) = args.get(1).unwrap().eval(&mut context) {
        if let Type::Function(f) = args.get(0).unwrap().eval(&mut context) {
            f.eval(&mut context, &elems[..])
        } else { panic!() }
    } else { panic!() }
}

fn push(mut context: &mut Context, args:&[Type]) -> Type {
    let first = args.get(0).unwrap().eval(&mut context);
    if let Type::List(elems) = args.get(1).unwrap().eval(&mut context) {
        let mut new_list = elems.clone();
        new_list.push(first);
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

fn fn_generic(context: &mut Context, args:& [Type], is_macro: bool) -> Type {
    let name = if let Type::Symbol(name) = args.get(0).unwrap() {
        name.clone()
    } else { panic!()};
    let argument_bindings = if let Type::List(names) = args.get(1).unwrap() {
        names.clone()
    } else { panic!() };
    let body = args[2..].to_vec();
    let arglen = argument_bindings.len();

    let is_vararg = arglen > 0 && if let Type::Symbol(name) = argument_bindings.get(arglen - 1).unwrap() {
        name.ends_with("...")
    } else {
        panic!()
    };

    let vararg = if is_vararg {
        let vararg_name = if let Type::Symbol(name) = argument_bindings.get(arglen - 1).unwrap() {
            let len = name.len() - 3; // drop ...
            name[0..len].to_string()
        } else { panic!() };
        Some(Type::Symbol(vararg_name))
    } else {
        None
    };

    let argument_bindings = if is_vararg {
        let butlast = arglen - 1;
        argument_bindings[0..butlast].to_vec()
    } else {
        argument_bindings
    };

    Type::Function(FunctionType::UserDefined(Rc::new(Function {
        context: context.clone(),
        name: name,
        args: argument_bindings,
        body: body,
        is_macro: is_macro,
        vararg: vararg
    })))
}

/**
(fn name (a b c)
    (+ a b c))
*/
fn fn_special(mut context: &mut Context, args:& [Type]) -> Type {
    fn_generic(&mut context, &args, false)
}

fn macro_scpecial(mut context: &mut Context, args:& [Type]) -> Type {
    fn_generic(&mut context, &args, true)
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

fn add_to_context(name: &str, context: &mut Context, value: NativeFunction) {
    let name = String::from(name);
    context.insert(name.clone(), Type::Function(FunctionType::Native(name, value)));
}

pub fn init_context() -> Context {
    macro_rules! add {
        ( $( $n:expr , $f:expr  ),* ) => {{
            let mut context: HashMap<String, Type> = HashMap::new();
            $(
                add_to_context($n, &mut context, $f);
            )*
            context
        }};
    }

    add!["def", def_special,
         "let", let_special,
         "fn", fn_special,
         "if", if_special,
         "macro", macro_scpecial,
         "*", mult,
         "+", add,
         "-", sub,
         ">", gt,
         "quote", quote,
         "list", list,
         "car", car,
         "cdr", cdr,
         "push", push,
         "dbg", dbg,
         "prn", prn,
         "is-list", is_list,
         "cons", cons,
         "apply", apply,
         "eq", eq,
         "and", and,
         "or", or,
         "not", not]
}