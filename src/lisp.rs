use std::collections::HashMap;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt;
use std::rc::Rc;

pub type Applyable = Fn(&mut Context, &[Type]) -> Type;
pub type Context = HashMap<String, Type>;

#[derive(Clone)]
pub enum Type {
    Symbol(String), Bool(bool), Number(u32), List(Vec<Type>), Function(String, Rc<Applyable>)
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Type::Function(name, _function) => name.fmt(f),
            Type::List(elems) => elems.fmt(f),
            Type::Number(n) => n.fmt(f),
            Type::Symbol(s) => s.fmt(f),
            _ => write!(f, "Unknown")
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
                    panic!()
                };
            },
            Type::Number(_n) => self.clone(), // evaluates to itself
            Type::Bool(_b) => self.clone(), // evaluates to itself
            Type::Symbol(name) => {
                let option = context.get(name);
                option.unwrap().clone()
            },
            Type::Function(_name, _f) => unimplemented!()
        }
    }
}