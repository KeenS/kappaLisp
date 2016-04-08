#[macro_use]
extern crate kappa_lisp;
use kappa_lisp::{run_new};
use kappa_lisp::util::*;

#[test]
fn test_list() {
    assert_eq!(run_new("(list)"), Ok(knil()));
    assert_eq!(run_new("(list 1)"), Ok(klist!(kint(1))));
    assert_eq!(run_new("(list 1 2)"), Ok(klist!(kint(1), kint(2))));
}



#[test]
fn test_assoc() {
    assert_eq!(run_new("(cdr (assoc 'two '((one . 1) (two . 2))))"), Ok(kint(2)));
}
