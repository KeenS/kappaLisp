use std::rc::Rc;
use std::fmt;
use std::fmt::{Display, Formatter, Error as E};
use std::error;
use std::result;
use std::convert::From;
use std::ops::Deref;

use env::Env;
use ::util::*;

pub type Kfloat = f32;
pub type Kint = isize;
pub type Result<T> = result::Result<T, Error>;

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Int(Kint),
    Float(Kfloat),
    Cons(Rc<Expr>, Rc<Expr>),
    Nil,
    Sym(String),
    Str(String),
    Proc(Proc),
    EOF,
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
    Any,
}

#[derive(Clone)]
pub enum Proc {
    Lambda(Rc<Expr>, Rc<Expr>),
    Prim(String, Rc<Fn(&mut Env, &Expr) -> Result<Expr>>),
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidArgument(Expr),
    Type(Type, Expr),
    ArityShort,
    ArityExceed,
    Form(Expr),
    NotFunction(Expr),
    Unbound(String),
    User(String),
}



impl From<Kint> for Expr {
    fn from(i: Kint) -> Self {
        kint(i)
    }
}
impl From<Kfloat> for Expr {
    fn from(f: Kfloat) -> Self {
        kfloat(f)
    }
}
impl<'a> From<&'a str> for Expr {
    fn from(s: &str) -> Self {
        kstr(s.to_owned())
    }
}
impl From<String> for Expr {
    fn from(s: String) -> Self {
        kstr(s)
    }
}
impl<'a> From<&'a Expr> for Expr {
    fn from(e: &'a Expr) -> Self {
        e.clone()
    }
}





impl PartialEq for Proc {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Proc::Lambda(ref param1, ref body1),
             &Proc::Lambda(ref param2, ref body2)) => param1 == param2 && body1 == body2,
            _ => false,
        }
    }
}

impl fmt::Debug for Proc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Proc::Lambda(ref param, ref body) => write!(f, "Lambda({}, {})", param, body),
            &Proc::Prim(ref name, _) => write!(f, "Prim(#<native function {}>)", name),
        }
    }
}


impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Expr::Int(i) => write!(f, "{}", i),
            &Expr::Float(fl) => write!(f, "{}", fl),
            // :TODO: pretty print for lists
            &Expr::Cons(ref car, ref cdr) => {
                let mut tmp = cdr.deref();
                try!(write!(f, "("));
                try!(write!(f, "{}", car));

                loop {
                    match tmp {
                        &Expr::Cons(ref car, ref cdr) => {
                            try!(write!(f, " {}", car));
                            tmp = cdr.deref()
                        }
                        &Expr::Nil => break,
                        cdr => {
                            try!(write!(f, " . {}", cdr));
                            break;
                        }
                    }
                }
                write!(f, ")")
            }
            &Expr::Nil => write!(f, "nil"),
            &Expr::Sym(ref s) => write!(f, "{}", s),
            &Expr::Str(ref s) => write!(f, "\"{}\"", s),
            &Expr::Proc(ref p) => write!(f, "{}", p),
            &Expr::EOF => write!(f, "<EOF>"),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Type::Int => write!(f, "integer"),
            &Type::Float => write!(f, "float"),
            &Type::Cons => write!(f, "cons"),
            &Type::Nil => write!(f, "nil"),
            &Type::Sym => write!(f, "symbol"),
            &Type::Str => write!(f, "string"),
            &Type::Proc => write!(f, "procedure"),
            &Type::Any => write!(f, "any"),
        }
    }
}



impl fmt::Display for Proc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Proc::Lambda(ref args, ref body) => write!(f, "(lambda {} {})", args, body),
            &Proc::Prim(ref name, _) => write!(f, "{}", name),
        }
    }
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), E> {
        let res = match self {
            &Error::InvalidArgument(ref args) => write!(f, "invalid argument: {}", args),
            &Error::Type(ref t, ref args) => {
                write!(f, "type mismatch: expected: {}, got: {}", t, args)
            }
            &Error::ArityShort => write!(f, "too few argument"),
            &Error::ArityExceed => write!(f, "too many argument"),
            &Error::Form(ref e) => write!(f, "invalid form: {}", e),
            &Error::NotFunction(ref e) => write!(f, "not a function: {}", e),
            &Error::Unbound(ref s) => write!(f, "unbound variable: {}", s),
            &Error::User(ref s) => write!(f, "user error: {}", s),
        };
        try!(res);
        Ok(())
    }
}


impl error::Error for Error {
    fn description(&self) -> &str {
        "Lisp Error"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
