use std::ops::Deref;
use std::rc::Rc;

use env::Env;
use expr::{Error as E, Expr, Kfloat, Kint, Proc, Result, Type};

#[inline]
pub fn kbool(b: bool) -> Expr {
    match b {
        true => ksym("t"),
        false => Expr::Nil,
    }
}

#[inline]
pub fn kint(i: Kint) -> Expr {
    Expr::Int(i)
}

#[inline]
pub fn kfloat(f: Kfloat) -> Expr {
    Expr::Float(f)
}

#[inline]
pub fn kcons(car: Expr, cdr: Expr) -> Expr {
    Expr::Cons(Rc::new(car), Rc::new(cdr))
}

#[inline]
pub fn knil() -> Expr {
    Expr::Nil
}

#[inline]
pub fn ksym<S: Into<String>>(s: S) -> Expr {
    Expr::Sym(Rc::new(s.into()))
}

#[inline]
pub fn kkw<S: Into<String>>(s: S) -> Expr {
    Expr::Keyword(Rc::new(s.into()))
}

#[inline]
pub fn kstr<S: Into<String>>(s: S) -> Expr {
    Expr::Str(Rc::new(s.into()))
}

#[inline]
pub fn kproc(p: Proc) -> Expr {
    Expr::Proc(p)
}

#[inline]
pub fn kmacro(p: Proc) -> Expr {
    kcons(ksym("macro"), Expr::Proc(p))
}

#[inline]
pub fn klambda(param: Expr, body: Expr) -> Proc {
    Proc::Lambda(Rc::new(param), Rc::new(body))
}

#[inline]
pub fn kprim<S: Into<String>, F: 'static + Fn(&mut Env, &Expr) -> Result<Expr> + Sized>(
    name: S,
    f: F,
) -> Proc {
    Proc::Prim(name.into(), Rc::new(f))
}

pub fn is_macro(exp: &Proc) -> bool {
    match exp {
        Proc::Expr(exp) => match exp.deref() {
            Expr::Cons(sym, _) => sym.deref() == &kstr("macro"),
            _ => false,
        },
        _ => false,
    }
}

pub fn car(cons: &Expr) -> Result<Expr> {
    match cons {
        Expr::Cons(car, _) => Ok(car.deref().clone()),
        arg => Err(E::Type(Type::Cons, arg.clone())),
    }
}

pub fn cdr(cons: &Expr) -> Result<Expr> {
    match cons {
        Expr::Cons(_, cdr) => Ok(cdr.deref().clone()),
        arg => Err(E::Type(Type::Cons, arg.clone())),
    }
}

#[macro_export]
macro_rules! klist {
    ($car: expr, $($cdr: expr), *) => (
        kcons($crate::expr::Expr::from($car), klist!($($cdr),*))
    );
    ($car: expr) => (
        kcons($crate::expr::Expr::from($car), knil())
    );
    () => (
        knil()
    );
}

macro_rules! get_args_one {
    ($v:expr, Nullable $($ident: tt)+) => (
        match $v {
            &Expr::Nil => Ok(None),
            v => Ok(Some(get_args_one!(v, $($ident)+)?))
        }
    );
    ($v:expr, Int) => (
        match $v {
            &Expr::Int(x) => Ok(x),
            hd => Err(E::Type(Type::Int, hd.clone()))
        }
    );

    ($v:expr, Float) => {
        match $v {
            &Expr::Float(x) => Ok(x),
            hd => Err(E::Type(Type::Float, hd.clone()))
        }
    };
    ($v:expr, Str) => (
        match $v {
            &Expr::Str(ref x) => Ok(x),
            hd => Err(E::Type(Type::Str, hd.clone()))

        }
    );
    ($v:expr, Sym) => (
        match $v {
            &Expr::Sym(ref x) => Ok(x),
            hd => Err(E::Type(Type::Sym, hd.clone()))
        }
    );
    ($v:expr, Nil) => (
        match $v {
            &Expr::Nil => Ok(()),
            hd => Err(E::Type(Type::Nil, hd.clone()))
        }
    );
    ($v:expr, Bool) => (
        match $v {
            &Expr::Nil => Ok(false),
            _ => Ok(true)
        }
    );
    ($v:expr, Cons) => (
        match $v {
            &Expr::Cons(ref car, ref cdr) => Ok((car.deref(), cdr.deref())),
            hd => Err(E::Type(Type::Cons, hd.clone()))
        }
    );
    ($v:expr, Proc) => (
        match $v {
            &Expr::Proc(ref p) => Ok(p),
            hd => Err(E::Type(Type::Proc, hd.clone()))
        }
    );
    ($v:expr, Any) => (
        match $v {
            hd => if true {
                Ok(hd)
            } else {
                unreachable!()
            }
        };
    )
}

macro_rules! gen_pattern {
    (($var: pat, $($ident: tt)*) $($other:tt) *) => (
        ($var, gen_pattern!($($other)*))
    );
    (&optional ($var: pat, $($ident: tt)*) $($other:tt) *) => (
        ($var, gen_pattern!($($other)*))
    );
    () => (())
}

macro_rules! gen_match {
    ($args: expr, ($var: pat, $($ident: tt)+) $($other:tt) *) =>
        (
            match $args {
                Expr::Cons(hd, tl) => {
                    let v = get_args_one!(hd.deref(), $($ident)+)?;
                    (v, gen_match!(tl.deref(), $($other)*))
                },
                Expr::Nil => return Err(E::ArityShort),
                args => return Err(E::InvalidArgument(args.clone()))
            };
        );
    ($args: expr, &optional ($var: pat, $($ident: tt)+) $($other:tt) *) =>
        (
            match $args {
                Expr::Cons(hd, tl) => {
                    let v = get_args_one!(hd.deref(), $($ident)+)?;
                    (Some(v), gen_match!(tl.deref(), &optional $($other)*))
                },
                Expr::Nil => {
                    (None, gen_match!($args, &optional $($other)*))
                },
                args => return Err(E::InvalidArgument(args.clone()))
            };
        );
    ($args: expr, &optional) => (
        match $args {
            Expr::Nil => (),
            _ => return Err(E::ArityExceed)
        }
    );
    ($args: expr, ) => (
        match $args {
            Expr::Nil => (),
            _ => return Err(E::ArityExceed)
        }
    );
}

#[macro_export]
macro_rules! get_args {
    ($args: expr, $($other:tt) *) =>
        (
            let gen_pattern!($($other)*) = gen_match!($args, $($other)*);
        ) ;
    ($args: expr, ) => (
        let () = gen_match!($args,);
    );
    ($args: expr) => (
        let () = gen_match!($args,);
    );
}

pub fn f_foldl<F>(env: &mut Env, f: &F, init: &Expr, args: &Expr) -> Result<Expr>
where
    F: Fn(&mut Env, &Expr, &Expr) -> Result<Expr>,
{
    let mut res = init.clone();
    let mut head = args;
    let nil = &Expr::Nil;
    while head != nil {
        match head {
            Expr::Cons(car, cdr) => {
                res = f(env, &res, car)?;
                head = cdr;
            }
            _ => return Err(E::InvalidArgument(args.clone())),
        }
    }
    Ok(res)
}

// fn f_reverse(mut env: &mut Env, args: &Expr) -> Result<Expr> {
//     f_foldl(env, &|_, acc, x| Ok(Expr::Cons(Rc::new(x.clone()), Rc::new(acc))), Expr::Nil, args)
// }

pub fn f_foldr<F>(env: &mut Env, f: &F, init: &Expr, args: &Expr) -> Result<Expr>
where
    F: Fn(&mut Env, &Expr, &Expr) -> Result<Expr>,
{
    match args {
        Expr::Nil => Ok(init.clone()),
        Expr::Cons(car, cdr) => {
            let v = f_foldr(env, f, init, cdr)?;
            f(env, &v, car)
        }
        args => Err(E::InvalidArgument(args.clone())),
    }
}

pub fn f_map<F>(env: &mut Env, f: &F, list: &Expr) -> Result<Expr>
where
    F: Fn(&mut Env, &Expr) -> Result<Expr>,
{
    f_foldr(
        env,
        &|env, acc, x| Ok(kcons(f(env, x)?, acc.clone())),
        &knil(),
        list,
    )
}

// fn f_iter<F>(mut env: &mut Env, f: &F, list: &Expr) -> Result<Expr>
//     where F: Fn(&mut Env, Expr) -> Result<()>{
//     f_foldr(env, &|env, _, x| {try!(f(env,x.clone())); Ok(Expr::Nil)}
//                  , Expr::Nil, list)
// }
