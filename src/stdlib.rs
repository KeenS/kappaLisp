use std::ops::Deref;

use expr::{Expr, Type};
use error::Error as E;
use env::{Env, Result};
use util::*;
use eval::funcall;
#[cfg(test)]
use eval::eval;
#[cfg(test)]
use read::read;

// since rust's macro cannot treat binop, work around macro is needed.
macro_rules! expr {
    ($e:expr) => {
        $e
    }
}

macro_rules! def_arith_op {
    ($name: ident, $op: tt, $init: expr) => {
        pub fn $name(mut env: &mut Env, args: &Expr) -> Result<Expr> {
            let (init, args) = match args {
                &Expr::Cons(ref hd, ref tl) => match tl.deref() {
                    tl @ &Expr::Cons(_, _) => (hd.deref().clone(), tl),
                    _ => ($init, args)
                },
                args => ($init, args)
            };
            f_foldl(env, &|_, x, y| match (x, y) {
                (&Expr::Int(x), &Expr::Int(y)) => Ok(Expr::Int(expr!(x $op y))),
                (&Expr::Int(_), y) => Err(E::Type(Type::Int, y.clone())),
                (x, _) => Err(E::Type(Type::Int, x.clone())),
                    
            }, &init, args)

        }
    }
}

def_arith_op!(k_add, +, Expr::Int(0));
def_arith_op!(k_sub, -, Expr::Int(0));
def_arith_op!(k_mul, *, Expr::Int(1));
def_arith_op!(k_div, /, Expr::Int(1));

pub fn k_concat(mut env: &mut Env, args: &Expr) -> Result<Expr> {
    let res = f_foldl(env, &|_, acc, x| match (acc, x) {
        (&Expr::Str(ref acc), &Expr::Str(ref x)) => Ok(Expr::Str(format!("{}{}",acc, x))),
        (_, y) => Err(E::Type(Type::Str, y.clone()))
    }
                      , &Expr::Str("".to_string()), &args);
    Ok(try!(res).clone())
    
}


pub fn k_funcall(mut env: &mut Env, args: &Expr) -> Result<Expr> {
    match args {
        &Expr::Cons(ref f, ref args) => match f.deref() {
            &Expr::Proc(ref f) => funcall(env, f , args.deref()),
            f => Err(E::NotFunction(f.clone()))
        },
        args => Err(E::Form(args.clone()))
    }
}

pub fn k_car(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, ((car, _), Cons));
    Ok(car.clone())
}

pub fn k_cdr(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, ((_, cdr), Cons));
    Ok(cdr.clone())

}


#[test]
fn test_funcall(){
    assert_eq!(eval(&mut Env::new(), &read("(funcall #'+ 1 2)")), Ok(Expr::Int(3)));
    assert_eq!(eval(&mut Env::new(), &read("(funcall #'(lambda (x y) (* x y)) 1 2)")), Ok(Expr::Int(2)));
    assert_eq!(eval(&mut Env::new(), &read("(funcall (lambda (x y) (* x y)) 1 2)")), Ok(Expr::Int(2)))
}

#[test]
fn test_add(){
    assert_eq!(eval(&mut Env::new(), &read("(+)")), Ok(Expr::Int(0)));
    assert_eq!(eval(&mut Env::new(), &read("(+ 1)")), Ok(Expr::Int(1)));
    assert_eq!(eval(&mut Env::new(), &read("(+ 1 2)")), Ok(Expr::Int(3)));
    assert_eq!(eval(&mut Env::new(), &read("(+ 1 2 3)")), Ok(Expr::Int(6)));
}

#[test]
fn test_sub(){
    assert_eq!(eval(&mut Env::new(), &read("(-)")), Ok(Expr::Int(0)));
    assert_eq!(eval(&mut Env::new(), &read("(- 1)")), Ok(Expr::Int(-1)));
    assert_eq!(eval(&mut Env::new(), &read("(- 1 2)")), Ok(Expr::Int(-1)));
    assert_eq!(eval(&mut Env::new(), &read("(- 1 2 3)")), Ok(Expr::Int(-4)));
}

#[test]
fn test_mul(){
    assert_eq!(eval(&mut Env::new(), &read("(*)")), Ok(Expr::Int(1)));
    assert_eq!(eval(&mut Env::new(), &read("(* 1)")), Ok(Expr::Int(1)));
    assert_eq!(eval(&mut Env::new(), &read("(* 1 2)")), Ok(Expr::Int(2)));
    assert_eq!(eval(&mut Env::new(), &read("(* 1 2 3)")), Ok(Expr::Int(6)));
}

#[test]
fn test_div(){
    assert_eq!(eval(&mut Env::new(), &read("(/)")), Ok(Expr::Int(1)));
    assert_eq!(eval(&mut Env::new(), &read("(/ 1)")), Ok(Expr::Int(1)));
    assert_eq!(eval(&mut Env::new(), &read("(/ 3 2)")), Ok(Expr::Int(1)));
    assert_eq!(eval(&mut Env::new(), &read("(/ 3 2 1)")), Ok(Expr::Int(1)));
}

#[test]
fn test_nested_arith(){
    assert_eq!(eval(&mut Env::new(), &read("(/ (- (+ 1 (* 2 3)) 3) 2)")), Ok(Expr::Int(2)));
}

#[test]
fn test_concat(){
    assert_eq!(eval(&mut Env::new(), &read("(concat \"a\" \"b\" \"cd\")")), Ok(Expr::Str("abcd".to_string())))
}