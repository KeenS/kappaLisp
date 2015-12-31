extern crate kappa_lisp;
use kappa_lisp::{init, run, Expr, Env};

#[test]
fn test_skk_calc(){
    let mut env = Env::new();
    init(&mut env).unwrap();
    run(&mut env, "(set 'skk-num-list '(3 2))").unwrap();
    assert_eq!(run(&mut env, ("(skk-calc '+)")), Ok(Expr::Int(5)));
    assert_eq!(run(&mut env, ("(skk-calc '-)")), Ok(Expr::Int(1)));
    assert_eq!(run(&mut env, ("(skk-calc '*)")), Ok(Expr::Int(6)));
    assert_eq!(run(&mut env, ("(skk-calc '/)")), Ok(Expr::Int(1)));
}


#[test]
fn test_skk_gadget_units_conversion(){
    let mut env = Env::new();
    init(&mut env).unwrap();
    assert_eq!(run(&mut env, ("(skk-gadget-units-conversion \"mile\" 1 \"km\")")),
               Ok(Expr::Float(1.6093)));
}
