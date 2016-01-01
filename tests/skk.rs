extern crate kappa_lisp;
use kappa_lisp::{init, run, Env};
use kappa_lisp::util::*;

#[test]
fn test_skk_calc(){
    let mut env = Env::new();
    init(&mut env).unwrap();
    run(&mut env, "(set 'skk-num-list '(3 2))").unwrap();
    assert_eq!(run(&mut env, ("(skk-calc '+)")), Ok(kint(5)));
    assert_eq!(run(&mut env, ("(skk-calc '-)")), Ok(kint(1)));
    assert_eq!(run(&mut env, ("(skk-calc '*)")), Ok(kint(6)));
    assert_eq!(run(&mut env, ("(skk-calc '/)")), Ok(kint(1)));
}


#[test]
fn test_skk_gadget_units_conversion(){
    let mut env = Env::new();
    init(&mut env).unwrap();
    assert_eq!(run(&mut env, ("(skk-gadget-units-conversion \"mile\" 1 \"km\")")),
               Ok(kfloat(1.6093)));
}
