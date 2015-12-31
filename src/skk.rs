extern crate time;

use std::ops::Deref;


use expr::{Expr, Type};
use error::Error as E;
use env::{Env, Result};
use util::*;
#[cfg(test)]
use read::read;
#[cfg(test)]
use eval::eval;

pub fn k_current_time_string(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args);
    let now = time::now();
    Ok(Expr::Str(format!("{}", now.ctime())))
}

pub fn k_skk_calc(mut env: &mut Env, args: &Expr) -> Result<Expr> {
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

pub fn k_skk_gadget_units_conversion(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (base_unit, Str) (v, Int) (target_unit, Str));
    // (* v (cdr (assoc target_unit (cdr (assoc base skk-units-alist)))))
    // ("mile" ("km" . 1.6093)
    //         ("yard" . 1760))

    //     ("yard" ("feet" . 3)
    //      ("cm" . 91.44))

    //     ("feet" ("inch" . 12)
    //      ("cm" . 30.48))

    //     ("inch" ("feet" . 0.5)
    //      ("cm" . 2.54))
    Ok(Expr::Nil)
}


#[test]
fn test_skk_calc(){
    let mut env = Env::new();
    env.init().unwrap();
    env.register("skk-num-list".to_string(), list2(Expr::Int(3), Expr::Int(2)));
    println!("{:?}", eval(&mut Env::new(), &read("(skk-calc '+)")));
    assert_eq!(eval(&mut env, &read("(skk-calc '+)")), Ok(Expr::Int(5)));
    assert_eq!(eval(&mut env, &read("(skk-calc '-)")), Ok(Expr::Int(1)));
    assert_eq!(eval(&mut env, &read("(skk-calc '*)")), Ok(Expr::Int(6)));
    assert_eq!(eval(&mut env, &read("(skk-calc '/)")), Ok(Expr::Int(1)));
}
