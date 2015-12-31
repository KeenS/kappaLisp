use std::collections::HashMap;
use std::collections::LinkedList;

use expr::{Expr, Proc};
use error::Error as E;
use read::read_in;
use eval::eval;
use skk;
use stdlib;
use std::result;


pub type Result<T> = result::Result<T, E>;

pub struct Env {
    global: HashMap<String, Expr>,
    local: LinkedList<HashMap<String, Expr>>,
    fglobal: HashMap<String, Proc>,
    flocal: LinkedList<HashMap<String, Proc>>,
}


impl Env {
    pub fn new() -> Env {
        let mut env = Env {
            global: HashMap::new(),
            local: LinkedList::new(),
            fglobal: HashMap::new(),
            flocal: LinkedList::new()
        };

        stdlib::init(&mut env);
        skk::init(&mut env);
        env
    }

    pub fn init(&mut self) -> Result<()>{
        let stdlib = include_str!("stdlib.lisp");
        let mut input = stdlib.chars().peekable();
        while let Some(e) = read_in(&mut input) {
            let _ = try!(eval(self, &e));
        }
        Ok(())
    }

    pub fn new_local(&mut self) {
        self.local.push_front(HashMap::new());
        self.flocal.push_front(HashMap::new());
    }

    pub fn end_local(&mut self) {
        self.local.pop_front();
        self.flocal.pop_front();
    }

    pub fn register<S: Into<String>>(&mut self, name: S, value: Expr) {
        match self.local.front_mut() {
            Some(l) => l.insert(name.into(), value),
            None => self.global.insert(name.into(), value)
        };
    }

    pub fn fregister<S: Into<String>>(&mut self, name: S, value: Proc) {
        match self.flocal.front_mut() {
            Some(l) => l.insert(name.into(), value),
            None => self.fglobal.insert(name.into(), value)
        };
    }

    pub fn find(&self, name: &String)  -> Result<&Expr> {
        for m in self.local.iter() {
            match m.get(name) {
                Some(v) => return Ok(v),
                None => ()
            }
        };
        match self.global.get(name) {
            Some(v) => Ok(v),
            None => Err(E::Unbound(name.clone()))
        }
    }

    pub fn ffind(&self, name: &String)  -> Result<&Proc> {
        for m in self.flocal.iter() {
            match m.get(name) {
                Some(v) => return Ok(v),
                None => ()
            }
        };
        println!("{:?}", self.fglobal);
        match self.fglobal.get(name) {
            Some(v) => Ok(v),
            None => Err(E::Unbound(name.clone()))
        }
    }

}
