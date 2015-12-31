extern crate kappa_lisp;
use kappa_lisp::{run_new,Expr};
use kappa_lisp::util::{cons, list1, list2};

#[test]
fn test_funcall(){
    assert_eq!(run_new(("(funcall #'+ 1 2)")), Ok(Expr::Int(3)));
    assert_eq!(run_new(("(funcall #'(lambda (x y) (* x y)) 1 2)")), Ok(Expr::Int(2)));
    assert_eq!(run_new(("(funcall (lambda (x y) (* x y)) 1 2)")), Ok(Expr::Int(2)))
}

#[test]
fn test_add(){
    assert_eq!(run_new(("(+)")), Ok(Expr::Int(0)));
    assert_eq!(run_new(("(+ 1)")), Ok(Expr::Int(1)));
    assert_eq!(run_new(("(+ 1 2)")), Ok(Expr::Int(3)));
    assert_eq!(run_new(("(+ 1 2 3)")), Ok(Expr::Int(6)));
    assert_eq!(run_new(("(+ 1 2 3.0)")), Ok(Expr::Float(6.0)));
}

#[test]
fn test_sub(){
    assert_eq!(run_new(("(-)")), Ok(Expr::Int(0)));
    assert_eq!(run_new(("(- 1)")), Ok(Expr::Int(-1)));
    assert_eq!(run_new(("(- 1 2)")), Ok(Expr::Int(-1)));
    assert_eq!(run_new(("(- 1.0 2 3)")), Ok(Expr::Float(-4.0)));
}

#[test]
fn test_mul(){
    assert_eq!(run_new(("(*)")), Ok(Expr::Int(1)));
    assert_eq!(run_new(("(* 1)")), Ok(Expr::Int(1)));
    assert_eq!(run_new(("(* 1 2.0)")), Ok(Expr::Float(2.0)));
    assert_eq!(run_new(("(* 1 2 3)")), Ok(Expr::Int(6)));
}

#[test]
fn test_div(){
    assert_eq!(run_new(("(/)")), Ok(Expr::Int(1)));
    assert_eq!(run_new(("(/ 1)")), Ok(Expr::Int(1)));
    assert_eq!(run_new(("(/ 3 2)")), Ok(Expr::Int(1)));
    assert_eq!(run_new(("(/ 3 2.0)")), Ok(Expr::Float(1.5)));
    assert_eq!(run_new(("(/ 3 2 1)")), Ok(Expr::Int(1)));
}

#[test]
fn test_nested_arith(){
    assert_eq!(run_new(("(/ (- (+ 1 (* 2 3)) 3) 2)")), Ok(Expr::Int(2)));
}

#[test]
fn test_concat(){
    assert_eq!(run_new(("(concat \"a\" \"b\" \"cd\")")), Ok(Expr::Str("abcd".to_string())))
}

#[test]
fn test_cons() {
    assert_eq!(run_new(("(cons 1 2)")), Ok(cons(Expr::Int(1), Expr::Int(2))));
    assert_eq!(run_new(("(cons () 2)")), Ok(cons(Expr::Nil, Expr::Int(2))));
}



#[test]
fn test_car() {
    assert_eq!(run_new(("(car (cons 1 2))")), Ok(Expr::Int(1)));
    assert_eq!(run_new(("(car (list 1 2))")), Ok(Expr::Int(1)));
}

#[test]
fn test_cdr() {
    assert_eq!(run_new(("(cdr (cons 1 2))")), Ok(Expr::Int(2)));
    assert_eq!(run_new(("(cdr (list 1 2))")), Ok(list1(Expr::Int(2))));
}


#[test]
fn test_list() {
    assert_eq!(run_new(("(list)")), Ok(Expr::Nil));
    assert_eq!(run_new(("(list 1)")), Ok(list1(Expr::Int(1))));
    assert_eq!(run_new(("(list 1 2)")), Ok(list2(Expr::Int(1), Expr::Int(2))));
}

#[test]
fn test_equal_p() {
    assert_eq!(run_new(("(equal? 1 1)")), Ok(Expr::Int(1)));
    assert_eq!(run_new(("(equal? 'sym 'sym)")), Ok(Expr::Int(1)));
    assert_eq!(run_new(("(equal? \"str\" \"str\")")), Ok(Expr::Int(1)));
    assert_eq!(run_new(("(equal? 1 \"str\")")), Ok(Expr::Nil));
    assert_eq!(run_new(("(equal? 'sym \"str\")")), Ok(Expr::Nil));
    assert_eq!(run_new(("(equal? (list \"str\") \"str\")")), Ok(Expr::Nil));
}


#[test]
fn test_string_to_number() {
    assert_eq!(run_new(("(string-to-number \"1\")")), Ok(Expr::Int(1)));
}

// TODO: test current-time-string

