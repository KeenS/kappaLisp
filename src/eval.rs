use std::rc::Rc;
use std::ops::Deref;

use expr::{Expr,Prim, Proc};
use error::Error as E;
use env::{Env, Result};
use skk;
#[cfg(test)]
use read::read;

fn f_foldl<F>(mut env: &mut Env, f: &F, init: &Expr, args: &Expr) -> Result<Expr>
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

fn f_foldr<F>(mut env: &mut Env, f: &F, init: &Expr, args: &Expr) -> Result<Expr>
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

fn f_map<F>(mut env: &mut Env, f: &F, list: &Expr) -> Result<Expr>
    where F: Fn(&mut Env, &Expr) -> Result<Expr>{
    f_foldr(env, &|env, acc, x| Ok(Expr::cons(try!(f(env, x)), acc.clone()))
                 , &Expr::Nil, list)
}

// fn f_iter<F>(mut env: &mut Env, f: &F, list: &Expr) -> Result<Expr>
//     where F: Fn(&mut Env, Expr) -> Result<()>{
//     f_foldr(env, &|env, _, x| {try!(f(env,x.clone())); Ok(Expr::Nil)}
//                  , Expr::Nil, list)
// }


// since rust's macro cannot treat binop, work around macro is needed.
macro_rules! expr {
    ($e:expr) => {
        $e
    }
}

macro_rules! def_arith_op {
    ($name: ident, $op: tt, $init: expr) => {
        fn $name(mut env: &mut Env, args: &Expr) -> Result<Expr> {
            let (init, args) = match args {
                &Expr::Cons(ref hd, ref tl) => match tl.deref() {
                    tl @ &Expr::Cons(_, _) => (hd.deref().clone(), tl),
                    _ => ($init, args)
                },
                args => ($init, args)
            };
            f_foldl(env, &|_, x, y| match (x, y) {
                (&Expr::Int(x), &Expr::Int(y)) => Ok(Expr::Int(expr!(x $op y))),
                (&Expr::Int(_), y) => Err(E::Type("int".to_string(), y.clone())),
                (x, _) => Err(E::Type("int".to_string(), x.clone())),
                    
            }, &init, args)

        }
    }
}

def_arith_op!(k_add, +, Expr::Int(0));
def_arith_op!(k_sub, -, Expr::Int(0));
def_arith_op!(k_mul, *, Expr::Int(1));
def_arith_op!(k_div, /, Expr::Int(1));

macro_rules! get_args_one {
    ($v:expr, int) => (
        match $v {
            &Expr::Int(x) => Ok(x),
            hd => Err(E::Type("int".to_string(), hd.clone()))
        }
    );
    ($v:expr, str) => (
        match $v {
            &Expr::Str(ref x) => Ok(x),
            hd => Err(E::Type("string".to_string(), hd.clone()))

        }
    );
    ($v:expr, sym) => (
        match $v {
            &Expr::Sym(ref x) => Ok(x),
            hd => Err(E::Type("symbol".to_string(), hd.clone()))
        }
    );
    ($v:expr, nil) => (
        match $v {
            &Expr::Nil => Ok(()),
            hd => Err(E::Type("nil".to_string(), hd.clone()))
        }
    );
    ($v:expr, cons) => (
        match $v {
            &Expr::Cons(ref car, ref cdr) => Ok((car.deref(), cdr.deref())),
            hd => Err(E::Type("cons".to_string(), hd.clone()))
        }
    );
    ($v:expr, fun) => (
        match $v {
            &Expr::Proc(ref p) => Ok(p.deref()),
            hd => Err(E::Type("function".to_string(), hd.clone()))
        }
    );
    ($v:expr, any) => (
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


fn k_concat(mut env: &mut Env, args: &Expr) -> Result<Expr> {
    let res = f_foldl(env, &|_, acc, x| match (acc, x) {
        (&Expr::Str(ref acc), &Expr::Str(ref x)) => Ok(Expr::Str(format!("{}{}",acc, x))),
        (_, y) => Err(E::Type("string".to_string(), y.clone()))
    }
                      , &Expr::Str("".to_string()), &args);
    Ok(try!(res).clone())
    
}


fn k_funcall(mut env: &mut Env, args: &Expr) -> Result<Expr> {
    match args {
        &Expr::Cons(ref f, ref args) => match f.deref() {
            &Expr::Proc(ref f) => funcall(env, f , args.deref()),
            f => Err(E::NotFunction(f.clone()))
        },
        args => Err(E::Form(args.clone()))
    }
}

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


fn k_car(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, ((car, _), cons));
    Ok(car.clone())
}

fn k_cdr(_: &mut Env, args: &Expr) -> Result<Expr> {
    get_args!(args, ((_, cdr), cons));
    Ok(cdr.clone())

}
fn funcall(mut env: &mut Env, f: &Proc, args: &Expr) -> Result<Expr> {
    match f {
        &Proc::Prim(prim) => {
            match prim {
                Prim::Add => k_add(env, args),
                Prim::Sub => k_sub(env, args),
                Prim::Div => k_div(env, args),
                Prim::Mul => k_mul(env, args),
                Prim::Concat => k_concat(env, args),
                Prim::Funcall => k_funcall(env, args),
                Prim::Car => k_car(env, args),
                Prim::Cdr => k_cdr(env, args),
                Prim::CurrentTimeString => skk::k_current_time_string(env, args),
                Prim::SkkCalc => skk::k_skk_calc(env, args),
                Prim::SkkGadgetUnitsConversion  =>  skk::k_skk_gadget_units_conversion(env, args)
            }
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
    get_args!(args, (sexp, any));
    Ok(sexp.clone())
}

fn k_feval(mut env: &mut Env, args: &Expr) -> Result<Expr> {
    match args {
        &Expr::Cons(ref car, _) => Ok(Expr::Proc(try!(feval(env, car.deref())))),
        _ => unreachable!()
    }
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
    get_args!(args, (s, any) (f, any));
    let s = try!(eval(env, s));
    let f = try!(feval(env, f));
    let tmp = Expr::cons(s, Expr::Nil);
    get_args!(&tmp, (s, sym));
    env.fregister(s.clone(), f.clone());
    return Ok(Expr::Nil);
}

fn k_if(mut env: &mut Env, args: &Expr) -> Result<Expr> {
    // TODO: optional else clasue. Need optional argments.
    get_args!(args, (cnd, any) (thn, any) (els, any));
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
                    "quote" => k_quote(env, cdr),
                    "function" => k_feval(env, cdr),
                    "lambda" => k_lambda(env, cdr),
                    "progn" => k_progn(env, cdr),
                    "fset" => k_fset(env, cdr),
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

#[test]
fn test_add(){
    assert_eq!(eval(&mut Env::new(), &read("(+)")), Ok(Expr::Int(0)));
    assert_eq!(eval(&mut Env::new(), &read("(+ 1)")), Ok(Expr::Int(1)));
    assert_eq!(eval(&mut Env::new(), &read("(+ 1 2)")), Ok(Expr::Int(3)));
    assert_eq!(eval(&mut Env::new(), &read("(+ 1 2 3)")), Ok(Expr::Int(6)));
}

#[test]
fn test_sub(){
    assert_eq!(eval(&mut Env::new(), &read("(-)")), Ok(Expr::Int(0)));
    assert_eq!(eval(&mut Env::new(), &read("(- 1)")), Ok(Expr::Int(-1)));
    assert_eq!(eval(&mut Env::new(), &read("(- 1 2)")), Ok(Expr::Int(-1)));
    assert_eq!(eval(&mut Env::new(), &read("(- 1 2 3)")), Ok(Expr::Int(-4)));
}

#[test]
fn test_mul(){
    assert_eq!(eval(&mut Env::new(), &read("(*)")), Ok(Expr::Int(1)));
    assert_eq!(eval(&mut Env::new(), &read("(* 1)")), Ok(Expr::Int(1)));
    assert_eq!(eval(&mut Env::new(), &read("(* 1 2)")), Ok(Expr::Int(2)));
    assert_eq!(eval(&mut Env::new(), &read("(* 1 2 3)")), Ok(Expr::Int(6)));
}

#[test]
fn test_div(){
    assert_eq!(eval(&mut Env::new(), &read("(/)")), Ok(Expr::Int(1)));
    assert_eq!(eval(&mut Env::new(), &read("(/ 1)")), Ok(Expr::Int(1)));
    assert_eq!(eval(&mut Env::new(), &read("(/ 3 2)")), Ok(Expr::Int(1)));
    assert_eq!(eval(&mut Env::new(), &read("(/ 3 2 1)")), Ok(Expr::Int(1)));
}

#[test]
fn test_nested_arith(){
    assert_eq!(eval(&mut Env::new(), &read("(/ (- (+ 1 (* 2 3)) 3) 2)")), Ok(Expr::Int(2)));
}

#[test]
fn test_concat(){
    assert_eq!(eval(&mut Env::new(), &read("(concat \"a\" \"b\" \"cd\")")), Ok(Expr::Str("abcd".to_string())))
}

#[test]
fn test_progn(){
    assert_eq!(eval(&mut Env::new(), &read("(progn 1 2)")), Ok(Expr::Int(2)));
    assert_eq!(eval(&mut Env::new(), &read("(progn (+ 1 2) (+ 2 3))")), Ok(Expr::Int(5)));
}

#[test]
fn test_lambda(){
    assert_eq!(eval(&mut Env::new(), &read("(lambda (x) x)")), Ok(Expr::Proc(Proc::Lambda(Rc::new(Expr::list1(Expr::Sym("x".to_string()))),
                                                                           Rc::new(Expr::list2(Expr::Sym("progn".to_string()), Expr::Sym("x".to_string())))))));
    assert_eq!(eval(&mut Env::new(), &read("((lambda (x) (+ x x)) 1)")), Ok(Expr::Int(2)))
}


#[test]
fn test_funcall(){
    assert_eq!(eval(&mut Env::new(), &read("(funcall #'+ 1 2)")), Ok(Expr::Int(3)));
    assert_eq!(eval(&mut Env::new(), &read("(funcall #'(lambda (x y) (* x y)) 1 2)")), Ok(Expr::Int(2)));
    assert_eq!(eval(&mut Env::new(), &read("(funcall (lambda (x y) (* x y)) 1 2)")), Ok(Expr::Int(2)))
}

#[test]
fn test_fset(){
    let mut env = Env::new();
    assert_eq!(eval(&mut env, &read("(fset 'add2 (lambda (x) (+ x 2)))")), Ok(Expr::Nil));
    assert_eq!(eval(&mut env, &read("(add2 2)")), Ok(Expr::Int(4)));
}
