pub use ::expr::{Result, Expr};
pub use ::eval::eval;
pub use ::env::Env;
pub use ::read::read;
use ::base;
use ::stdlib;
use ::skk;
    

pub fn init(mut env: &mut Env) -> Result<()> {
    base::init(&mut env).unwrap();
    stdlib::init(&mut env).unwrap();
    skk::init(&mut env).unwrap();
    Ok(())
}
    

pub fn run(mut env: &mut Env, sexp: &str) -> Result<Expr> {
    let expr = read(sexp);
    eval(env, &expr)
}

pub fn run_new(sexp: &str) -> Result<Expr> {
    let mut env = Env::new();
    try!(init(&mut env));
    let expr = read(sexp);
    eval(&mut env, &expr)
}
