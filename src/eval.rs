use std::rc::Rc;
use std::ops::Deref;

use ::expr::{Expr, Type, Proc, Error as E, Result};
use ::env::Env;
use ::util::*;
#[cfg(test)]
use ::read::read;

fn bind_names(mut env: &mut Env, params: &Expr, args: &Expr) -> Result<()>{
    let mut phead = params;
    let mut ahead = args;
    let nil = &Expr::Nil;
    while phead != nil && ahead != nil {
        match (phead, ahead) {
            (&Expr::Cons(ref pcar, ref pcdr), &Expr::Cons(ref acar, ref acdr)) => {
                match pcar.deref() {
                    &Expr::Sym(ref name) => env.register(name.clone(), acar.deref().clone()),
                    pcar => return Err(E::Form(pcar.clone()))
                };   
                phead = pcdr.deref();
                ahead = acdr.deref();
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
        &Expr::Cons(ref params, ref body) => Ok(Proc::Lambda(params.clone(), Rc::new(Expr::Cons(Rc::new(Expr::Sym("progn".to_string())), body.clone())))),
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
    let nil = &Expr::Nil;
    let mut res = Expr::Nil;
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
    let tmp = list1(s);
    get_args!(&tmp, (s, Sym));
    env.fregister(s.clone(), f.clone());
    return Ok(Expr::Nil);
}

fn k_set(mut env: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, (s, Any) (e, Any));
    let s = try!(eval(env, s));
    let e = try!(eval(env, e));
    let tmp = list1(s);
    get_args!(&tmp, (s, Sym));
    env.register(s.clone(), e.clone());
    return Ok(Expr::Nil);
}


fn k_if(mut env: &mut Env, args: &Expr) -> Result<Expr> {
    // TODO: optional else clasue. Need optional argments.
    get_args!(args, (cnd, Any) (thn, Any) (els, Any));
    let res = try!(eval(env, cnd));
    if res != Expr::Nil {
        eval(env, thn)
    } else {
        eval(env, els)
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
        &Expr::Sym(ref name) => match env.find(&name.to_string()) {
            Ok(v) =>Ok(v.clone()),
            Err(m) => Err(m)
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


#[test]
fn test_atom(){
    assert_eq!(eval(&mut Env::new(), &read("1")), Ok(Expr::Int(1)));
    assert_eq!(eval(&mut Env::new(), &read("()")), Ok(Expr::Nil));
    assert_eq!(eval(&mut Env::new(), &read("")), Ok(Expr::EOF));
    assert_eq!(eval(&mut Env::new(), &read("\"string\"")), Ok(Expr::Str("string".to_string())));
}
// TODO: test `function`


#[test]
fn test_progn(){
    assert_eq!(eval(&mut Env::new(), &read("(progn 1 2)")), Ok(Expr::Int(2)));
    assert_eq!(eval(&mut Env::new(), &read("(progn (+ 1 2) (+ 2 3))")), Ok(Expr::Int(5)));
}

#[test]
fn test_lambda(){
    assert_eq!(eval(&mut Env::new(), &read("(lambda (x) x)")), Ok(Expr::Proc(Proc::Lambda(Rc::new(list1(Expr::Sym("x".to_string()))),
                                                                           Rc::new(list2(Expr::Sym("progn".to_string()), Expr::Sym("x".to_string())))))));
    assert_eq!(eval(&mut Env::new(), &read("((lambda (x) (+ x x)) 1)")), Ok(Expr::Int(2)))
}


#[test]
fn test_fset(){
    let mut env = Env::new();
    assert_eq!(eval(&mut env, &read("(fset 'add2 (lambda (x) (+ x 2)))")), Ok(Expr::Nil));
    assert_eq!(eval(&mut env, &read("(add2 2)")), Ok(Expr::Int(4)));
}


#[test]
fn test_set() {
    let mut env = Env::new();
    assert_eq!(eval(&mut env, &read("(set 'foo (+ 1 2 3))")), Ok(Expr::Nil));
    assert_eq!(eval(&mut env, &read("foo")), Ok(Expr::Int(6)));
    
}

#[test]
fn test_if() {
    assert_eq!(eval(&mut Env::new(), &read("(if () 1 2)")), Ok(Expr::Int(2)));
    assert_eq!(eval(&mut Env::new(), &read("(if 1 1 2)")), Ok(Expr::Int(1)));
}
