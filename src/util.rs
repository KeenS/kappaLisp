use std::rc::Rc;
use std::ops::Deref;

use expr::{Expr, Type};
use error::Error as E;
use env::{Env, Result};


pub fn cons(car: Expr, cdr: Expr) -> Expr {
    Expr::Cons(Rc::new(car), Rc::new(cdr))
}

pub fn list1(a1: Expr) -> Expr {
    cons(a1, Expr::Nil)
}
pub fn list2(a1: Expr, a2: Expr) -> Expr {
    cons(a1, list1(a2))
}

pub fn car(cons: &Expr) -> Result<Expr> {
    match cons {
        &Expr::Cons(ref car, _) => Ok(car.deref().clone()),
        arg => Err(E::Type(Type::Cons, cons.clone()))
    }
}

pub fn cdr(cons: &Expr) -> Result<Expr> {
    match cons {
        &Expr::Cons(_, ref cdr) => Ok(cdr.deref().clone()),
        arg => Err(E::Type(Type::Cons, cons.clone()))
    }
}




macro_rules! get_args_one {
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
    ($v:expr, Cons) => (
        match $v {
            &Expr::Cons(ref car, ref cdr) => Ok((car.deref(), cdr.deref())),
            hd => Err(E::Type(Type::Cons, hd.clone()))
        }
    );
    ($v:expr, Proc) => (
        match $v {
            &Expr::Proc(ref p) => Ok(p.deref()),
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
    (($var: pat, $ident: tt) $($other:tt) *) => (
        ($var, gen_pattern!($($other)*))
            );
    () => (())
}

macro_rules! gen_match {
    ($args: expr, ($var: pat, $ident: tt) $($other:tt) *) =>
        (
            match $args {
                &Expr::Cons(ref hd, ref tl) => {
                    let v = try!(get_args_one!(hd.deref(), $ident));
                    (v, gen_match!(tl.deref(), $($other)*))
                },
                &Expr::Nil => return Err(E::ArityShort),
                args => return Err(E::InvalidArgument(args.clone()))
            };
            );
    ($args: expr, ) => (
        match $args {
            &Expr::Nil => (),
            _ => return Err(E::ArityExceed)
        }
        );
}

#[macro_export]
macro_rules! get_args {
    ($args: expr, ($var: pat, $ident: tt) $($other:tt) *) =>
        (
            let gen_pattern!(($var, $ident) $($other)*) = gen_match!($args, ($var, $ident) $($other)*)
            ) ;
    ($args: expr, ) => (
        let () = gen_match!($args,)
        );
    ($args: expr) => (
        let () = gen_match!($args,)
        );
}



pub fn f_foldl<F>(mut env: &mut Env, f: &F, init: &Expr, args: &Expr) -> Result<Expr>
    where F: Fn(&mut Env, &Expr, &Expr) -> Result<Expr>{
    let mut res = init.clone();
    let mut head = args;
    let nil = &Expr::Nil;
    while head != nil {
        match head {
            &Expr::Cons(ref car, ref cdr) => {
                res = try!(f(env, &res, car));
                head = cdr;
            }
            _ => return Err(E::InvalidArgument(args.clone()))
        }
    }
    Ok(res)
}

// fn f_reverse(mut env: &mut Env, args: &Expr) -> Result<Expr> {
//     f_foldl(env, &|_, acc, x| Ok(Expr::Cons(Rc::new(x.clone()), Rc::new(acc))), Expr::Nil, args)
// }

pub fn f_foldr<F>(mut env: &mut Env, f: &F, init: &Expr, args: &Expr) -> Result<Expr>
    where F: Fn(&mut Env, &Expr, &Expr) -> Result<Expr>{
    match args {
        &Expr::Nil => Ok(init.clone()),
        &Expr::Cons(ref car, ref cdr) => {
            let v = try!(f_foldr(env, f, init, cdr));
            f(env, &v, car)
        }
        args => Err(E::InvalidArgument(args.clone()))
    }
}

pub fn f_map<F>(mut env: &mut Env, f: &F, list: &Expr) -> Result<Expr>
    where F: Fn(&mut Env, &Expr) -> Result<Expr>{
    f_foldr(env, &|env, acc, x| Ok(cons(try!(f(env, x)), acc.clone()))
                 , &Expr::Nil, list)
}

// fn f_iter<F>(mut env: &mut Env, f: &F, list: &Expr) -> Result<Expr>
//     where F: Fn(&mut Env, Expr) -> Result<()>{
//     f_foldr(env, &|env, _, x| {try!(f(env,x.clone())); Ok(Expr::Nil)}
//                  , Expr::Nil, list)
// }

