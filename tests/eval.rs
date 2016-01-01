extern crate kappa_lisp;
use kappa_lisp::{init, run_new, run, Env};
use kappa_lisp::util::*;

#[test]
fn test_atom(){
    assert_eq!(run_new("1"), Ok(kint(1)));
    assert_eq!(run_new("()"), Ok(knil()));
//    assert_eq!(run_new(("")), Ok(keOF));
    assert_eq!(run_new("\"string\""), Ok(kstr("string".to_string())));
}
// TODO: test `function`


#[test]
fn test_progn(){
    assert_eq!(run_new("(progn 1 2)"), Ok(kint(2)));
    assert_eq!(run_new("(progn (+ 1 2) (+ 2 3))"), Ok(kint(5)));
}

#[test]
fn test_lambda(){
    assert_eq!(run_new(("(lambda (x) x)")), Ok(kproc(klambda(list1(ksym("x")),
                                                                           list2(ksym("progn"), ksym("x"))))));
    assert_eq!(run_new("((lambda (x) (+ x x)) 1)"), Ok(kint(2)))
}


#[test]
fn test_fset(){
    let mut env = Env::new();
    init(&mut env).unwrap();
    assert_eq!(run(&mut env, "(fset 'add2 (lambda (x) (+ x 2)))"), Ok(knil()));
    assert_eq!(run(&mut env, "(add2 2)"), Ok(kint(4)));
}


#[test]
fn test_set() {
    let mut env = Env::new();
    init(&mut env).unwrap();
    assert_eq!(run(&mut env, "(set 'foo (+ 1 2 3))"), Ok(knil()));
    assert_eq!(run(&mut env, "foo"), Ok(kint(6)));
    
}

#[test]
fn test_if() {
    assert_eq!(run_new("(if () 1 2)"), Ok(kint(2)));
    assert_eq!(run_new("(if 1 1 2)"), Ok(kint(1)));
}
