extern crate kappa_lisp;
use kappa_lisp::kappa_lisp::{run_new, Expr};

#[test]
fn test_assoc() {
    assert_eq!(run_new(("(cdr (assoc 'two '((one . 1) (two . 2))))")), Ok(Expr::Int(2)));
    
}
