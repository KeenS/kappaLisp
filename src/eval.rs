use std::rc::Rc;
use std::ops::Deref;

use expr::{Expr,Prim};
use env::Env;
use read::read;

fn f_foldl<F:Fn(&mut Env, Expr, &Expr) -> Expr>(mut env: &mut Env, f: &F, init: Expr, args: &Expr) -> Expr {
    let mut res = init;
    let mut head = args;
    let nil = &Expr::Nil;
    while head != nil {
        match head {
            &Expr::Cons(ref car, ref cdr) => {
                res = f(env, res, car);
                head = cdr;
            }
            _ => panic!("invalid argument {:?} to function", args.clone())
        }
    }
    res
}

fn f_reverse(mut env: &mut Env, args: &Expr) -> Expr {
    f_foldl(env, &|_, acc, x| Expr::Cons(Rc::new(x.clone()), Rc::new(acc)), Expr::Nil, args)
}

fn f_foldr<F:Fn(&mut Env, Expr, &Expr) -> Expr>(mut env: &mut Env, f: &F, init: Expr, args: &Expr) -> Expr {
    match args {
        &Expr::Nil => init,
        &Expr::Cons(ref car, ref cdr) => {
            let v = f_foldr(env, f, init, cdr);
            f(env, v, car)
        }
        _ => panic!("invalid argument {:?} to function", args)
    }
}

fn f_map<F:Fn(&mut Env, Expr) -> Expr>(mut env: &mut Env, f: &F, list: &Expr) -> Expr {
    f_foldr(env, &|env, acc, x| Expr::cons(f(env, x.clone()), acc)
                 , Expr::Nil, list)
}

fn f_iter<F:Fn(&mut Env, Expr)>(mut env: &mut Env, f: &F, list: &Expr) -> Expr {
    f_foldr(env, &|env, acc, x| {f(env,x.clone()); Expr::Nil}
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
        fn $name(mut env: &mut Env, args: Expr) -> Expr {
            let (init, args) = match args {
                Expr::Cons(ref hd, ref tl) => match tl.deref() {
                    tl @ &Expr::Cons(_, _) => (hd.deref().clone(), tl.clone()),
                    tl => ($init, Expr::cons(hd.deref().clone(), tl.clone()))
                },
                args => ($init, args)
            };
            f_foldl(env, &|_, x, y| match (x, y) {
                (Expr::Int(x), &Expr::Int(y)) => Expr::Int(expr!(x $op y)),
                (x, y) => panic!("non int args {:?} and {:?} are given to $op", x, y)
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
            &Expr::Int(x) => {x.clone()},
            hd => panic!("nil expected but got {:?}", hd)
        }
    };
    ($v:expr, str) => {
        match $v {
            &Expr::Str(ref x) => {x.clone()},
            hd => panic!("string expected but got {:?}", hd)
        }
    };
    ($v:expr, sym) => {
        match $v {
            &Expr::Sym(ref x) => {x.clone()},
            hd => panic!("symbol expected but got {:?}", hd)
        }
    };
    ($v:expr, nil) => {
        match $v {
            &Expr::Int(x) => {x.clone()},
            hd => panic!("nil expected but got {:?}", hd)
        }
    };
}

#[macro_export]
macro_rules! get_args {
    ($args: expr, ($var: ident, $ident: tt), $($other:tt), *) =>
        (
            let $var = match $args {
                Expr::Cons(ref hd, ref tl) => get_args_one!(hd.deref(), $ident),
                Expr::Nil => panic!("invalid number of arguments"),
                args => panic!("invalid argument to function call")
            };
            get_args!(tl.deref.clone(), $($other),*)
        )
    ;
    ($args: expr, ) => ();
}


fn k_double(mut env: &mut Env, args: Expr) -> Expr {
    get_args!(args, (x, int),);
    Expr::Int(x*2)
}


fn k_concat(mut env: &mut Env, args: Expr) -> Expr {
    f_foldl(env, &|_, acc, x| match (acc, x) {
        (Expr::Str(ref acc), &Expr::Str(ref x)) => {
            Expr::Str(format!("{}{}",acc, x))
        },
        (_, y) => panic!("non string args {:?} are given to concat", y)
    }
            , Expr::Str("".to_string()), &args)
}


fn k_funcall(mut env: &mut Env, args: Expr) -> Expr {
    match args {
        Expr::Cons(f, args) => funcall(env, f.deref(), args.deref().clone()),
        args => panic!("illeagal form of funcall {:?}", args)
    }
}

fn bind_names(mut env: &mut Env, params: Expr, args: Expr) {
    let mut phead = &params;
    let mut ahead = &args;
    let nil = &Expr::Nil;
    while phead != nil && ahead != nil {
        match (phead, ahead) {
            (&Expr::Cons(ref pcar, ref pcdr), &Expr::Cons(ref acar, ref acdr)) => {
                match pcar.deref().clone() {
                    Expr::Sym(ref name) => env.register(name.clone(), acar.deref().clone()),
                    pcar => panic!("illeagal form of params {:?}", pcar)
                };
                phead = pcdr.deref();
                ahead = acdr.deref();
            },
            _ => panic!("ileagal form of funcall")
        }
    }
    
}


fn funcall(mut env: &mut Env, f: &Expr, args: Expr) -> Expr {
    match f {
        &Expr::FLambda(ref prim) => match prim {
            &Prim::Add => k_add(env, args),
            &Prim::Sub => k_sub(env, args),
            &Prim::Mul => k_mul(env, args),
            &Prim::Div => k_div(env, args),
            &Prim::Funcall => k_funcall(env, args),
            &Prim::Concat => k_concat(env, args),
            &Prim::Double => k_double(env, args)
        },
        &Expr::Lambda(ref params, ref body) => {
            env.new_local();
            bind_names(env, params.deref().clone(), args);
            let ret = eval(env, body.deref().clone());
            env.end_local();
            ret
        },
        f => panic!("{:?} is not a function", f)
    }
}

fn k_quote(mut env: &mut Env, args: Expr) -> Expr {
    match args {
        Expr::Cons(ref car, ref nil) => car.deref().clone(),
        _ => unreachable!()
    }
}

fn k_feval(mut env: &mut Env, args: Expr) -> Expr {
    match args {
        Expr::Cons(ref car, ref nil) => feval(env, car.deref().clone()),
        _ => unreachable!()
    }
}

fn k_lambda(mut env: &mut Env, args: Expr) -> Expr {
    match args {
        Expr::Cons(params, body) => Expr::Lambda(params, Rc::new(Expr::Cons(Rc::new(Expr::Sym("progn".to_string())), body))),
        _ => unreachable!()
    }
}

fn k_progn(mut env: &mut Env, args: Expr) -> Expr {
    let mut head = &args;
    let nil = &Expr::Nil;
    let mut res = Expr::Nil;
    while head != nil {
        match head {
            &Expr::Cons(ref car, ref cdr) => {
                res = eval(env, car.deref().clone());
                head = cdr.deref();
            },
            _ => panic!("invalid form of progn or lambda")
        }
    };
    res
}

fn feval(mut env: &mut Env, expr: Expr) -> Expr {
    match expr {
        Expr::Sym(ref sym) => match &sym[..] {
            "+" => Expr::FLambda(Prim::Add),
            "-" => Expr::FLambda(Prim::Sub),
            "*" => Expr::FLambda(Prim::Mul),
            "/" => Expr::FLambda(Prim::Div),
            "concat" => Expr::FLambda(Prim::Concat),
            "funcall" => Expr::FLambda(Prim::Funcall),
            "double" => Expr::FLambda(Prim::Double),
            fun => match env.ffind(&fun.to_string()) {
                Some(f) => f.clone(),
                None => panic!("function {:?} not found", fun)
            }
        },
        Expr::Cons(ref op, ref rest) => {
            let op = op.deref();
            match op {
                &Expr::Sym(ref sym) => match &sym[..] {
                    "lambda" => k_lambda(env, rest.deref().clone()),
                    op => panic!("invalid expression '({:?} {:?})' found at function potision", op, rest)
                },
                op => panic!("invalid expression '({:?} {:?})' found at function potision", op, rest)
            }
        }
        Expr::Lambda(_, _) => expr,
        x => panic!("{:?} is not a function", x)
    }
}


pub fn eval(mut env: &mut Env, expr: Expr) -> Expr {
    match expr {
        Expr::Nil |
        Expr::EOF |
        Expr::Str(_) |
        Expr::Int(_) |
        Expr::Lambda(_, _) |
        Expr::FLambda(_) => expr,
        Expr::Sym(ref name) => match env.find(&name.to_string()) {
            Some(v) => v.clone(),
            None => panic!("variable {:?} not found", name)
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
                        let f = feval(env, car.clone());
                        let arg = f_map(env, &|env, x| eval(env, x), cdr);
                        funcall(env, &f, arg)                        
                    }
                },
                car => {
                    let f = feval(env, car.clone());
                    let arg = f_map(env, &|env, x| eval(env, x), cdr);
                    funcall(env, &f, arg)                        
                }
            }    
        }
    }
}


#[test]
fn test_atom(){
    assert!(eval(&mut Env::new(), read("1")) == (Expr::Int(1)));
    assert!(eval(&mut Env::new(), read("()")) == (Expr::Nil));
    assert!(eval(&mut Env::new(), read("")) == (Expr::EOF));
    assert!(eval(&mut Env::new(), read("\"string\"")) == (Expr::Str("string".to_string())));
}

#[test]
fn test_add(){
    assert!(eval(&mut Env::new(), read("(+)")) == (Expr::Int(0)));
    assert!(eval(&mut Env::new(), read("(+ 1)")) == (Expr::Int(1)));
    assert!(eval(&mut Env::new(), read("(+ 1 2)")) == (Expr::Int(3)));
    assert!(eval(&mut Env::new(), read("(+ 1 2 3)")) == (Expr::Int(6)));
}

#[test]
fn test_sub(){
    assert!(eval(&mut Env::new(), read("(-)")) == (Expr::Int(0)));
    assert!(eval(&mut Env::new(), read("(- 1)")) == (Expr::Int(-1)));
    assert!(eval(&mut Env::new(), read("(- 1 2)")) == (Expr::Int(-1)));
    assert!(eval(&mut Env::new(), read("(- 1 2 3)")) == (Expr::Int(-4)));
}

#[test]
fn test_mul(){
    assert!(eval(&mut Env::new(), read("(*)")) == (Expr::Int(1)));
    assert!(eval(&mut Env::new(), read("(* 1)")) == (Expr::Int(1)));
    assert!(eval(&mut Env::new(), read("(* 1 2)")) == (Expr::Int(2)));
    assert!(eval(&mut Env::new(), read("(* 1 2 3)")) == (Expr::Int(6)));
}

#[test]
fn test_div(){
    assert!(eval(&mut Env::new(), read("(/)")) == (Expr::Int(1)));
    assert!(eval(&mut Env::new(), read("(/ 1)")) == (Expr::Int(1)));
    assert!(eval(&mut Env::new(), read("(/ 3 2)")) == (Expr::Int(1)));
    assert!(eval(&mut Env::new(), read("(/ 3 2 1)")) == (Expr::Int(1)));
}

#[test]
fn test_nested_arith(){
    assert!(eval(&mut Env::new(), read("(/ (- (+ 1 (* 2 3)) 3) 2)")) == (Expr::Int(2)));
}

#[test]
fn test_concat(){
    assert!(eval(&mut Env::new(), read("(concat \"a\" \"b\" \"cd\")")) == (Expr::Str("abcd".to_string())))
}

#[test]
fn test_progn(){
    assert!(eval(&mut Env::new(), read("(progn 1 2)")) == (Expr::Int(2)));
    assert!(eval(&mut Env::new(), read("(progn (+ 1 2) (+ 2 3))")) == (Expr::Int(5)));
}

#[test]
fn test_lambda(){
    assert!(eval(&mut Env::new(), read("(lambda (x) x)")) == (Expr::Lambda(Rc::new(Expr::list1(Expr::Sym("x".to_string()))),
                                                                           Rc::new(Expr::list2(Expr::Sym("progn".to_string()), Expr::Sym("x".to_string()))))));
    assert!(eval(&mut Env::new(), read("((lambda (x) (+ x x)) 1)")) == (Expr::Int(2)))
}


#[test]
fn test_funcall(){
    assert!(eval(&mut Env::new(), read("(funcall #'+ 1 2)")) == (Expr::Int(3)));
    assert!(eval(&mut Env::new(), read("(funcall #'(lambda (x y) (* x y)) 1 2)")) == (Expr::Int(2)));
    assert!(eval(&mut Env::new(), read("(funcall (lambda (x y) (* x y)) 1 2)")) == (Expr::Int(2)))
}


#[test]
fn test_double() {
    assert!(eval(&mut Env::new(), read("(double 1)")) == (Expr::Int(2)))
}
