use std::ops::Deref;

use ::expr::{Expr, Type, Error as E, Result};
use ::env::Env;
use ::util::*;


pub fn k_skk_calc(env: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (op, Sym));
    let skk_num_list = try!(env.find(&"skk-num-list".to_string())).clone();
    get_args!(&skk_num_list, (x, Int) (y, Int));
    let res = match &op[..] {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        op => return //Err(format!("unknown operator {}", op))
                       Err(E::User(format!("unknown operator {}", op)))
    };
    Ok(Expr::Int(res))
}


pub fn init(mut env: &mut Env) -> Result<()>{
    env.fregister("skk-calc", procedure("k_skk_calc", k_skk_calc));
    Ok(())
}
