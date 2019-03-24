use std::collections::HashMap;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt;
use std::rc::Rc;
use core::ops::Deref;

use crate::lexer;
use crate::parser;
use crate::built_in;

#[derive(Clone, PartialEq, Debug)]
pub struct Function {
    pub is_macro: bool,
    pub context: Context,
    pub name: String,
    pub args: Vec<Type>,
    pub vararg: Option<Type>,
    pub body: Vec<Type>
}

#[derive(Clone)]
pub enum FunctionType {
    Native(String, NativeFunction),
    UserDefined(Rc<Function>)
}
pub type NativeFunction = fn(&mut Context, &[Type]) -> Type;
pub type Context = HashMap<String, Type>;

#[derive(Clone, PartialEq)]
pub enum Type {
    Symbol(String), Bool(bool), Number(u32), List(Vec<Type>), Function(FunctionType)
}

impl PartialEq for FunctionType {
    fn eq(&self, other: &FunctionType) -> bool {
        match (self, other) {
            (FunctionType::UserDefined(f), FunctionType::UserDefined(f_other)) => f.eq(f_other),
            (FunctionType::Native(n, f), FunctionType::Native(n_other, f_other)) => n.eq(n_other) && (f as *const _ == f_other as *const _),
            (_,_) => false
        }
    }
}

impl fmt::Debug for FunctionType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            FunctionType::Native(name, _) => format!("native({})", name).fmt(f),
            FunctionType::UserDefined(fun) => fun.name.fmt(f)
        }
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Type::Function(ft) => ft.fmt(f),
            Type::List(elems) => elems.fmt(f),
            Type::Number(n) => n.fmt(f),
            Type::Symbol(s) => s.fmt(f),
            Type::Bool(b) => b.fmt(f)
        }
    }
}

//TODO decouple allowing macroexpand-1
impl FunctionType {
    pub fn eval(&self, mut context: &mut Context, args: &[Type]) -> Type {
        match self {
            FunctionType::Native(_, f) => {
                f(&mut context, args)
            },
            FunctionType::UserDefined(f_struct) => {
                let Function {
                    context: captured_context, 
                    name: f_name, 
                    args: argument_bindings, 
                    body, 
                    is_macro, 
                    vararg
                } = f_struct.deref();
                // TODO speed up clone
                let mut current_context = if *is_macro {
                    context.clone()
                } else {
                    captured_context.clone()
                };
                // assert_eq!(args.len(), argument_bindings.len(), "argument size mismatch {:?} -> {:?}", &args, &argument_bindings);
                for i in 0..argument_bindings.len() {
                    let arg_name = if let Type::Symbol(name) = argument_bindings.get(i).unwrap() {
                        name
                    } else { panic!() };
                    let arg = if *is_macro {
                        args.get(i).unwrap().clone() //eval function args first with current lexical scope
                    } else {
                        args.get(i).unwrap().eval(&mut context) //macro arg should not be avaluated
                    };
                    current_context.insert(arg_name.clone(), arg);
                    current_context.insert(f_name.clone(), Type::Function(self.clone())); //named lambdas
                }

                match vararg {
                    Some(Type::Symbol(name)) => {
                        let from = argument_bindings.len();
                        let to = args.len();
                        let varargs = if *is_macro {
                            args[from..to].to_vec()
                        } else {
                            args[from..to].into_iter().map(|a| a.eval(&mut context)).collect()
                        };
                        current_context.insert(name.clone(), Type::List(varargs));
                    },
                    None => (),
                    _ => unreachable!()
                }

                let result = eval_forms(&body, &mut current_context);
                if *is_macro {
                    eval_forms(&result, &mut context)
                } else {
                    result
                }.last().unwrap().clone()
            }
        }
    }
}

fn eval_forms(forms: &[Type], mut ctx: &mut Context) -> Vec<Type> {
    let mut result = vec![];
    for form in &forms[..] {
        result.push(form.eval(&mut ctx));
    }
    result
}

impl Type {
    pub fn eval(&self, mut context: &mut Context) -> Type {
        match self {
            Type::List(elems) => {
                let symbol = elems.get(0).unwrap().eval(&mut context);
                if let Type::Function(f) = symbol  {
                    f.eval(&mut context, &elems[1..])
                } else {
                    panic!("function expected as first argument")
                }
            },
            Type::Number(_n) => self.clone(), // evaluates to itself
            Type::Bool(_b) => self.clone(), // evaluates to itself
            Type::Symbol(name) => {
                context.get(name).expect(format!("Symbol not found -> {:?}", name).as_str()).clone()
            },
            _ => unimplemented!()
        }
    }
}

pub fn eval_in_context(input: &String, mut context: &mut Context) -> Type {
    if input.is_empty() {
        return Type::List(vec![]); // empty list is nil
    }
    let res = lexer::parse_fsm(&input);
    let (n, _) = parser::build(&res, 0);

    // execute all forms and return result from last form
    let len = &n.len();
    let butlast = len - 1;
    for form in &n[..butlast] {
        form.eval(&mut context);
    }
    n.get(len - 1).unwrap().eval(&mut context)
}

pub fn eval(input: &String) -> Type {
    let mut context = built_in::init_context();
    eval_in_context(&input, &mut context)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn bootstrap_and_eval(input: &String) -> Type {
        let mut context = built_in::init_context();
        let bytes = include_bytes!("../res/init.lisp");
        let init_str = String::from_utf8_lossy(bytes).to_string();
        eval_in_context(&init_str, &mut context);
        eval_in_context(&input, &mut context)
    }

    #[test]
    fn test_simple_forms() {
        assert_eq!(eval(&String::from("(def a 1) (+ a a)")), Type::Number(2));
        assert_eq!(eval(&String::from(" (def a 1) (+ a a) ")), Type::Number(2));
        assert_eq!(eval(&String::from("(def a 10) (def sq (fn sq (a) (* a a))) (sq a)")), Type::Number(100));
        assert_eq!(eval(&String::from("((fn sq (a) (* a a)) 10)")), Type::Number(100));
        assert_eq!(eval(&String::from("1")), Type::Number(1));
        assert_eq!(eval(&String::from("")), Type::List(vec![]));
        assert_eq!(eval(&String::from("(+ 1 2)")), Type::Number(3));
        assert_eq!(eval(&String::from("(* 2 2)")), Type::Number(4));
        assert_eq!(eval(&String::from("(> 4 2)")), Type::Bool(true));
        assert_eq!(eval(&String::from("(- 4 2)")), Type::Number(2));
        assert_eq!(eval(&String::from("(* 10 20) (- 4 2)")), Type::Number(2));
    }

    #[test]
    fn test_vararg() {
        assert_eq!(eval(&String::from("((fn a (c...) c))")), Type::List(vec![]));
        assert_eq!(eval(&String::from("((fn a (b c...) b) 1 2 3)")), Type::Number(1));
        assert_eq!(eval(&String::from("((fn a (b c...) c) 1 2 3)")), Type::List(vec![Type::Number(2), Type::Number(3)]));
        assert_eq!(eval(&String::from("((fn a (c...) c) 1 2 3)")), Type::List(vec![Type::Number(1), Type::Number(2), Type::Number(3)]));
    }

    #[test]
    fn test_eq() {
        assert_eq!(eval(&String::from("(eq 1 1)")), Type::Bool(true));
        assert_eq!(eval(&String::from("(eq 1 2)")), Type::Bool(false));
        assert_eq!(eval(&String::from("(eq (list 1 2) (list 1 2))")), Type::Bool(true));
        assert_eq!(eval(&String::from("(eq (list 1 2) (list 1 2 3))")), Type::Bool(false));
        assert_eq!(eval(&String::from("(eq (quote 1) 1)")), Type::Bool(true));
    }

    #[test]
    fn test_and_or_not() {
        assert_eq!(eval(&String::from("(and (eq 1 1) (> 2 1))")), Type::Bool(true));
        assert_eq!(eval(&String::from("(and (eq 1 2) (> 2 1))")), Type::Bool(false));
        assert_eq!(eval(&String::from("(not (and (eq 1 2) (> 2 1)))")), Type::Bool(true));
        assert_eq!(eval(&String::from("(not (or (eq 1 2) (> 2 1)))")), Type::Bool(false));
        assert_eq!(eval(&String::from("(not (or (eq 1 2) (> 2 3)))")), Type::Bool(true));
    }

    #[test]
    fn test_named_lambdas() {
        assert_eq!(eval(&String::from("((fn sum (l) (if (> l 0) (+ l (sum (- l 1))) l)) 3)")),
                   Type::Number(6));
    }

    #[test]
    fn test_macro() {
        assert_eq!(eval(&String::from("(def add (macro add (a b) (list (quote +) a b)))
                                       (add 10 20)
                                       (add 10 30)")),
                   Type::Number(40));
        assert_eq!(eval(&String::from("(def defmacro (macro defmacro (name args body) (list (quote def) name (list (quote macro) name args body))))
                                       (defmacro defn (name args body) (list (quote def) name (list (quote fn) name args body)))
                                       (defn add (a b) (+ a b))
                                       (add 10 20)")),
                   Type::Number(30));
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(eval(&String::from("(let ((fib (fn fib (n) \
                                                (if (> 3 n)\
                                                    (- n 1)\
                                                    (+ (fib (- n 1))\
                                                    (fib (- n 2)))))))\
                                        (fib 3)\
                                        (fib 8))")),
                   Type::Number(13));
    }

    #[test]
    fn test_higher_order_functions() {
        assert_eq!(eval(&String::from("(let ((apply (fn apply (f n) (f (n)))))\
                                        (apply (fn _ (a) (* a a)) (fn _ () 10)))")),
                   Type::Number(100));
    }

    #[test]
    fn test_fn_eq() {
        // TODO fix
        // doesn't work :(
        // assert_eq!(eval(&String::from("(eq + +)")),
        //            Type::Bool(true));
        assert_eq!(eval(&String::from("(eq + -)")),
                   Type::Bool(false));
        assert_eq!(eval(&String::from("(def a (fn a () 1))
                                       (def b (fn b () 2))
                                       (list (eq a b) (eq a a) (eq b b))")),
                   Type::List(vec![Type::Bool(false), Type::Bool(true), Type::Bool(true)]));
    }

    #[test]
    fn test_closures() {
        assert_eq!(eval(&String::from("(let ((a 100)\
                                            (adda (fn adda (n) (+ a n))))\
                                        (adda 3))")),
                   Type::Number(103));
    }

    #[test]
    fn test_closures_with_recursion() {
        assert_eq!(eval(&String::from("(let ((one 1)\
                                            (three 3)\
                                            (two 2)\
                                            (fib (fn fib (n) \
                                                (if (> three n)\
                                                    (- n one)\
                                                    (+ (fib (- n one))\
                                                    (fib (- n two)))))))\
                                        (fib 8))")),
                   Type::Number(13));
    }

    #[test]
    #[should_panic]
    fn test_nested_scope_invisible() {
        assert_eq!(eval(&String::from("(let ((a (let ((b 1)(c 2)) (+ b c))))\
                                            b)")),
                   Type::Number(13));
    }

    #[test]
    fn test_nested_scope_invisible_proper_setup() {
        assert_eq!(eval(&String::from("(let ((a (let ((b 1)(c 2)) (+ b c))))\
                                            a)")),
                   Type::Number(3));
    }

    #[test]
    fn test_closure_captures_nested_context() {
        assert_eq!(eval(&String::from("(let ((a (let ((b 1)(c 2)) (fn _ () (+ b c)))))\
                                            (a))")),
                   Type::Number(3));
    }

    #[test]
    fn test_quote() {
        assert_eq!(eval(&String::from("(quote (1))")),
                   Type::List(vec![Type::Number(1)]));
        assert_eq!(eval(&String::from("(quote 1)")),
                   Type::Number(1));
        assert_eq!(eval(&String::from("(quote ())")),
                   Type::List(vec![]));
    }

    #[test]
    fn test_list() {
        assert_eq!(eval(&String::from("(list)")),
                   Type::List(vec![]));
        assert_eq!(eval(&String::from("(list 1 2 3)")),
                   Type::List(vec![Type::Number(1), Type::Number(2), Type::Number(3)]));
        assert_eq!(eval(&String::from("(list 1 (+ 1 2))")),
                   Type::List(vec![Type::Number(1), Type::Number(3)]));
    }

    #[test]
    fn test_car() {
        assert_eq!(eval(&String::from("(car (list))")),
                   Type::List(vec![]));
        assert_eq!(eval(&String::from("(car)")),
                   Type::List(vec![]));
        assert_eq!(eval(&String::from("(car (list (+ 0 0 0 0 1) 2 3))")),
                   Type::Number(1));
    }

    #[test]
    fn test_cdr() {
        assert_eq!(eval(&String::from("(cdr (list 1 (+ 0 2) (+ 1 2)))")),
                   Type::List(vec![Type::Number(2), Type::Number(3)]));
        assert_eq!(eval(&String::from("(cdr (list))")),
                   Type::List(vec![]));
        assert_eq!(eval(&String::from("(cdr)")),
                   Type::List(vec![]));
    }

    #[test]
    fn test_cons() {
        assert_eq!(eval(&String::from("(cons 0 (list 1 (+ 0 2) (+ 1 2)))")),
                   Type::List(vec![Type::Number(0), Type::Number(1), Type::Number(2), Type::Number(3)]));
        assert_eq!(eval(&String::from("(cons 0 (list))")),
                   Type::List(vec![Type::Number(0)]));
        assert_eq!(eval(&String::from("(cons (quote 0) (list))")),
                   Type::List(vec![Type::Number(0)]));
    }

    #[test]
    fn test_push() {
        assert_eq!(eval(&String::from("(push 1 (list 1 2 3))")),
                   Type::List(vec![Type::Number(1), Type::Number(2), Type::Number(3), Type::Number(1),]));
    }

    #[test]
    fn integration_1() {
        assert_eq!(bootstrap_and_eval(&String::from("(-> 10 (genlist) (map square) (map square))")),
                   Type::List(vec![Type::Number(10000), Type::Number(6561), Type::Number(4096), Type::Number(2401), 
                                   Type::Number(1296), Type::Number(625), Type::Number(256), Type::Number(81),
                                   Type::Number(16), Type::Number(1)]));
    }

    #[test]
    fn integration_2() {
        assert_eq!(bootstrap_and_eval(&String::from("(-> (list 1 2 3) (reverse))")),
                   Type::List(vec![Type::Number(3), Type::Number(2), Type::Number(1)]));
    }
}

