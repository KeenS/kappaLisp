use env::Env;
use eval::eval;
use expr::Result;
use read::read_in;

pub fn init(env: &mut Env) -> Result<()> {
    let lisp = include_str!("stdlib.lisp");
    let mut input = lisp.chars().peekable();
    while let Some(e) = read_in(&mut input) {
        let _ = eval(env, &e)?;
    }
    Ok(())
}
