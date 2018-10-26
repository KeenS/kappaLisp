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
    assert_eq!(read("nil"), Ok(knil()));
}

#[test]
fn test_read_int() {
    assert_eq!(read("0"), Ok(kint(0)));
    assert_eq!(read("10"), Ok(kint(10)));
    assert_eq!(read("-10"), Ok(kint(-10)));
    assert_eq!(read("+10"), Ok(kint(10)));
}

#[test]
fn test_read_float() {
    assert_eq!(read("0.0"), Ok(kfloat(0.0)));
    assert_eq!(read("10.123"), Ok(kfloat(10.123)));
    assert_eq!(read("-0.1"), Ok(kfloat(-0.1)));
    assert_eq!(read("-10.1"), Ok(kfloat(-10.1)));
    assert_eq!(read("+0.0"), Ok(kfloat(0.0)));
    assert_eq!(read("+10.0123"), Ok(kfloat(10.0123)));
}

#[test]
fn test_read_list() {
    assert_eq!(read("()"), Ok(knil()));
    assert_eq!(read("(1)"), Ok(klist!(kint(1))));
    assert_eq!(read("(1 2)"), Ok(klist!(kint(1), kint(2))));
    assert_eq!(read("(1 . 2)"), Ok(kcons(kint(1), kint(2))));
    assert_eq!(
        read("(1 2 . 3)"),
        Ok(kcons(kint(1), kcons(kint(2), kint(3))))
    );
    assert_eq!(
        read("'(1 (2 . 3))"),
        Ok(klist!(
            ksym("quote"),
            klist!(kint(1), kcons(kint(2), kint(3)))
        ))
    );
}

#[test]
fn test_read_symbol() {
    assert_eq!(read("symbol"), Ok(ksym("symbol")));
    assert_eq!(read("+symbol"), Ok(ksym("+symbol")));
    assert_eq!(read("-symbol"), Ok(ksym("-symbol")));
    assert_eq!(read("sym-bol"), Ok(ksym("sym-bol")));
    assert_eq!(read("symbol2"), Ok(ksym("symbol2")));
}

#[test]
fn test_read_string() {
    assert_eq!(read("\"string\""), Ok(kstr("string")));
    assert_eq!(read("\"str()ing\""), Ok(kstr("str()ing")));
    assert_eq!(read("\"str123ing\""), Ok(kstr("str123ing")));
    assert_eq!(read("\"()string\""), Ok(kstr("()string")));
    assert_eq!(read("\"123string\""), Ok(kstr("123string")));
    assert_eq!(read("(\"string\")"), Ok(klist!(kstr("string"))));
}

#[test]
fn test_read_quote() {
    assert_eq!(read("'1"), Ok(klist!(ksym("quote"), kint(1))));
    assert_eq!(read("'symbol"), Ok(klist!(ksym("quote"), ksym("symbol"))));
    assert_eq!(
        read("'\"string\""),
        Ok(klist!(ksym("quote"), kstr("string")))
    );
    assert_eq!(
        read("'(1 2)"),
        Ok(klist!(ksym("quote"), klist!(kint(1), kint(2))))
    )
}

#[test]
fn test_read_function() {
    assert_eq!(read("#'1"), Ok(klist!(ksym("function"), kint(1))));
    assert_eq!(
        read("#'symbol"),
        Ok(klist!(ksym("function"), ksym("symbol")))
    );
    assert_eq!(
        read("#'\"string\""),
        Ok(klist!(ksym("function"), kstr("string")))
    );
    assert_eq!(
        read("#'(1 2)"),
        Ok(klist!(ksym("function"), klist!(kint(1), kint(2))))
    )
}
