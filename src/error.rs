use std::fmt::{Display, Formatter, Error as E};
use std::error;


#[derive(Debug)]
pub enum Error {
    InvalidArgument,
    Type,
    ArityShort,
    ArityExceed,
    Form,
    NotFunction,
    Unbound,
    User(String)
}


impl Display for Error {
    fn fmt(&self,  f:&mut Formatter) -> Result<(), E> {
        let res = match self {
            &Error::InvalidArgument => write!(f, "invalid argument"),
            &Error::Type => write!(f, "type mismatch"),
            &Error::ArityShort => write!(f, "too few argument"),
            &Error::ArityExceed => write!(f, "too many argument"),
            &Error::Form => write!(f, "invalid form"),
            &Error::NotFunction => write!(f, "not a function"),
            &Error::Unbound => write!(f, "variable unbound"),
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

    
