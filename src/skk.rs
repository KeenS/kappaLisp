extern crate time;

use std::ops::Deref;

use datetime::datetime_info_to_timespec;
use env::Env;
use eval::{eval, funcall};
use expr::{Error as E, Expr, Result, Type};
use read::read_in;
use util::*;

pub fn k_skk_calc(env: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (op, Sym));
    let skk_num_list = env.find(&"skk-num-list".to_owned())?.clone();
    get_args!(&skk_num_list, (x, Int)(y, Int));
    let res = match &op[..] {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        op => return Err(E::User(format!("unknown operator {}", op))),
    };
    Ok(kint(res))
}

pub fn k_skk_current_date_1(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, &optional(specified_time, Any));
    // TODO: don't allocate month/wday table every time
    let mvec = vec![
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let wvec = vec!["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    let now = specified_time.map_or(Ok(time::now()), |st| {
        datetime_info_to_timespec(st).map(|tm| time::at(tm))
    });
    let now = now?;
    let year = (now.tm_year + 1900).to_string();
    let month = mvec[now.tm_mon as usize];
    let mday = now.tm_mday.to_string();
    let wday = wvec[now.tm_wday as usize];
    let hour = now.tm_hour.to_string();
    let min = now.tm_min.to_string();
    let sec = now.tm_sec.to_string();
    Ok(klist!(year, month, mday, wday, hour, min, sec))
}

pub fn k_skk_current_date(env: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, &optional(f, Proc)(format, Any)(and_time, Any));
    let date_information = k_skk_current_date_1(env, &knil())?;
    let nil = knil();
    let format = format.unwrap_or(&nil);
    let gengo = knil(); //or t
    let and_time = and_time.unwrap_or(&nil);
    match f {
        Some(f) => funcall(env, f, &klist!(date_information, format, gengo, and_time)),
        None => Ok(knil()),
    }
}

pub fn k_skk_default_current_date(_: &mut Env, args: &Expr) -> Result<Expr> {
    // get_args!(args,
    //           (date_information, Any)
    //           (format, Nullable Str)
    //           (num_type, Int)
    //           (gengo, Bool)
    //           (gengo_index, Nullable Int)
    //           (month_alist_index, Nullable Int)
    //           (dayofweek_alist_index, Nullable Int)
    //           &optional (and_time, Bool)
    //           );
    // get_args!(date_information,
    //           (year, Str)(month, Str)(day, Str)(day_of_week, Str)(hour, Str)(minute, Str)(second,
    //                                                                                       Str));
    // let res = match and_time {
    //     None | Some(false) => {
    //         let format = format.map_or("%s年%s月%s日(%s)", |s| &s);
    //         kstr(format)
    //     }
    //     _ => {
    //         let format = format.map_or("%s年%s月%s日(%s)%s時%s分%s秒", |s| &s);
    //         kstr(format)
    //     }
    // };
    Ok(knil())
}

pub fn init(mut env: &mut Env) -> Result<()> {
    env.fregister("skk-calc", kprim("k_skk_calc", k_skk_calc));
    env.fregister(
        "skk-current-date-1",
        kprim("k_skk_current_date_1", k_skk_current_date_1),
    );
    env.fregister(
        "skk-current-date",
        kprim("k_skk_current_date", k_skk_current_date),
    );
    let lisp = include_str!("skk.lisp");
    let mut input = lisp.chars().peekable();
    while let Some(e) = read_in(&mut input) {
        let _ = eval(&mut env, &e)?;
    }

    Ok(())
}
