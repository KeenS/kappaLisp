use std::convert::From;
use std::error;
use std::fmt;
use std::fmt::{Display, Error as E, Formatter};
use std::ops::Deref;
use std::rc::Rc;
use std::result;

use env::Env;
use util::*;

pub type Kfloat = f32;
pub type Kint = isize;
pub type Result<T> = result::Result<T, Error>;

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Int(Kint),
    Float(Kfloat),
    Cons(Rc<Expr>, Rc<Expr>),
    Nil,
    Sym(Rc<String>),
    Keyword(Rc<String>),
    Str(Rc<String>),
    Proc(Proc),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Type {
    Int,
    Float,
    Cons,
    Nil,
    Sym,
    Keyword,
    Str,
    Proc,
    Any,
}

#[derive(Clone)]
pub enum Proc {
    Lambda(Rc<Expr>, Rc<Expr>),
    Prim(String, Rc<Fn(&mut Env, &Expr) -> Result<Expr>>),
    Expr(Rc<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Error {
    ReadError,
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
            (Proc::Lambda(param1, body1), Proc::Lambda(param2, body2)) => {
                param1 == param2 && body1 == body2
            }
            _ => false,
        }
    }
}

impl fmt::Debug for Proc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Proc::Lambda(param, body) => write!(f, "Lambda({}, {})", param, body),
            Proc::Prim(name, _) => write!(f, "Prim(#<native function {}>)", name),
            Proc::Expr(e) => write!(f, "{}", e),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Int(i) => write!(f, "{}", i),
            Expr::Float(fl) => write!(f, "{}", fl),
            // :TODO: pretty print for lists
            Expr::Cons(car, cdr) => {
                let mut tmp = cdr.deref();
                write!(f, "(")?;
                write!(f, "{}", car)?;

                loop {
                    match tmp {
                        Expr::Cons(car, cdr) => {
                            write!(f, " {}", car)?;
                            tmp = cdr.deref()
                        }
                        Expr::Nil => break,
                        cdr => {
                            write!(f, " . {}", cdr)?;
                            break;
                        }
                    }
                }
                write!(f, ")")
            }
            Expr::Nil => write!(f, "nil"),
            Expr::Sym(s) => write!(f, "{}", s),
            Expr::Keyword(s) => write!(f, ":{}", s),
            Expr::Str(s) => write!(f, "\"{}\"", s),
            Expr::Proc(p) => write!(f, "{}", p),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Int => write!(f, "integer"),
            Type::Float => write!(f, "float"),
            Type::Cons => write!(f, "cons"),
            Type::Nil => write!(f, "nil"),
            Type::Sym => write!(f, "symbol"),
            Type::Keyword => write!(f, "keyword"),
            Type::Str => write!(f, "string"),
            Type::Proc => write!(f, "procedure"),
            Type::Any => write!(f, "any"),
        }
    }
}

impl fmt::Display for Proc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Proc::Lambda(args, body) => write!(f, "(lambda {} {})", args, body),
            Proc::Prim(name, _) => write!(f, "{}", name),
            Proc::Expr(e) => write!(f, "{}", e),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), E> {
        let res = match self {
            Error::ReadError => write!(f, "read error"),
            Error::InvalidArgument(args) => write!(f, "invalid argument: {}", args),
            Error::Type(t, args) => write!(f, "type mismatch: expected: {}, got: {}", t, args),
            Error::ArityShort => write!(f, "too few argument"),
            Error::ArityExceed => write!(f, "too many argument"),
            Error::Form(e) => write!(f, "invalid form: {}", e),
            Error::NotFunction(e) => write!(f, "not a function: {}", e),
            Error::Unbound(s) => write!(f, "unbound variable: {}", s),
            Error::User(s) => write!(f, "user error: {}", s),
        };
        res?;
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
