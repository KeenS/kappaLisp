extern crate time;
use self::time::Timespec;

use std::ops::Deref;

use env::Env;
use expr::{Error as E, Expr, Kint, Result, Type};
use util::*;

const LOWER_BITS: i64 = 16;

pub fn datetime_info_to_timespec(args: &Expr) -> Result<Timespec> {
    get_args!(args, (hi, Int) (lo, Int) (nsec, Int) (_, Int));
    let hi = hi as i64;
    let lo = lo as i64;
    let sec = (hi << LOWER_BITS) + lo;
    Ok(Timespec {
        sec: sec,
        nsec: nsec as i32,
    })
}

pub fn k_current_time(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args);
    let time::Timespec { sec, nsec } = time::get_time();
    let hi = sec >> LOWER_BITS;
    let lo = sec & ((1 << LOWER_BITS) - 1);
    Ok(klist!(hi as Kint, lo as Kint, nsec as Kint, 0))
}

pub fn k_current_time_string(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, &optional (specified_time, Any) (_, Any));
    let now = match specified_time {
        None => time::now(),
        Some(st) => time::at(datetime_info_to_timespec(st)?),
    };
    Ok(kstr(format!("{}", now.ctime())))
}

pub fn init(env: &mut Env) -> Result<()> {
    env.fregister(
        "current-time-string",
        kprim("k_current_time_string", k_current_time_string),
    );
    env.fregister("current-time", kprim("k_current_time", k_current_time));
    Ok(())
}
