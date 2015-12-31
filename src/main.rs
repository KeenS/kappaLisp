extern crate kappa_lisp;
use kappa_lisp::kappa_lisp::run_new;


fn main() {
    let expr_ = run_new("(current-time-string)").unwrap();
    println!("{}", expr_);
}
