use std::ops::Deref;

use env::Env;
use eval::funcall;
use expr::{Error as E, Expr, Kfloat, Kint, Result, Type};
use util::*;

// since rust's macro cannot treat binop, work around macro is needed.
macro_rules! expr {
    ($e:expr) => {
        $e
    };
}

macro_rules! def_arith_op {
    ($name: ident, $op: tt, $init: expr) => {
        pub fn $name(env: &mut Env, args: &Expr) -> Result<Expr> {
            let (init, args) = match args {
                Expr::Cons(hd, tl) => match tl.deref() {
                    tl @ &Expr::Cons(_, _) => (hd.deref().clone(), tl),
                    _ => ($init, args),
                },
                args => ($init, args),
            };
            f_foldl(
                env,
                &|_, x, y| match (x, y) {
                    (&Expr::Int(x), &Expr::Int(y)) => Ok(kint(expr!(x $op y))),
                    (&Expr::Float(x), &Expr::Int(y)) => Ok(kfloat(expr!(x $op (y as Kfloat)))),
                    (&Expr::Int(x), &Expr::Float(y)) => Ok(kfloat(expr!((x as Kfloat) $op y))),
                    (&Expr::Float(x), &Expr::Float(y)) => Ok(kfloat(expr!(x $op y))),
                    (&Expr::Int(_), y) => Err(E::Type(Type::Int, y.clone())),
                    (x, _) => Err(E::Type(Type::Int, x.clone())),
                },
                &init,
                args,
            )
        }
    };
}

def_arith_op!(k_add, +, kint(0));
def_arith_op!(k_sub, -, kint(0));
def_arith_op!(k_mul, *, kint(1));
def_arith_op!(k_div, /, kint(1));

// FIXME: accept multiple arguments
macro_rules! def_arith_cmp {
    ($name: ident, $op: tt) => {
        pub fn $name(_: &mut Env, args: &Expr) -> Result<Expr> {
            get_args!(args, (x, Int)(y, Int));
            Ok(kbool(expr!(x $op y)))
        }
    };
}

def_arith_cmp!(k_gt, >);
def_arith_cmp!(k_ge, >=);
def_arith_cmp!(k_lt, <);
def_arith_cmp!(k_le, <=);
def_arith_cmp!(k_eq, ==);
def_arith_cmp!(k_neq, !=);

pub fn k_concat(env: &mut Env, args: &Expr) -> Result<Expr> {
    let res = f_foldl(
        env,
        &|_, acc, x| match (acc, x) {
            (&Expr::Str(ref acc), &Expr::Str(ref x)) => Ok(kstr(format!("{}{}", acc, x))),
            (_, y) => Err(E::Type(Type::Str, y.clone())),
        },
        &kstr(""),
        &args,
    );
    Ok(res?.clone())
}

pub fn k_funcall(env: &mut Env, args: &Expr) -> Result<Expr> {
    match args {
        &Expr::Cons(ref f, ref args) => match f.deref() {
            &Expr::Proc(ref f) => funcall(env, f, args.deref()),
            f => Err(E::NotFunction(f.clone())),
        },
        args => Err(E::Form(args.clone())),
    }
}

pub fn k_cons(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (car, Any)(cdr, Any));
    Ok(kcons(car.clone(), cdr.clone()))
}

pub fn k_car(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, ((car, _), Cons));
    Ok(car.clone())
}

pub fn k_cdr(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, ((_, cdr), Cons));
    Ok(cdr.clone())
}

pub fn k_equal_p(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (x, Any)(y, Any));
    if x == y {
        Ok(ksym("t"))
    } else {
        Ok(knil())
    }
}

pub fn k_string_to_number(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (s, Str));
    match s.parse() {
        // TODO: handle float case
        Ok(i) => Ok(Expr::Int(i)),
        Err(_) => Err(E::InvalidArgument(args.clone())),
    }
}

pub fn k_substring(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (s, Str) & optional(start, Int)(end, Int));
    let len = s.len();
    let ilen = len as Kint;
    let start = start.unwrap_or(0);
    let end = end.unwrap_or(ilen);
    if 0 <= start && start <= end && end < ilen {
        let start = start as usize;
        let end = end as usize;
        Ok(kstr((&s[start..end]).to_owned()))
    } else {
        Err(E::InvalidArgument(args.clone()))
    }
}

pub fn init(env: &mut Env) -> Result<()> {
    env.fregister("+", kprim("k_add", k_add));
    env.fregister("-", kprim("k_sub", k_sub));
    env.fregister("/", kprim("k_div", k_div));
    env.fregister("*", kprim("k_mul", k_mul));
    env.fregister(">", kprim("k_gt", k_gt));
    env.fregister(">=", kprim("k_ge", k_ge));
    env.fregister("<", kprim("k_lt", k_lt));
    env.fregister("<=", kprim("k_le", k_le));
    env.fregister("=", kprim("k_eq", k_eq));
    env.fregister("/=", kprim("k_neq", k_neq));
    env.fregister("concat", kprim("k_concat", k_concat));
    env.fregister("funcall", kprim("k_funcall", k_funcall));
    env.fregister("cons", kprim("k_cons", k_cons));
    env.fregister("car", kprim("k_car", k_car));
    env.fregister("cdr", kprim("k_cdr", k_cdr));
    env.fregister("equalp", kprim("k_equal_p", k_equal_p));
    env.fregister(
        "string-to-number",
        kprim("k_string_to_number", k_string_to_number),
    );
    env.fregister("substring", kprim("k_substring", k_substring));
    env.register("t", ksym("t"));
    Ok(())
}
