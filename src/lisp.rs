use std::collections::HashMap;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt;
use std::rc::Rc;

use lexer;
use parser;
use built_in;

pub type Function = Fn(&mut Context, &[Type]) -> Type;
pub type Context = HashMap<String, Type>;

#[derive(Clone)]
pub enum Type {
    Symbol(String), Bool(bool), Number(u32), List(Vec<Type>), Function(String, Rc<Function>)
}

impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Function(name, rc), Type::Function(name_other, rc_other))  => {
                // should work, probably xD
                name.eq(name_other) && (rc.as_ref() as *const _ ==  rc_other.as_ref() as *const _)
            },
            (Type::List(elems), Type::List(elems_other)) => elems.eq(elems_other),
            (Type::Number(n), Type::Number(n_other)) => n.eq(n_other),
            (Type::Symbol(s), Type::Symbol(s_other)) => s.eq(s_other),
            (Type::Bool(b), Type::Bool(b_other)) => b.eq(b_other),
            (_,_) => false
        }
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Type::Function(name, _function) => name.fmt(f),
            Type::List(elems) => elems.fmt(f),
            Type::Number(n) => n.fmt(f),
            Type::Symbol(s) => s.fmt(f),
            Type::Bool(b) => b.fmt(f)
        }

    }
}

pub trait Eval {
    fn eval(&self, context: &mut Context) -> Type;
}

impl Eval for Type {
    fn eval(&self, mut context: &mut Context) -> Type {
        match self {
            Type::List(elems) => {
                let symbol = elems.get(0).unwrap().eval(&mut context);
                return if let Type::Function(_name, f) = symbol  {
                    f(&mut context, &elems[1..])
                } else {
                    panic!("function expected as first argument")
                };
            },
            Type::Number(_n) => self.clone(), // evaluates to itself
            Type::Bool(_b) => self.clone(), // evaluates to itself
            Type::Symbol(name) => {
                context.get(name).unwrap().clone()
            },
            Type::Function(_name, _f) => unimplemented!()
        }
    }
}

pub fn eval(input: &String) -> Type {
    if input.is_empty() {
        return Type::List(vec![]); // empty list is nil in scheme
    }
    let res = lexer::parse_fsm(&input);
    let (n, _) = parser::build(&res, 0);
    let mut context = built_in::init_context();

    let len = &n.len();
    let butlast = len - 1;
    for form in &n[..butlast] {
        form.eval(&mut context);
    }
    n.get(len - 1).unwrap().eval(&mut context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_forms() {
        assert_eq!(eval(&String::from("(def a 1) (+ a a)")), Type::Number(2));
        assert_eq!(eval(&String::from("(def a 10) (def sq (fn sq (a) (* a a))) (sq a)")), Type::Number(100));
        assert_eq!(eval(&String::from("1")), Type::Number(1));
        assert_eq!(eval(&String::from("")), Type::List(vec![]));
        assert_eq!(eval(&String::from("(+ 1 2)")), Type::Number(3));
        assert_eq!(eval(&String::from("(* 2 2)")), Type::Number(4));
        assert_eq!(eval(&String::from("(> 4 2)")), Type::Bool(true));
        assert_eq!(eval(&String::from("(- 4 2)")), Type::Number(2));
        assert_eq!(eval(&String::from("(* 10 20) (- 4 2)")), Type::Number(2));
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
}