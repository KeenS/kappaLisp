use std::rc::Rc;
use std::ops::Deref;
use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Expr {
    Int(isize),
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
    Cons,
    Nil,
    Sym,
    Str,
    Proc,
    Any
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Proc {
    Lambda(Rc<Expr>, Rc<Expr>),
    Prim(Prim)
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum Prim {
    Add,
    Sub,
    Div,
    Mul,
    Concat,
    Funcall,
    Car,
    Cdr,
    CurrentTimeString,
    SkkCalc,
    SkkGadgetUnitsConversion
}


impl Expr {
    pub fn cons(car: Expr, cdr: Expr) -> Expr {
        Expr::Cons(Rc::new(car), Rc::new(cdr))
    }

    pub fn list1(a1: Expr) -> Expr {
        Expr::cons(a1, Expr::Nil)
    }
    pub fn list2(a1: Expr, a2: Expr) -> Expr {
        Expr::cons(a1, Expr::list1(a2))
    }

    pub fn car(cons: Expr) -> Result<Expr, String> {
        match cons {
            Expr::Cons(ref car, _) => Ok(car.deref().clone()),
            arg => Err(format!("invalid argument {} is passed to car", arg))
        }
    }
    pub fn cdr(cons: Expr) -> Result<Expr, String> {
        match cons {
            Expr::Cons(_, ref cdr) => Ok(cdr.deref().clone()),
            arg => Err(format!("invalid argument {} is passed to cdr", arg))
        }
    }
}


impl fmt::Display for Expr {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self.clone() {
            Expr::Int(i) => write!(f, "{}", i),
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
            Type::Int  => write!(f, "integer"),
            Type::Cons => write!(f, "cons"),
            Type::Nil  => write!(f, "nil"),
            Type::Sym  => write!(f, "symbol"),
            Type::Str  => write!(f, "string"),
            Type::Proc => write!(f, "procedure"),
            Type::Any  => write!(f, "any")
        }
    }
}



impl fmt::Display for Proc {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self.clone() {
            Proc::Lambda(args, body) => write!(f, "(lambda {} {})", args, body),
            Proc::Prim(prim) => write!(f, "{}", prim)
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
            Prim::Car => write!(f, "car"),
            Prim::Cdr => write!(f, "cdr"),
            Prim::CurrentTimeString => write!(f, "current-time-string"),
            Prim::SkkCalc => write!(f, "skk-calc"),
            Prim::SkkGadgetUnitsConversion => write!(f, "skk-gadget-units-conversion")
        }
    }
    
}
