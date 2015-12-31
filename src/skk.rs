
use std::ops::Deref;


use expr::{Expr, Type};
use error::Error as E;
use env::{Env, Result};
use util::*;


#[cfg(test)]
use read::read;
#[cfg(test)]
use eval::eval;

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


pub fn init(mut env: &mut Env) {
    env.fregister("skk-calc", procedure("k_skk_calc", k_skk_calc));

}

#[test]
fn test_skk_calc(){
    let mut env = Env::new();
    env.init().unwrap();
    env.register("skk-num-list".to_string(), list2(Expr::Int(3), Expr::Int(2)));
    assert_eq!(eval(&mut env, &read("(skk-calc '+)")), Ok(Expr::Int(5)));
    assert_eq!(eval(&mut env, &read("(skk-calc '-)")), Ok(Expr::Int(1)));
    assert_eq!(eval(&mut env, &read("(skk-calc '*)")), Ok(Expr::Int(6)));
    assert_eq!(eval(&mut env, &read("(skk-calc '/)")), Ok(Expr::Int(1)));
}


#[test]
fn test_skk_gadget_units_conversion(){
    let mut env = Env::new();
    env.init().unwrap();
    assert_eq!(eval(&mut env, &read("(skk-gadget-units-conversion \"mile\" 1 \"km\")")),
               Ok(Expr::Float(1.6093)));
}
