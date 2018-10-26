extern crate kappa_lisp;
use kappa_lisp::env::Env;
use kappa_lisp::kappa_lisp::{init, run};

fn main() {
    let mut env = Env::new();
    init(&mut env).unwrap();
    run(
        &mut env,
        r"
(defun fib (n)
  (if (< n 2)
      1
      (+ (fib (- n 1)) (fib (- n 2)))))
",
    ).unwrap();
    let expr = run(&mut env, r" (fib 20) ").unwrap();
    println!("{}", expr);
}
