extern crate kappa_lisp;
use kappa_lisp::{run_new};
use kappa_lisp::util::*;


#[test]
fn test_assoc() {
    assert_eq!(run_new("(cdr (assoc 'two '((one . 1) (two . 2))))"), Ok(kint(2)));
    
}
