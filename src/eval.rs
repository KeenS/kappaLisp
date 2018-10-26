use std::ops::Deref;
use std::rc::Rc;

use env::Env;
use expr::{Error as E, Expr, Proc, Result, Type};
use util::*;

fn bind_name(env: &mut Env, name: &Expr, value: Expr) -> Result<()> {
    match name {
        &Expr::Sym(ref name) => Ok(env.register(name.deref().clone(), value)),
        name => return Err(E::Form(name.clone())),
    }
}

fn bind_names(env: &mut Env, params: &Expr, args: &Expr) -> Result<()> {
    let mut phead = params;
    let mut ahead = args;
    let mut in_optional = false;
    let optional = ksym("&optional");
    let rest = ksym("&rest");
    let nil = &knil();
    while phead != nil || ahead != nil {
        match phead {
            Expr::Cons(pcar, pcdr) => {
                let pcar = pcar.deref();
                if pcar == &optional {
                    in_optional = true;
                    phead = pcdr.deref();
                    continue;
                }
                if pcar == &rest {
                    match pcdr.deref() {
                        Expr::Cons(name, tail) => {
                            if tail.deref() != nil {
                                return Err(E::Form(tail.deref().clone()));
                            }
                            bind_name(env, name, ahead.clone())?;
                            return Ok(());
                        }
                        _ => return Err(E::Form(pcdr.deref().clone())),
                    }
                }

                match ahead {
                    Expr::Cons(acar, acdr) => {
                        bind_name(env, pcar, acar.deref().clone())?;
                        phead = pcdr.deref();
                        ahead = acdr.deref();
                    }
                    Expr::Nil => {
                        if !in_optional {
                            return Err(E::Form(pcar.clone()));
                        }
                        bind_name(env, pcar, knil())?;
                        phead = pcdr.deref();
                    }
                    _ => return Err(E::Form(args.clone())),
                }
            }
            _ => return Err(E::Form(args.clone())),
        }
    }
    Ok(())
}

pub fn funcall(env: &mut Env, f: &Proc, args: &Expr) -> Result<Expr> {
    match f {
        Proc::Prim(_, f) => f(env, args),
        Proc::Lambda(params, body) => {
            env.new_local();
            bind_names(env, params.deref(), args)?;
            let ret = eval(env, body.deref());
            env.end_local();
            ret
        }
        f => Err(E::NotFunction(kproc(f.clone()))),
    }
}

fn k_quote(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (sexp, Any));
    Ok(sexp.clone())
}

fn f_lambda(_: &mut Env, args: &Expr) -> Result<Proc> {
    match args {
        Expr::Cons(params, body) => Ok(klambda(
            params.deref().clone(),
            kcons(ksym("progn"), body.deref().clone()),
        )),
        _ => unreachable!(),
    }
}

fn k_lambda(env: &mut Env, args: &Expr) -> Result<Expr> {
    Ok(kproc(f_lambda(env, args)?))
}

fn k_feval(env: &mut Env, args: &Expr) -> Result<Expr> {
    match args {
        Expr::Cons(car, _) => Ok(kproc(feval(env, car.deref())?)),
        _ => unreachable!(),
    }
}

fn k_progn(env: &mut Env, args: &Expr) -> Result<Expr> {
    let mut head = args;
    let nil = &knil();
    let mut res = knil();
    while head != nil {
        match head {
            Expr::Cons(car, cdr) => {
                res = eval(env, car.deref())?;
                head = cdr.deref();
            }
            _ => return Err(E::Form(args.clone())),
        }
    }
    Ok(res)
}

fn k_fset(env: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (s, Any)(f, Any));
    let s = eval(env, s)?;
    let f = feval(env, f)?;
    let tmp = klist!(s);
    get_args!(&tmp, (s, Sym));
    env.fregister(s.deref().clone(), f.clone());
    return Ok(knil());
}

fn k_set(env: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (s, Any)(e, Any));
    let s = eval(env, s)?;
    let e = eval(env, e)?;
    let tmp = klist!(s);
    get_args!(&tmp, (s, Sym));
    env.register(s.deref().clone(), e.clone());
    return Ok(knil());
}

fn k_if(env: &mut Env, args: &Expr) -> Result<Expr> {
    // TODO: optional else clasue. Need optional argments.
    get_args!(args, (cnd, Any)(thn, Any) & optional(els, Any));
    let res = eval(env, cnd)?;
    if res != knil() {
        eval(env, thn)
    } else {
        match els {
            Some(els) => eval(env, els),
            None => Ok(knil()),
        }
    }
}

fn feval(env: &mut Env, expr: &Expr) -> Result<Proc> {
    match expr {
        Expr::Sym(sym) => match env.ffind(sym) {
            Ok(f) => Ok(f.clone()),
            Err(e) => Err(e),
        },
        Expr::Cons(op, rest) => {
            let op = op.deref();
            match op {
                Expr::Sym(sym) => match &sym[..] {
                    "lambda" => f_lambda(env, rest.deref()),
                    _ => Ok(Proc::Expr(Rc::new(eval(env, expr)?))),
                },
                _ => Err(E::NotFunction(expr.clone())),
            }
        }
        Expr::Proc(f) => Ok(f.clone()),
        _ => Err(E::NotFunction(expr.clone())),
    }
}

pub fn macro_fn(env: &mut Env, p: &Proc) -> Result<Option<Proc>> {
    match p {
        Proc::Expr(exp) => match exp.deref() {
            Expr::Cons(sym, f) => if sym.deref() == &ksym("macro") {
                Ok(Some(feval(env, f.deref())?))
            } else {
                Ok(None)
            },
            _ => Ok(None),
        },
        _ => Ok(None),
    }
}

pub fn eval(env: &mut Env, expr: &Expr) -> Result<Expr> {
    match expr {
        Expr::Nil | Expr::Str(_) | Expr::Int(_) | Expr::Float(_) | Expr::Proc(_) => {
            Ok(expr.clone())
        }
        Expr::Sym(name) => match env.find(&name.to_owned()) {
            Ok(v) => Ok(v.clone()),
            Err(m) => {
                if name.deref() == "t" {
                    Ok(ksym("t"))
                } else {
                    Err(m)
                }
            }
        },
        Expr::Cons(car, cdr) => {
            let car = car.deref();
            let cdr = cdr.deref();
            match car {
                Expr::Sym(sym) => {
                    match &sym[..] {
                        // Eval special forms first
                        "quote" => k_quote(env, cdr),
                        "function" => k_feval(env, cdr),
                        "lambda" => k_lambda(env, cdr),
                        "progn" => k_progn(env, cdr),
                        "fset" => k_fset(env, cdr),
                        "set" => k_set(env, cdr),
                        "if" => k_if(env, cdr),
                        _ => {
                            let f = feval(env, car)?;
                            match macro_fn(env, &f)? {
                                Some(f) => {
                                    let body = funcall(env, &f, cdr)?;
                                    eval(env, &body)
                                }
                                None => {
                                    let arg = f_map(env, &|env, x| eval(env, x), cdr)?;
                                    funcall(env, &f, &arg)
                                }
                            }
                        }
                    }
                }
                car => {
                    let f = feval(env, car)?;
                    let arg = f_map(env, &|env, x| eval(env, x), cdr)?;
                    funcall(env, &f, &arg)
                }
            }
        }
    }
}
