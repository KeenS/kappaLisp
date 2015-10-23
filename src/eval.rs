use std::rc::Rc;
use std::ops::Deref;

use expr::{Expr,Prim};
use env::Env;
use read::read;
use skk;

fn f_foldl<F>(mut env: &mut Env, f: &F, init: Expr, args: &Expr) -> Result<Expr, String>
    where F: Fn(&mut Env, Expr, &Expr) -> Result<Expr, String>{
    let mut res = init;
    let mut head = args;
    let nil = &Expr::Nil;
    while head != nil {
        match head {
            &Expr::Cons(ref car, ref cdr) => {
                res = try!(f(env, res, car));
                head = cdr;
            }
            _ => return Err(format!("invalid argument {:?} to function", args.clone()))
        }
    }
    Ok(res)
}

fn f_reverse(mut env: &mut Env, args: &Expr) -> Result<Expr, String> {
    f_foldl(env, &|_, acc, x| Ok(Expr::Cons(Rc::new(x.clone()), Rc::new(acc))), Expr::Nil, args)
}

fn f_foldr<F>(mut env: &mut Env, f: &F, init: Expr, args: &Expr) -> Result<Expr, String>
    where F: Fn(&mut Env, Expr, &Expr) -> Result<Expr, String>{
    match args {
        &Expr::Nil => Ok(init),
        &Expr::Cons(ref car, ref cdr) => {
            let v = try!(f_foldr(env, f, init, cdr));
            f(env, v, car)
        }
        _ => Err(format!("invalid argument {:?} to function", args))
    }
}

fn f_map<F>(mut env: &mut Env, f: &F, list: &Expr) -> Result<Expr, String>
    where F: Fn(&mut Env, Expr) -> Result<Expr, String>{
    f_foldr(env, &|env, acc, x| Ok(Expr::cons(try!(f(env, x.clone())), acc))
                 , Expr::Nil, list)
}

fn f_iter<F>(mut env: &mut Env, f: &F, list: &Expr) -> Result<Expr, String>
    where F: Fn(&mut Env, Expr) -> Result<(), String>{
    f_foldr(env, &|env, acc, x| {try!(f(env,x.clone())); Ok(Expr::Nil)}
                 , Expr::Nil, list)
}


// since rust's macro cannot treat binop, work around macro is needed.
macro_rules! expr {
    ($e:expr) => {
        $e
    }
}

macro_rules! def_arith_op {
    ($name: ident, $op: tt, $init: expr) => {
        fn $name(mut env: &mut Env, args: Expr) -> Result<Expr, String> {
            let (init, args) = match args {
                Expr::Cons(ref hd, ref tl) => match tl.deref() {
                    tl @ &Expr::Cons(_, _) => (hd.deref().clone(), tl.clone()),
                    tl => ($init, Expr::cons(hd.deref().clone(), tl.clone()))
                },
                args => ($init, args)
            };
            f_foldl(env, &|_, x, y| match (x, y) {
                (Expr::Int(x), &Expr::Int(y)) => Ok(Expr::Int(expr!(x $op y))),
                (x, y) => Err(format!("non int args {:?} and {:?} are given to $op", x, y))
            }, init, &args)

        }
    }
}

def_arith_op!(k_add, +, Expr::Int(0));
def_arith_op!(k_sub, -, Expr::Int(0));
def_arith_op!(k_mul, *, Expr::Int(1));
def_arith_op!(k_div, /, Expr::Int(1));

macro_rules! get_args_one {
    ($v:expr, int) => {
        match $v {
            &Expr::Int(x) => Ok(x.clone()),
            hd => Err(format!("nil expected but got {:?}", hd))
        }
    };
    ($v:expr, str) => {
        match $v {
            &Expr::Str(ref x) => Ok(x.clone()),
            hd => Err(format!("string expected but got {:?}", hd))
        }
    };
    ($v:expr, sym) => {
        match $v {
            &Expr::Sym(ref x) => Ok(x.clone()),
            hd => Err(format!("symbol expected but got {:?}", hd))
        }
    };
    ($v:expr, nil) => {
        match $v {
            &Expr::Int(x) => Ok(x.clone()),
            hd => Err(format!("nil expected but got {:?}", hd))
        }
    };
}

#[macro_export]
macro_rules! get_args {
    ($args: expr, ($var: ident, $ident: tt), $($other:tt), *) =>
        (
            let $var = match $args {
                Expr::Cons(ref hd, ref tl) => try!(get_args_one!(hd.deref(), $ident)),
                Expr::Nil => return Err(format!("invalid number of arguments")),
                args => return Err(format!("invalid argument to function call"))
            };
            get_args!(tl.deref.clone(), $($other),*)
        )
    ;
    ($args: expr, ) => ();
}


fn k_double(mut env: &mut Env, args: Expr) -> Result<Expr, String> {
    get_args!(args, (x, int),);
    Ok(Expr::Int(x*2))
}


fn k_concat(mut env: &mut Env, args: Expr) -> Result<Expr, String> {
    f_foldl(env, &|_, acc, x| match (acc, x) {
        (Expr::Str(ref acc), &Expr::Str(ref x)) => Ok(Expr::Str(format!("{}{}",acc, x))),
        (_, y) => Err(format!("non string args {:?} are given to concat", y))
    }
            , Expr::Str("".to_string()), &args)
}


fn k_funcall(mut env: &mut Env, args: Expr) -> Result<Expr, String> {
    match args {
        Expr::Cons(f, args) => funcall(env, f.deref(), args.deref().clone()),
        args => Err(format!("illeagal form of funcall {:?}", args))
    }
}

fn bind_names(mut env: &mut Env, params: Expr, args: Expr) -> Result<(), String>{
    let mut phead = &params;
    let mut ahead = &args;
    let nil = &Expr::Nil;
    while phead != nil && ahead != nil {
        match (phead, ahead) {
            (&Expr::Cons(ref pcar, ref pcdr), &Expr::Cons(ref acar, ref acdr)) => {
                match pcar.deref().clone() {
                    Expr::Sym(ref name) => env.register(name.clone(), acar.deref().clone()),
                    pcar => return Err(format!("illeagal form of params {:?}", pcar))
                };
                phead = pcdr.deref();
                ahead = acdr.deref();
            },
            _ => return Err(format!("ileagal form of funcall"))
        }
    };
    Ok(())
}


fn funcall(mut env: &mut Env, f: &Expr, args: Expr) -> Result<Expr, String> {
    match f {
        &Expr::FLambda(prim) => {
            match prim {
                Prim::Add => k_add(env, args),
                Prim::Sub => k_sub(env, args),
                Prim::Div => k_div(env, args),
                Prim::Mul => k_mul(env, args),
                Prim::Concat => k_concat(env, args),
                Prim::Funcall => k_funcall(env, args),
                Prim::CurrentTimeString => skk::k_current_time_string(env, args),
                Prim::SkkCalc => skk::k_skk_calc(env, args)
            }
        },
        &Expr::Lambda(ref params, ref body) => {
            env.new_local();
            try!(bind_names(env, params.deref().clone(), args));
            let ret = eval(env, body.deref().clone());
            env.end_local();
            ret
        },
        f => Err(format!("{:?} is not a function", f))
    }
}

fn k_quote(mut env: &mut Env, args: Expr) -> Result<Expr, String> {
    match args {
        Expr::Cons(ref car, ref nil) => Ok(car.deref().clone()),
        _ => Err(format!("unreachable"))
    }
}

fn k_feval(mut env: &mut Env, args: Expr) -> Result<Expr, String> {
    match args {
        Expr::Cons(ref car, ref nil) => feval(env, car.deref().clone()),
        _ => Err(format!("unreachable"))
    }
}

fn k_lambda(mut env: &mut Env, args: Expr) -> Result<Expr, String> {
    match args {
        Expr::Cons(params, body) => Ok(Expr::Lambda(params, Rc::new(Expr::Cons(Rc::new(Expr::Sym("progn".to_string())), body)))),
        _ => Err(format!("unreachable"))
    }
}

fn k_progn(mut env: &mut Env, args: Expr) -> Result<Expr, String> {
    let mut head = &args;
    let nil = &Expr::Nil;
    let mut res = Expr::Nil;
    while head != nil {
        match head {
            &Expr::Cons(ref car, ref cdr) => {
                res = try!(eval(env, car.deref().clone()));
                head = cdr.deref();
            },
            _ => return Err(format!("invalid form of progn or lambda"))
        }
    };
    Ok(res)
}

fn feval(mut env: &mut Env, expr: Expr) -> Result<Expr, String> {
    match expr {
        Expr::Sym(ref sym) => {
            match env.ffind(sym) {
                Some(f) => Ok(f.clone()),
                None => {
                    let prim = match &sym[..] {
                        "+" => Prim::Add,
                        "-" => Prim::Sub,
                        "/" => Prim::Div,
                        "*" =>Prim:: Mul,
                        "concat" => Prim::Concat,
                        "funcall" => Prim::Funcall,
                        "current-time-string" => Prim::CurrentTimeString,
                        "skk-calc" => Prim::SkkCalc,
                        _ => return Err(format!("function {:?} not found", sym))
                    };
                    Ok(Expr::FLambda(prim))
                }
            }
        },
        Expr::Cons(ref op, ref rest) => {
            let op = op.deref();
            match op {
                &Expr::Sym(ref sym) => match &sym[..] {
                    "lambda" => k_lambda(env, rest.deref().clone()),
                    op => Err(format!("invalid expression '({:?} {:?})' found at function potision", op, rest))
                },
                op => Err(format!("invalid expression '({:?} {:?})' found at function potision", op, rest))
            }
        }
        Expr::Lambda(_, _) => Ok(expr),
        x => Err(format!("{:?} is not a function", x))
    }
}


pub fn eval(mut env: &mut Env, expr: Expr) -> Result<Expr, String> {
    match expr {
        Expr::Nil |
        Expr::EOF |
        Expr::Str(_) |
        Expr::Int(_) |
        Expr::Lambda(_, _) |
        Expr::FLambda(_) => Ok(expr),
        Expr::Sym(ref name) => match env.find(&name.to_string()) {
            Some(v) => Ok(v.clone()),
            None => Err(format!("variable {:?} not found", name))
        }
,
        Expr::Cons(ref car, ref cdr) => {
            let car = car.deref();
            let cdr = cdr.deref();
            match car {
                &Expr::Sym(ref sym) => match &sym[..] {
                    "quote" => k_quote(env, cdr.clone()),
                    "function" => k_feval(env, cdr.clone()),
                    "lambda" => k_lambda(env, cdr.clone()),
                    "progn" => k_progn(env, cdr.clone()),
                    _ => {
                        let f = try!(feval(env, car.clone()));
                        let arg = try!(f_map(env, &|env, x| eval(env, x), cdr));
                        funcall(env, &f, arg)                        
                    }
                },
                car => {
                    let f = try!(feval(env, car.clone()));
                    let arg = try!(f_map(env, &|env, x| eval(env, x), cdr));
                    funcall(env, &f, arg)
                }
            }    
        }
    }
}


#[test]
fn test_atom(){
    assert!(eval(&mut Env::new(), read("1")) == Ok(Expr::Int(1)));
    assert!(eval(&mut Env::new(), read("()")) == Ok(Expr::Nil));
    assert!(eval(&mut Env::new(), read("")) == Ok(Expr::EOF));
    assert!(eval(&mut Env::new(), read("\"string\"")) == Ok(Expr::Str("string".to_string())));
}

#[test]
fn test_add(){
    assert!(eval(&mut Env::new(), read("(+)")) == Ok(Expr::Int(0)));
    assert!(eval(&mut Env::new(), read("(+ 1)")) == Ok(Expr::Int(1)));
    assert!(eval(&mut Env::new(), read("(+ 1 2)")) == Ok(Expr::Int(3)));
    assert!(eval(&mut Env::new(), read("(+ 1 2 3)")) == Ok(Expr::Int(6)));
}

#[test]
fn test_sub(){
    assert!(eval(&mut Env::new(), read("(-)")) == Ok(Expr::Int(0)));
    assert!(eval(&mut Env::new(), read("(- 1)")) == Ok(Expr::Int(-1)));
    assert!(eval(&mut Env::new(), read("(- 1 2)")) == Ok(Expr::Int(-1)));
    assert!(eval(&mut Env::new(), read("(- 1 2 3)")) == Ok(Expr::Int(-4)));
}

#[test]
fn test_mul(){
    assert!(eval(&mut Env::new(), read("(*)")) == Ok(Expr::Int(1)));
    assert!(eval(&mut Env::new(), read("(* 1)")) == Ok(Expr::Int(1)));
    assert!(eval(&mut Env::new(), read("(* 1 2)")) == Ok(Expr::Int(2)));
    assert!(eval(&mut Env::new(), read("(* 1 2 3)")) == Ok(Expr::Int(6)));
}

#[test]
fn test_div(){
    assert!(eval(&mut Env::new(), read("(/)")) == Ok(Expr::Int(1)));
    assert!(eval(&mut Env::new(), read("(/ 1)")) == Ok(Expr::Int(1)));
    assert!(eval(&mut Env::new(), read("(/ 3 2)")) == Ok(Expr::Int(1)));
    assert!(eval(&mut Env::new(), read("(/ 3 2 1)")) == Ok(Expr::Int(1)));
}

#[test]
fn test_nested_arith(){
    assert!(eval(&mut Env::new(), read("(/ (- (+ 1 (* 2 3)) 3) 2)")) == Ok(Expr::Int(2)));
}

#[test]
fn test_concat(){
    assert!(eval(&mut Env::new(), read("(concat \"a\" \"b\" \"cd\")")) == Ok(Expr::Str("abcd".to_string())))
}

#[test]
fn test_progn(){
    assert!(eval(&mut Env::new(), read("(progn 1 2)")) == Ok(Expr::Int(2)));
    assert!(eval(&mut Env::new(), read("(progn (+ 1 2) (+ 2 3))")) == Ok(Expr::Int(5)));
}

#[test]
fn test_lambda(){
    assert!(eval(&mut Env::new(), read("(lambda (x) x)")) == Ok(Expr::Lambda(Rc::new(Expr::list1(Expr::Sym("x".to_string()))),
                                                                           Rc::new(Expr::list2(Expr::Sym("progn".to_string()), Expr::Sym("x".to_string()))))));
    assert!(eval(&mut Env::new(), read("((lambda (x) (+ x x)) 1)")) == Ok(Expr::Int(2)))
}


#[test]
fn test_funcall(){
    assert!(eval(&mut Env::new(), read("(funcall #'+ 1 2)")) == Ok(Expr::Int(3)));
    assert!(eval(&mut Env::new(), read("(funcall #'(lambda (x y) (* x y)) 1 2)")) == Ok(Expr::Int(2)));
    assert!(eval(&mut Env::new(), read("(funcall (lambda (x y) (* x y)) 1 2)")) == Ok(Expr::Int(2)))
}
