use std::fmt::{Display, Formatter, Error as E};
use std::error;
use expr::{Expr, Type};

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidArgument(Expr),
    Type(Type, Expr),
    ArityShort,
    ArityExceed,
    Form(Expr),
    NotFunction(Expr),
    Unbound(String),
    User(String)
}


impl Display for Error {
    fn fmt(&self,  f:&mut Formatter) -> Result<(), E> {
        let res = match self {
            &Error::InvalidArgument(ref args) => write!(f, "invalid argument: {}", args),
            &Error::Type(ref t, ref args) => write!(f, "type mismatch: expected: {}, got: {}", t, args),
            &Error::ArityShort => write!(f, "too few argument"),
            &Error::ArityExceed => write!(f, "too many argument"),
            &Error::Form(ref e) => write!(f, "invalid form: {}", e),
            &Error::NotFunction(ref e) => write!(f, "not a function: {}", e),
            &Error::Unbound(ref s) => write!(f, "unbound variable: {}", s),
            &Error::User(ref s) => write!(f, "user error: {}", s)
                
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

    
