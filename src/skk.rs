extern crate time;

#[macro_use]
use expr::Expr;
use read::read;
use eval::eval;
use env::Env;


fn k_current_time_string(mut env: &mut Env, args: Expr) -> Expr {
    let now = time::now();
    Expr::Str(format!("{}", now.ctime()))
}
