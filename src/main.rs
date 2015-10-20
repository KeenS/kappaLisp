extern crate kappaLisp;
use kappaLisp::env::Env;
use kappaLisp::read::read;
use kappaLisp::eval::eval;


fn main() {
    let env = &mut kappaLisp::env::Env::new();
    let expr = read("(current-time-string)");
    let expr_ = eval(env, expr).unwrap();
    println!("{}", expr_);
}
