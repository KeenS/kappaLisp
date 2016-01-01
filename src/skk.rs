extern crate time;

use std::ops::Deref;

use ::expr::{Expr, Type, Error as E, Result};
use ::eval::funcall;
use ::env::Env;
use ::util::*;
use ::datetime::datetime_info_to_timespec;


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
    Ok(kint(res))
}

pub fn k_skk_current_date_1(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, &optional (specified_time, Any));
    // TODO: don't allocate month/wday table every time
    let mvec = vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let wvec = vec!["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    let now = specified_time.map_or(Ok(time::now()), |st| datetime_info_to_timespec(st).map(|tm| time::at(tm)));
    let now = try!(now);
    let year = (now.tm_year + 1900).to_string();
    let month = mvec[now.tm_mon as usize];
    let mday = now.tm_mday.to_string();
    let wday = wvec[now.tm_wday as usize];
    let hour = now.tm_hour.to_string();
    let min = now.tm_min.to_string();
    let sec = now.tm_sec.to_string();
    Ok(klist!(year, month, mday, wday, hour, min, sec))
}


pub fn k_skk_current_date(mut env: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, &optional (f, Proc) (format, Any) (and_time, Any));
    let date_information = try!(k_skk_current_date_1(&mut env, &knil()));
    let nil = knil();
    let format = format.unwrap_or(&nil);
    let gengo = knil(); //or t
    let and_time = and_time.unwrap_or(&nil);
    match f {
        Some(f) => funcall(env, f, &klist!(date_information, format, gengo, and_time)),
        None => Ok(knil())
    }    
}

pub fn k_skk_default_current_date(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args,
              (date_information, Any)
              (format, Str)
              (num_type, Int)
              (gengo, Any)//bool
              (gengo_index, Any)
              (month_alist_index, Any)
              (dayofweek_alist_index, Any)
              &optional (and_time, Any) // bool
              );
    Ok(knil())
}


pub fn init(mut env: &mut Env) -> Result<()>{
    env.fregister("skk-calc", kprim("k_skk_calc", k_skk_calc));
    env.fregister("skk-current-date-1", kprim("k_skk_current_date_1", k_skk_current_date_1));
    env.fregister("skk-current-date", kprim("k_skk_current_date", k_skk_current_date));
    Ok(())
}
