extern crate time;

use std::ops::Deref;

use ::expr::{Expr, Type, Kint, Result, Error as E};
use ::env::{Env};
use ::util::*;

pub fn k_current_time(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args);
    let nbits = 16;
    let time::Timespec{sec, nsec} = time::get_time();
    let hi  = sec >> nbits;
    let lo = sec & ((1 << nbits) - 1);
    Ok(klist!(hi as Kint, lo as Kint, nsec as Kint, 0))
}

pub fn k_current_time_string(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, &optional (specified_time, Any) (_, Any));
    let nbits = 16;
    let now = match specified_time {
        None => time::now(),
        Some(t) => {
            get_args!(t, (hi, Int) (lo, Int) (nsec, Int) (_, Int));
            let hi = hi as i64;
            let lo = lo as i64;
            let sec = (hi <<nbits) + lo;
            time::at(time::Timespec{sec: sec, nsec: nsec as i32})
        }
    };
    Ok(Expr::Str(format!("{}", now.ctime())))
}

pub fn init(mut env: &mut Env) -> Result<()>{
    env.fregister("current-time-string", kprim("k_current_time_string", k_current_time_string));
    env.fregister("current-time", kprim("k_current_time", k_current_time));
    Ok(())
}


