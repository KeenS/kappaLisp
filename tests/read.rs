extern crate kappa_lisp;
use kappa_lisp::{read, Expr};
use kappa_lisp::util::{cons, list1, list2};


#[test]
fn test_read_empty(){
    assert_eq!(read(""), (Expr::EOF));
    assert_eq!(read("(a b"), (Expr::EOF));
}

#[test]
fn test_read_nil() {
    assert_eq!(read("nil"), (Expr::Nil));
}

#[test]
fn test_read_int() {
    assert_eq!(read("0"), (Expr::Int(0)));
    assert_eq!(read("10"), (Expr::Int(10)));
    assert_eq!(read("-10"), (Expr::Int(-10)));
    assert_eq!(read("+10"), (Expr::Int(10)));
}

#[test]
fn test_read_float() {
    assert_eq!(read("0.0"), (Expr::Float(0.0)));
    assert_eq!(read("10.123"), (Expr::Float(10.123)));
    assert_eq!(read("-0.1"), (Expr::Float(-0.1)));
    assert_eq!(read("-10.1"), (Expr::Float(-10.1)));
    assert_eq!(read("+0.0"), (Expr::Float(0.0)));
    assert_eq!(read("+10.0123"), (Expr::Float(10.0123)));
}

#[test]
fn test_read_list(){
    assert_eq!(read("()"), (Expr::Nil));
    assert_eq!(read("(1)"), (list1(Expr::Int(1))));
    assert_eq!(read("(1 2)"), (list2(Expr::Int(1), Expr::Int(2))));
    assert_eq!(read("(1 . 2)"), (cons(Expr::Int(1), Expr::Int(2))));
    assert_eq!(read("(1 2 . 3)"), (cons(Expr::Int(1), cons(Expr::Int(2), Expr::Int(3)))));
    assert_eq!(read("'(1 (2 . 3))"), list2(Expr::Sym("quote".to_string()), (list2(Expr::Int(1), cons(Expr::Int(2), Expr::Int(3))))));
}


#[test]
fn test_read_symbol(){
    assert_eq!(read("symbol"), (Expr::Sym("symbol".to_string())));
    assert_eq!(read("+symbol"), (Expr::Sym("+symbol".to_string())));
    assert_eq!(read("-symbol"), (Expr::Sym("-symbol".to_string())));
    assert_eq!(read("sym-bol"), (Expr::Sym("sym-bol".to_string())));
    assert_eq!(read("symbol2"), (Expr::Sym("symbol2".to_string())));
}


#[test]
fn test_read_string(){
    assert_eq!(read("\"string\""), (Expr::Str("string".to_string())));
    assert_eq!(read("\"str()ing\""), (Expr::Str("str()ing".to_string())));
    assert_eq!(read("\"str123ing\""), (Expr::Str("str123ing".to_string())));
    assert_eq!(read("\"()string\""), (Expr::Str("()string".to_string())));
    assert_eq!(read("\"123string\""), (Expr::Str("123string".to_string())));
    assert_eq!(read("(\"string\")"), (list1(Expr::Str("string".to_string()))));
}

#[test]
fn test_read_quote(){
    assert_eq!(read("'1"), (list2(Expr::Sym("quote".to_string()), Expr::Int(1))));
    assert_eq!(read("'symbol"), (list2(Expr::Sym("quote".to_string()), Expr::Sym("symbol".to_string()))));
    assert_eq!(read("'\"string\""), (list2(Expr::Sym("quote".to_string()), Expr::Str("string".to_string()))));
    assert_eq!(read("'(1 2)"), (list2(Expr::Sym("quote".to_string()), list2(Expr::Int(1), Expr::Int(2)))))
}

#[test]
fn test_read_function(){
    assert_eq!(read("#'1"), (list2(Expr::Sym("function".to_string()), Expr::Int(1))));
    assert_eq!(read("#'symbol"), (list2(Expr::Sym("function".to_string()), Expr::Sym("symbol".to_string()))));
    assert_eq!(read("#'\"string\""), (list2(Expr::Sym("function".to_string()), Expr::Str("string".to_string()))));
    assert_eq!(read("#'(1 2)"), (list2(Expr::Sym("function".to_string()), list2(Expr::Int(1), Expr::Int(2)))))
}
