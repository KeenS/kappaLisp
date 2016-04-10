use expr::Result;
use env::Env;
use read::read_in;
use eval::eval;


pub fn init(mut env: &mut Env) -> Result<()> {
    let lisp = include_str!("stdlib.lisp");
    let mut input = lisp.chars().peekable();
    while let Some(e) = read_in(&mut input) {
        let _ = try!(eval(&mut env, &e));
    }
    Ok(())
}
