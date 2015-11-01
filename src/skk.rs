extern crate time;

use std::rc::Rc;
use std::ops::Deref;


#[macro_use]
use expr::Expr;
use read::read;
use env::Env;
use eval::eval;

pub fn k_current_time_string(mut env: &mut Env, args: Expr) -> Result<Expr, String> {
    get_args!(args);
    let now = time::now();
    Ok(Expr::Str(format!("{}", now.ctime())))
}

pub fn k_skk_calc(mut env: &mut Env, args: Expr) -> Result<Expr, String> {
    get_args!(args, (op, sym));
    let skk_num_list = try!(env.find(&"skk-num-list".to_string())).clone();
    get_args!(skk_num_list, (x, int) (y, int));
    let res = match &op[..] {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        op => return Err(format!("unknown operator {}", op))
    };
    Ok(Expr::Int(res))
}

// pub fn k_skk_gadget_units_conversion(mut env: &mut Env, args: Expr) -> Result<Expr, String> {
//     get_args!(args, (base_unit, sym), (v, int), (target_unit, sym),);

//     ("mile" ("km" . 1.6093)
//      ("yard" . 1760))

//         ("yard" ("feet" . 3)
//          ("cm" . 91.44))

//         ("feet" ("inch" . 12)
//          ("cm" . 30.48))

//         ("inch" ("feet" . 0.5)
//          ("cm" . 2.54))
// }


#[test]
fn test_skk_calc(){
    let mut env = Env::new();
    env.register("skk-num-list".to_string(), Expr::list2(Expr::Int(3), Expr::Int(2)));
    println!("{:?}", eval(&mut Env::new(), read("(skk-calc '+)")));
    assert!(eval(&mut env, read("(skk-calc '+)")) == Ok(Expr::Int(5)));
    assert!(eval(&mut env, read("(skk-calc '-)")) == Ok(Expr::Int(1)));
    assert!(eval(&mut env, read("(skk-calc '*)")) == Ok(Expr::Int(6)));
    assert!(eval(&mut env, read("(skk-calc '/)")) == Ok(Expr::Int(1)));
}
