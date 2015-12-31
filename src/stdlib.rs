extern crate time;
use std::ops::Deref;

use expr::{Expr, Type, Kfloat};
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
                (&Expr::Float(x), &Expr::Int(y)) => Ok(Expr::Float(expr!(x $op (y as Kfloat)))),
                (&Expr::Int(x), &Expr::Float(y)) => Ok(Expr::Float(expr!((x as Kfloat) $op y))),
                (&Expr::Float(x), &Expr::Float(y)) => Ok(Expr::Float(expr!(x $op y))),
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

pub fn k_cons(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (car, Any) (cdr, Any));
    Ok(cons(car.clone(), cdr.clone()))
}

pub fn k_car(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, ((car, _), Cons));
    Ok(car.clone())
}

pub fn k_cdr(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, ((_, cdr), Cons));
    Ok(cdr.clone())

}

pub fn k_list(_: &mut Env, args: &Expr) -> Result<Expr> {
    Ok(args.clone())
}

pub fn k_equal_p(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (x, Any) (y, Any));
    if x == y {
        // TODO: return `t`
        Ok(Expr::Int(1))
    } else {
        Ok(Expr::Nil)
    }
        
}


pub fn k_current_time_string(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args);
    let now = time::now();
    Ok(Expr::Str(format!("{}", now.ctime())))
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

#[test]
fn test_cons() {
    assert_eq!(eval(&mut Env::new(), &read("(cons 1 2)")), Ok(cons(Expr::Int(1), Expr::Int(2))));
    assert_eq!(eval(&mut Env::new(), &read("(cons () 2)")), Ok(cons(Expr::Nil, Expr::Int(2))));
}



#[test]
fn test_car() {
    assert_eq!(eval(&mut Env::new(), &read("(car (cons 1 2))")), Ok(Expr::Int(1)));
    assert_eq!(eval(&mut Env::new(), &read("(car (list 1 2))")), Ok(Expr::Int(1)));
}

#[test]
fn test_cdr() {
    assert_eq!(eval(&mut Env::new(), &read("(cdr (cons 1 2))")), Ok(Expr::Int(2)));
    assert_eq!(eval(&mut Env::new(), &read("(cdr (list 1 2))")), Ok(list1(Expr::Int(2))));
}


#[test]
fn test_list() {
    assert_eq!(eval(&mut Env::new(), &read("(list)")), Ok(Expr::Nil));
    assert_eq!(eval(&mut Env::new(), &read("(list 1)")), Ok(list1(Expr::Int(1))));
    assert_eq!(eval(&mut Env::new(), &read("(list 1 2)")), Ok(list2(Expr::Int(1), Expr::Int(2))));
}

#[test]
fn test_equal_p() {
    assert_eq!(eval(&mut Env::new(), &read("(equal? 1 1)")), Ok(Expr::Int(1)));
    assert_eq!(eval(&mut Env::new(), &read("(equal? 'sym 'sym)")), Ok(Expr::Int(1)));
    assert_eq!(eval(&mut Env::new(), &read("(equal? \"str\" \"str\")")), Ok(Expr::Int(1)));
    assert_eq!(eval(&mut Env::new(), &read("(equal? 1 \"str\")")), Ok(Expr::Nil));
    assert_eq!(eval(&mut Env::new(), &read("(equal? 'sym \"str\")")), Ok(Expr::Nil));
    assert_eq!(eval(&mut Env::new(), &read("(equal? (list \"str\") \"str\")")), Ok(Expr::Nil));
}


#[test]
fn test_assoc() {
    let mut env = Env::new();
    env.init().unwrap();
    assert_eq!(eval(&mut env, &read("(cdr (assoc 'two '((one . 1) (two . 2))))")), Ok(Expr::Int(2)));
    
}
