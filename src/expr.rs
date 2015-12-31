use std::rc::Rc;
use std::fmt;
use env::{Env, Result};

pub type Kfloat = f32;

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Int(isize),
    Float(Kfloat),
    Cons(Rc<Expr>, Rc<Expr>),
    Nil,
    Sym(String),
    Str(String),
    Proc(Proc),
    EOF
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Type {
    Int,
    Float,
    Cons,
    Nil,
    Sym,
    Str,
    Proc,
    Any
}

#[derive(Clone)]
pub enum Proc {
    Lambda(Rc<Expr>, Rc<Expr>),
    Prim(String, Rc<Fn(&mut Env, &Expr) -> Result<Expr>>)
}

impl PartialEq for Proc {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Proc::Lambda(ref param1, ref body1), &Proc::Lambda(ref param2, ref body2)) => param1 == param2 && body1 == body2,
            _ => false
        }
    }
}

impl fmt::Debug for Proc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Proc::Lambda(ref param, ref body) => write!(f, "Lambda({}, {})", param, body),
            &Proc::Prim(ref name, _) => write!(f, "Prim(#<native function {}>)", name)
        }
    }
} 

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum Prim {
    Add,
    Sub,
    Div,
    Mul,
    Concat,
    Funcall,
    Cons,
    Car,
    Cdr,
    List,
    EqualP,
    StringToNumber,
    CurrentTimeString,
    SkkCalc,
}


impl fmt::Display for Expr {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self.clone() {
            Expr::Int(i) => write!(f, "{}", i),
            Expr::Float(fl) => write!(f, "{}", fl),
            // :TODO: pretty print for lists
            Expr::Cons(ref car,ref cdr) => write!(f, "({} . {})", car, cdr),
            Expr::Nil => write!(f, "nil"),
            Expr::Sym(ref s) => write!(f, "{}", s),
            Expr::Str(ref s) => write!(f, "\"{}\"", s),
            Expr::Proc(p) => write!(f, "{}", p),
            Expr::EOF => write!(f, "<EOF>")
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self.clone() {
            Type::Int   => write!(f, "integer"),
            Type::Float => write!(f, "float"),
            Type::Cons  => write!(f, "cons"),
            Type::Nil   => write!(f, "nil"),
            Type::Sym   => write!(f, "symbol"),
            Type::Str   => write!(f, "string"),
            Type::Proc  => write!(f, "procedure"),
            Type::Any   => write!(f, "any")
        }
    }
}



impl fmt::Display for Proc {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self.clone() {
            Proc::Lambda(args, body) => write!(f, "(lambda {} {})", args, body),
            Proc::Prim(name, _) => write!(f, "{}", name)
        }
    }
}


impl fmt::Display for Prim {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self.clone() {
            Prim::Add => write!(f, "+"),
            Prim::Sub => write!(f, "-"),
            Prim::Div => write!(f, "/"),
            Prim::Mul => write!(f, "*"),
            Prim::Concat => write!(f, "concat"),
            Prim::Funcall => write!(f, "funcall"),
            Prim::Cons => write!(f, "cons"),
            Prim::Car => write!(f, "car"),
            Prim::Cdr => write!(f, "cdr"),
            Prim::List => write!(f, "list"),
            Prim::EqualP => write!(f, "equal?"),
            Prim::StringToNumber => write!(f, "string-to-number"),
            Prim::CurrentTimeString => write!(f, "current-time-string"),
            Prim::SkkCalc => write!(f, "skk-calc"),
        }
    }
    
}
