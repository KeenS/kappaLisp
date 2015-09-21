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

fn funcall(mut env: &mut Env, f: &Expr, args: Expr) -> Expr {
    match f {
        &Expr::FLambda(ref prim) => match prim {
            &Prim::Add => k_add(env, args),
            &Prim::Sub => k_sub(env, args),
            &Prim::Mul => k_mul(env, args),
            &Prim::Div => k_div(env, args),
            _ => panic!("unknown function ")
        },
        &Expr::Lambda(ref params, ref body) => panic!("not implemented"),
        _ => panic!("not a function ")
    }
}

fn feval(mut env: &mut Env, expr: Expr) -> Expr {
    match expr {
        Expr::Sym(ref sym) => match &sym[..] {
            "+" => Expr::FLambda(Prim::Add),
            "-" => Expr::FLambda(Prim::Sub),
            "*" => Expr::FLambda(Prim::Mul),
            "/" => Expr::FLambda(Prim::Div),
            "concat" => Expr::FLambda(Prim::Concat),
            fun => panic!("function {:?} not found", fun)
        },
        Expr::Lambda(_, _) => expr,
        x => panic!("{:?} is not a function", x)
    }
}

fn eval(mut env: &mut Env, expr: Expr) -> Expr {
    match expr {
        Expr::Nil |
        Expr::EOF |
        Expr::Str(_) |
        Expr::Int(_) |
        Expr::Lambda(_, _) |
        Expr::FLambda(_) => expr,
        Expr::Sym(_) => panic!("symbol evaluation is not supported"),
        Expr::Cons(car, cdr) => {
            let f = feval(env, car.deref().clone());
            let arg = f_map(env, &|env, x| eval(env, x), cdr.deref());
            funcall(env, &f, arg)
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
fn test_nested_arith() {
    assert!(eval(&mut Env::new(), read("(/ (- (+ 1 (* 2 3)) 3) 2)")) == (Expr::Int(2)));
}
