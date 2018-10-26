#[macro_use]
extern crate kappa_lisp;
use kappa_lisp::read;
use kappa_lisp::util::*;

#[test]
fn test_read_empty() {
    //    assert_eq!(read(""), (keOF));
    //    assert_eq!(read("(a b"), (keOF));
}

#[test]
fn test_read_nil() {
    assert_eq!(read("nil"), (knil()));
}

#[test]
fn test_read_int() {
    assert_eq!(read("0"), (kint(0)));
    assert_eq!(read("10"), (kint(10)));
    assert_eq!(read("-10"), (kint(-10)));
    assert_eq!(read("+10"), (kint(10)));
}

#[test]
fn test_read_float() {
    assert_eq!(read("0.0"), (kfloat(0.0)));
    assert_eq!(read("10.123"), (kfloat(10.123)));
    assert_eq!(read("-0.1"), (kfloat(-0.1)));
    assert_eq!(read("-10.1"), (kfloat(-10.1)));
    assert_eq!(read("+0.0"), (kfloat(0.0)));
    assert_eq!(read("+10.0123"), (kfloat(10.0123)));
}

#[test]
fn test_read_list() {
    assert_eq!(read("()"), (knil()));
    assert_eq!(read("(1)"), (klist!(kint(1))));
    assert_eq!(read("(1 2)"), (klist!(kint(1), kint(2))));
    assert_eq!(read("(1 . 2)"), (kcons(kint(1), kint(2))));
    assert_eq!(read("(1 2 . 3)"), (kcons(kint(1), kcons(kint(2), kint(3)))));
    assert_eq!(
        read("'(1 (2 . 3))"),
        klist!(ksym("quote"), (klist!(kint(1), kcons(kint(2), kint(3)))))
    );
}

#[test]
fn test_read_symbol() {
    assert_eq!(read("symbol"), (ksym("symbol")));
    assert_eq!(read("+symbol"), (ksym("+symbol")));
    assert_eq!(read("-symbol"), (ksym("-symbol")));
    assert_eq!(read("sym-bol"), (ksym("sym-bol")));
    assert_eq!(read("symbol2"), (ksym("symbol2")));
}

#[test]
fn test_read_string() {
    assert_eq!(read("\"string\""), (kstr("string")));
    assert_eq!(read("\"str()ing\""), (kstr("str()ing")));
    assert_eq!(read("\"str123ing\""), (kstr("str123ing")));
    assert_eq!(read("\"()string\""), (kstr("()string")));
    assert_eq!(read("\"123string\""), (kstr("123string")));
    assert_eq!(read("(\"string\")"), (klist!(kstr("string"))));
}

#[test]
fn test_read_quote() {
    assert_eq!(read("'1"), (klist!(ksym("quote"), kint(1))));
    assert_eq!(read("'symbol"), (klist!(ksym("quote"), ksym("symbol"))));
    assert_eq!(read("'\"string\""), (klist!(ksym("quote"), kstr("string"))));
    assert_eq!(
        read("'(1 2)"),
        (klist!(ksym("quote"), klist!(kint(1), kint(2))))
    )
}

#[test]
fn test_read_function() {
    assert_eq!(read("#'1"), (klist!(ksym("function"), kint(1))));
    assert_eq!(read("#'symbol"), (klist!(ksym("function"), ksym("symbol"))));
    assert_eq!(
        read("#'\"string\""),
        (klist!(ksym("function"), kstr("string")))
    );
    assert_eq!(
        read("#'(1 2)"),
        (klist!(ksym("function"), klist!(kint(1), kint(2))))
    )
}
