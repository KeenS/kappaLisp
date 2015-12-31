extern crate kappa_lisp;
use kappa_lisp::env::Env;
use kappa_lisp::read::read;
use kappa_lisp::eval::eval;


fn main() {
    let env = &mut Env::new();
    let expr = read("(current-time-string)");
    let expr_ = eval(env, &expr).unwrap();
    println!("{}", expr_);
}
