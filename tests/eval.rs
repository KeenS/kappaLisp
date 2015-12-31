extern crate kappa_lisp;
use kappa_lisp::kappa_lisp::{init, run_new, run, Env};
use kappa_lisp::util::{list1, list2};
use kappa_lisp::expr::*;

use std::rc::Rc;

#[test]
fn test_atom(){
    assert_eq!(run_new(("1")), Ok(Expr::Int(1)));
    assert_eq!(run_new(("()")), Ok(Expr::Nil));
    assert_eq!(run_new(("")), Ok(Expr::EOF));
    assert_eq!(run_new(("\"string\"")), Ok(Expr::Str("string".to_string())));
}
// TODO: test `function`


#[test]
fn test_progn(){
    assert_eq!(run_new(("(progn 1 2)")), Ok(Expr::Int(2)));
    assert_eq!(run_new(("(progn (+ 1 2) (+ 2 3))")), Ok(Expr::Int(5)));
}

#[test]
fn test_lambda(){
    assert_eq!(run_new(("(lambda (x) x)")), Ok(Expr::Proc(Proc::Lambda(Rc::new(list1(Expr::Sym("x".to_string()))),
                                                                           Rc::new(list2(Expr::Sym("progn".to_string()), Expr::Sym("x".to_string())))))));
    assert_eq!(run_new(("((lambda (x) (+ x x)) 1)")), Ok(Expr::Int(2)))
}


#[test]
fn test_fset(){
    let mut env = Env::new();
    init(&mut env).unwrap();
    assert_eq!(run(&mut env, ("(fset 'add2 (lambda (x) (+ x 2)))")), Ok(Expr::Nil));
    assert_eq!(run(&mut env, ("(add2 2)")), Ok(Expr::Int(4)));
}


#[test]
fn test_set() {
    let mut env = Env::new();
    init(&mut env).unwrap();
    assert_eq!(run(&mut env, ("(set 'foo (+ 1 2 3))")), Ok(Expr::Nil));
    assert_eq!(run(&mut env, ("foo")), Ok(Expr::Int(6)));
    
}

#[test]
fn test_if() {
    assert_eq!(run_new(("(if () 1 2)")), Ok(Expr::Int(2)));
    assert_eq!(run_new(("(if 1 1 2)")), Ok(Expr::Int(1)));
}
