extern crate time;
use std::ops::Deref;

use ::expr::{Expr, Type, Kfloat, Result, Error as E};
use ::env::{Env};
use ::util::*;
use ::eval::funcall;

// since rust's macro cannot treat binop, work around macro is needed.
macro_rules! expr {
    ($e:expr) => {
        $e
    }
}

macro_rules! def_arith_op {
    ($name: ident, $op: tt, $init: expr) => {
        pub fn $name(mut env: &mut Env, args: &Expr) -> Result<Expr> {
            let (init, args) = match args {
                &Expr::Cons(ref hd, ref tl) => match tl.deref() {
                    tl @ &Expr::Cons(_, _) => (hd.deref().clone(), tl),
                    _ => ($init, args)
                },
                args => ($init, args)
            };
            f_foldl(env, &|_, x, y| match (x, y) {
                (&Expr::Int(x), &Expr::Int(y)) => Ok(Expr::Int(expr!(x $op y))),
                (&Expr::Float(x), &Expr::Int(y)) => Ok(Expr::Float(expr!(x $op (y as Kfloat)))),
                (&Expr::Int(x), &Expr::Float(y)) => Ok(Expr::Float(expr!((x as Kfloat) $op y))),
                (&Expr::Float(x), &Expr::Float(y)) => Ok(Expr::Float(expr!(x $op y))),
                (&Expr::Int(_), y) => Err(E::Type(Type::Int, y.clone())),
                (x, _) => Err(E::Type(Type::Int, x.clone())),
                    
            }, &init, args)

        }
    }
}

def_arith_op!(k_add, +, Expr::Int(0));
def_arith_op!(k_sub, -, Expr::Int(0));
def_arith_op!(k_mul, *, Expr::Int(1));
def_arith_op!(k_div, /, Expr::Int(1));

pub fn k_concat(mut env: &mut Env, args: &Expr) -> Result<Expr> {
    let res = f_foldl(env, &|_, acc, x| match (acc, x) {
        (&Expr::Str(ref acc), &Expr::Str(ref x)) => Ok(Expr::Str(format!("{}{}",acc, x))),
        (_, y) => Err(E::Type(Type::Str, y.clone()))
    }
                      , &Expr::Str("".to_string()), &args);
    Ok(try!(res).clone())
    
}


pub fn k_funcall(mut env: &mut Env, args: &Expr) -> Result<Expr> {
    match args {
        &Expr::Cons(ref f, ref args) => match f.deref() {
            &Expr::Proc(ref f) => funcall(env, f , args.deref()),
            f => Err(E::NotFunction(f.clone()))
        },
        args => Err(E::Form(args.clone()))
    }
}

pub fn k_cons(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (car, Any) (cdr, Any));
    Ok(cons(car.clone(), cdr.clone()))
}

pub fn k_car(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, ((car, _), Cons));
    Ok(car.clone())
}

pub fn k_cdr(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, ((_, cdr), Cons));
    Ok(cdr.clone())

}

pub fn k_list(_: &mut Env, args: &Expr) -> Result<Expr> {
    Ok(args.clone())
}

pub fn k_equal_p(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (x, Any) (y, Any));
    if x == y {
        // TODO: return `t`
        Ok(Expr::Int(1))
    } else {
        Ok(Expr::Nil)
    }
        
}

pub fn k_string_to_number(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (s, Str));
    match s.parse() {
        // TODO: handle float case
        Ok(i) => Ok(Expr::Int(i)),
        Err(_) => Err(E::InvalidArgument(args.clone()))
    }
}


pub fn k_current_time_string(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args);
    let now = time::now();
    Ok(Expr::Str(format!("{}", now.ctime())))
}


pub fn k_substring(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (s, Str) (start, Int) (end, Int));
    let len = s.len();
    if 0 <= start && start <= end && end < (len as isize) {
        let start = start as usize;
        let end = end as usize;
        Ok(Expr::Str((&s[start..end]).to_string()))
    } else {
        Err(E::InvalidArgument(args.clone()))
    }
}

pub fn init(mut env: &mut Env) -> Result<()>{
    env.fregister("+",       procedure("k_add", k_add));
    env.fregister("-",       procedure("k_sub", k_sub));
    env.fregister("/",       procedure("k_div", k_div));
    env.fregister("*",       procedure("k_mul", k_mul));
    env.fregister("concat",  procedure("k_concat", k_concat));
    env.fregister("funcall", procedure("k_funcall", k_funcall));
    env.fregister("cons",    procedure("k_cons", k_cons));
    env.fregister("car",     procedure("k_car", k_car));
    env.fregister("cdr",     procedure("k_cdr", k_cdr));
    env.fregister("list",    procedure("k_list",k_list));
    env.fregister("equal?",  procedure("k_equal_p", k_equal_p));
    env.fregister("string-to-number", procedure("k_string_to_number", k_string_to_number));
    env.fregister("substring", procedure("k_substring", k_substring));
    env.fregister("current-time-string", procedure("k_current_time_string", k_current_time_string));
    Ok(())
}


