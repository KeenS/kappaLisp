pub use ::expr::{Result, Expr, Proc};
pub use ::eval::eval;
pub use ::env::Env;
pub use ::read::read;
use ::base;
use ::stdlib;
use ::datetime;
use ::skk;
    

pub fn init(mut env: &mut Env) -> Result<()> {
    try!(base::init(&mut env));
    try!(datetime::init(&mut env));
    try!(stdlib::init(&mut env));
    try!(skk::init(&mut env));
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
