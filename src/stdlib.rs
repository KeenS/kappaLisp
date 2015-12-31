use expr::Result;
use env::Env;
use read::read_in;
use eval::eval;

#[cfg(test)]
use read::read;
#[cfg(test)]
use expr::Expr;

pub fn init(mut env: &mut Env) -> Result<()> {
    let lisp = include_str!("stdlib.lisp");
    let mut input = lisp.chars().peekable();
    while let Some(e) = read_in(&mut input) {
        let _ = try!(eval(&mut env, &e));
    }
    Ok(())
}


#[test]
fn test_assoc() {
    let mut env = Env::new();
    assert_eq!(eval(&mut env, &read("(cdr (assoc 'two '((one . 1) (two . 2))))")), Ok(Expr::Int(2)));
    
}
