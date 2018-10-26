use base;
use datetime;
pub use env::Env;
pub use eval::eval;
pub use expr::{Expr, Proc, Result};
pub use read::read;
use skk;
use stdlib;

pub fn init(env: &mut Env) -> Result<()> {
    base::init(env)?;
    datetime::init(env)?;
    stdlib::init(env)?;
    skk::init(env)?;
    Ok(())
}

pub fn run(env: &mut Env, sexp: &str) -> Result<Expr> {
    let expr = read(sexp);
    eval(env, &expr)
}

pub fn run_new(sexp: &str) -> Result<Expr> {
    let mut env = Env::new();
    init(&mut env)?;
    let expr = read(sexp);
    eval(&mut env, &expr)
}
