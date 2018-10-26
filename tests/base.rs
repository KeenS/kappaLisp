#[macro_use]
extern crate kappa_lisp;
use kappa_lisp::run_new;
use kappa_lisp::util::*;

#[test]
fn test_funcall() {
    assert_eq!(run_new("(funcall #'+ 1 2)"), Ok(kint(3)));
    assert_eq!(
        run_new("(funcall #'(lambda (x y) (* x y)) 1 2)"),
        Ok(kint(2))
    );
    assert_eq!(run_new("(funcall (lambda (x y) (* x y)) 1 2)"), Ok(kint(2)))
}

#[test]
fn test_add() {
    assert_eq!(run_new("(+)"), Ok(kint(0)));
    assert_eq!(run_new("(+ 1)"), Ok(kint(1)));
    assert_eq!(run_new("(+ 1 2)"), Ok(kint(3)));
    assert_eq!(run_new("(+ 1 2 3)"), Ok(kint(6)));
    assert_eq!(run_new("(+ 1 2 3.0)"), Ok(kfloat(6.0)));
}

#[test]
fn test_sub() {
    assert_eq!(run_new("(-)"), Ok(kint(0)));
    assert_eq!(run_new("(- 1)"), Ok(kint(-1)));
    assert_eq!(run_new("(- 1 2)"), Ok(kint(-1)));
    assert_eq!(run_new("(- 1.0 2 3)"), Ok(kfloat(-4.0)));
}

#[test]
fn test_mul() {
    assert_eq!(run_new("(*)"), Ok(kint(1)));
    assert_eq!(run_new("(* 1)"), Ok(kint(1)));
    assert_eq!(run_new("(* 1 2.0)"), Ok(kfloat(2.0)));
    assert_eq!(run_new("(* 1 2 3)"), Ok(kint(6)));
}

#[test]
fn test_div() {
    assert_eq!(run_new("(/)"), Ok(kint(1)));
    assert_eq!(run_new("(/ 1)"), Ok(kint(1)));
    assert_eq!(run_new("(/ 3 2)"), Ok(kint(1)));
    assert_eq!(run_new("(/ 3 2.0)"), Ok(kfloat(1.5)));
    assert_eq!(run_new("(/ 3 2 1)"), Ok(kint(1)));
}

#[test]
fn test_gt() {
    assert_eq!(run_new("(> 1 2)"), Ok(kbool(false)));
    assert_eq!(run_new("(> 1 1)"), Ok(kbool(false)));
    assert_eq!(run_new("(> 2 1)"), Ok(kbool(true)));
    // assert_eq!(run_new("(> -1 1.0)"), Ok(kbool(false)));
    // assert_eq!(run_new("(> 1.0 -1)"), Ok(kbool(false)));
}

#[test]
fn test_ge() {
    assert_eq!(run_new("(>= 1 2)"), Ok(kbool(false)));
    assert_eq!(run_new("(>= 1 1)"), Ok(kbool(true)));
    assert_eq!(run_new("(>= 2 1)"), Ok(kbool(true)));
    // assert_eq!(run_new("(>= -1 1.0)"), Ok(kbool(false)));
    // assert_eq!(run_new("(>= 1.0 -1)"), Ok(kbool(false)));
}

#[test]
fn test_lt() {
    assert_eq!(run_new("(< 1 2)"), Ok(kbool(true)));
    assert_eq!(run_new("(< 1 1)"), Ok(kbool(false)));
    assert_eq!(run_new("(< 2 1)"), Ok(kbool(false)));
    // assert_eq!(run_new("(< -1 1.0)"), Ok(kbool(true)));
    // assert_eq!(run_new("(< 1.0 -1)"), Ok(kbool(false)));
}

#[test]
fn test_le() {
    assert_eq!(run_new("(<= 1 2)"), Ok(kbool(true)));
    assert_eq!(run_new("(<= 1 1)"), Ok(kbool(true)));
    assert_eq!(run_new("(<= 2 1)"), Ok(kbool(false)));
    // assert_eq!(run_new("(<= -1 1.0)"), Ok(kbool(true)));
    // assert_eq!(run_new("(<= 1.0 -1)"), Ok(kbool(false)));
}

#[test]
fn test_eq() {
    assert_eq!(run_new("(= 1 2)"), Ok(kbool(false)));
    assert_eq!(run_new("(= 1 1)"), Ok(kbool(true)));
    assert_eq!(run_new("(= 2 1)"), Ok(kbool(false)));
    // assert_eq!(run_new("(= -1 1.0)"), Ok(kbool(false)));
    // assert_eq!(run_new("(= -1 -1.0)"), Ok(kbool(true)));
    // assert_eq!(run_new("(= 1.0 -1)"), Ok(kbool(false)));
}

#[test]
fn test_neq() {
    assert_eq!(run_new("(/= 1 2)"), Ok(kbool(true)));
    assert_eq!(run_new("(/= 1 1)"), Ok(kbool(false)));
    assert_eq!(run_new("(/= 2 1)"), Ok(kbool(true)));
    // assert_eq!(run_new("(/= -1 1.0)"), Ok(kbool(true)));
    // assert_eq!(run_new("(/= -1 -1.0)"), Ok(kbool(false)));
    // assert_eq!(run_new("(/= 1.0 -1)"), Ok(kbool(true)));
}

#[test]
fn test_nested_arith() {
    assert_eq!(run_new("(/ (- (+ 1 (* 2 3)) 3) 2)"), Ok(kint(2)));
}

#[test]
fn test_concat() {
    assert_eq!(run_new("(concat \"a\" \"b\" \"cd\")"), Ok(kstr("abcd")))
}

#[test]
fn test_cons() {
    assert_eq!(run_new("(cons 1 2)"), Ok(kcons(kint(1), kint(2))));
    assert_eq!(run_new("(cons () 2)"), Ok(kcons(knil(), kint(2))));
}

#[test]
fn test_car() {
    assert_eq!(run_new("(car (cons 1 2))"), Ok(kint(1)));
    assert_eq!(run_new("(car (list 1 2))"), Ok(kint(1)));
}

#[test]
fn test_cdr() {
    assert_eq!(run_new("(cdr (cons 1 2))"), Ok(kint(2)));
    assert_eq!(run_new("(cdr (list 1 2))"), Ok(klist!(kint(2))));
}

#[test]
fn test_equal_p() {
    assert_eq!(run_new("(equalp 1 1)"), Ok(ksym("t")));
    assert_eq!(run_new("(equalp 'sym 'sym)"), Ok(ksym("t")));
    assert_eq!(run_new("(equalp \"str\" \"str\")"), Ok(ksym("t")));
    assert_eq!(run_new("(equalp 1 \"str\")"), Ok(knil()));
    assert_eq!(run_new("(equalp 'sym \"str\")"), Ok(knil()));
    assert_eq!(run_new("(equalp (list \"str\") \"str\")"), Ok(knil()));
}

#[test]
fn test_string_to_number() {
    assert_eq!(run_new("(string-to-number \"1\")"), Ok(kint(1)));
}

#[test]
fn test_substring() {
    assert_eq!(run_new("(substring \"abcdefg\" 0 3)"), Ok(kstr("abc")));
    assert_eq!(run_new("(substring \"abcdefg\" 4 6)"), Ok(kstr("ef")));
}

#[test]
fn test_t() {
    assert_eq!(run_new("t"), Ok(ksym("t")))
}
