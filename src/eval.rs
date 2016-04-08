use std::rc::Rc;
use std::ops::Deref;

use ::expr::{Expr, Type, Proc, Error as E, Result};
use ::env::Env;
use ::util::*;

fn bind_name(mut env: &mut Env, name: &Expr, value: Expr) -> Result<()> {
    match name {
        &Expr::Sym(ref name) => Ok(env.register(name.clone(), value)),
        name => return Err(E::Form(name.clone()))
    }
}

fn bind_names(mut env: &mut Env, params: &Expr, args: &Expr) -> Result<()>{
    let mut phead = params;
    let mut ahead = args;
    let mut in_optional = false;
    let optional = ksym("&optional");
    let rest = ksym("&rest");
    let nil = &knil();
    // matching exact
    while phead != nil || ahead != nil {
        match phead {
            &Expr::Cons(ref pcar, ref pcdr) => {
                let pcar = pcar.deref();
                if pcar == &optional {
                    in_optional = true;
                    phead = pcdr.deref();
                    continue;
                }
                if pcar == &rest {
                    match pcdr.deref() {
                        &Expr::Cons(ref name, ref tail) => {
                            if tail.deref() != nil {return Err(E::Form(tail.deref().clone()))}
                            try!(bind_name(env, name, ahead.clone()));
                            return Ok(());
                        },
                        _ => return Err(E::Form(pcdr.deref().clone()))
                    }
                }

                match ahead {
                    &Expr::Cons(ref acar, ref acdr) => {
                        try!(bind_name(env, pcar, acar.deref().clone()));
                        phead = pcdr.deref();
                        ahead = acdr.deref();
                    },
                    &Expr::Nil => {
                        if ! in_optional {
                            return Err(E::Form(pcar.clone()))
                        }
                        try!(bind_name(env, pcar, knil()));
                        phead = pcdr.deref();
                    },
                    _ => return Err(E::Form(args.clone()))
                }
            },
            _ => return Err(E::Form(args.clone()))
        }
    };
    Ok(())
}


pub fn funcall(mut env: &mut Env, f: &Proc, args: &Expr) -> Result<Expr> {
    match f {
        &Proc::Prim(_, ref f) => {
            f(env, args)
        },
        &Proc::Lambda(ref params, ref body) => {
            env.new_local();
            try!(bind_names(env, params.deref(), args));
            let ret = eval(env, body.deref());
            env.end_local();
            ret
        }
    }
}

fn k_quote(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (sexp, Any));
    Ok(sexp.clone())
}

fn f_lambda(_: &mut Env, args: &Expr) -> Result<Proc> {
    match args {
        &Expr::Cons(ref params, ref body) =>
            Ok(klambda(params.deref().clone(),
                       kcons(ksym("progn"), body.deref().clone()))),
        _ => unreachable!()
    }
}

fn k_lambda(mut env: &mut Env, args: &Expr) -> Result<Expr> {
    Ok(Expr::Proc(try!(f_lambda(env, args))))
}


fn k_feval(mut env: &mut Env, args: &Expr) -> Result<Expr> {
    match args {
        &Expr::Cons(ref car, _) => Ok(Expr::Proc(try!(feval(env, car.deref())))),
        _ => unreachable!()
    }
}


fn k_progn(mut env: &mut Env, args: &Expr) -> Result<Expr> {
    let mut head = args;
    let nil = &knil();
    let mut res = knil();
    while head != nil {
        match head {
            &Expr::Cons(ref car, ref cdr) => {
                res = try!(eval(env, car.deref()));
                head = cdr.deref();
            },
            _ => return Err(E::Form(args.clone()))
        }
    };
    Ok(res)
}

fn k_fset(mut env: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (s, Any) (f, Any));
    let s = try!(eval(env, s));
    let f = try!(feval(env, f));
    let tmp = klist!(s);
    get_args!(&tmp, (s, Sym));
    env.fregister(s.clone(), f.clone());
    return Ok(knil());
}

fn k_set(mut env: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (s, Any) (e, Any));
    let s = try!(eval(env, s));
    let e = try!(eval(env, e));
    let tmp = klist!(s);
    get_args!(&tmp, (s, Sym));
    env.register(s.clone(), e.clone());
    return Ok(knil());
}


fn k_if(mut env: &mut Env, args: &Expr) -> Result<Expr> {
    // TODO: optional else clasue. Need optional argments.
    get_args!(args, (cnd, Any) (thn, Any) &optional (els, Any));
    let res = try!(eval(env, cnd));
    if res != knil() {
        eval(env, thn)
    } else {
        match els {
            Some(els) =>  eval(env, els),
            None => Ok(knil())
        }
    }
}

fn feval(mut env: &mut Env, expr: &Expr) -> Result<Proc> {
    match expr {
        &Expr::Sym(ref sym) => {
            match env.ffind(sym) {
                Ok(f) => Ok(f.clone()),
                Err(e) => Err(e)
            }
        },
        &Expr::Cons(ref op, ref rest) => {
            let op = op.deref();
            match op {
                &Expr::Sym(ref sym) => match &sym[..] {
                    "lambda" => f_lambda(env, rest.deref()),
                    _ => Err(E::NotFunction(expr.clone()))
                },
                _ => Err(E::NotFunction(expr.clone()))
            }
        }
        &Expr::Proc(ref f) => Ok(f.clone()),
        _ => Err(E::NotFunction(expr.clone()))
    }
}


pub fn eval(mut env: &mut Env, expr: &Expr) -> Result<Expr> {
    match expr {
        &Expr::Nil |
        &Expr::EOF |
        &Expr::Str(_) |
        &Expr::Int(_) |
        &Expr::Float(_) |
        &Expr::Proc(_) => Ok(expr.clone()),
        &Expr::Sym(ref name) => match env.find(&name.to_owned()) {
            Ok(v) =>Ok(v.clone()),
            Err(m) => if name == "t" {
                Ok(ksym("t"))
            }else {
                Err(m)
            }
        },
        &Expr::Cons(ref car, ref cdr) => {
            let car = car.deref();
            let cdr = cdr.deref();
            match car {
                &Expr::Sym(ref sym) => match &sym[..] {
                    // Eval special forms first
                    "quote" => k_quote(env, cdr),
                    "function" => k_feval(env, cdr),
                    "lambda" => k_lambda(env, cdr),
                    "progn" => k_progn(env, cdr),
                    "fset" => k_fset(env, cdr),
                    "set"  => k_set(env, cdr),
                    "if" => k_if(env, cdr),
                    _ => {
                        let f = try!(feval(env, car));
                        let arg = try!(f_map(env, &|env, x| eval(env, x), cdr));
                        funcall(env, &f, &arg)
                    }
                },
                car => {
                    let f = try!(feval(env, car));
                    let arg = try!(f_map(env, &|env, x| eval(env, x), cdr));
                    funcall(env, &f, &arg)
                }
            }    
        }
    }
}

