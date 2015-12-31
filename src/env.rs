use std::collections::HashMap;
use std::collections::LinkedList;

use expr::{Expr, Proc, Prim};
use error::Error as E;
use read::read_in;
use eval::eval;
use stdlib::*;
use util::*;
use skk::*;
use std::result;
use std::rc::Rc;


pub type Result<T> = result::Result<T, E>;

pub struct Env {
    global: HashMap<String, Expr>,
    local: LinkedList<HashMap<String, Expr>>,
    fglobal: HashMap<String, Proc>,
    flocal: LinkedList<HashMap<String, Proc>>,
    // pglobal: HashMap<String, Box<Fn(&mut Env, Expr) -> Result<Expr, String>>>,
    // plocal: LinkedList<HashMap<String, Box<Fn(&mut Env, Expr) -> Result<Expr, String>>>>,
}


impl Env {
    pub fn new() -> Env {
        let mut env = Env {
            global: HashMap::new(),
            local: LinkedList::new(),
            fglobal: HashMap::new(),
            flocal: LinkedList::new()
            // pglobal: HashMap::new(),
            // plocal: LinkedList::new()
        };

            //     Prim::Add => k_add(env, args),
            //     Prim::Sub => k_sub(env, args),
            //     Prim::Div => k_div(env, args),
            //     Prim::Mul => k_mul(env, args),
            //     Prim::Concat => k_concat(env, args),
            //     Prim::Funcall => k_funcall(env, args),
            //     Prim::Cons => k_cons(env, args),
            //     Prim::Car => k_car(env, args),
            //     Prim::Cdr => k_cdr(env, args),
            //     Prim::List => k_list(env, args),
            //     Prim::EqualP => k_equal_p(env, args),
            //     Prim::StringToNumber => k_string_to_number(env, args),
            //     Prim::CurrentTimeString => k_current_time_string(env, args),
            //     Prim::SkkCalc => skk::k_skk_calc(env, args),
        env.fregister("+", procedure("k_add".to_string(), k_add));
        env.fregister("-", procedure("k_sub".to_string(), k_sub));
        env.fregister("/", procedure("k_div".to_string(), k_div));
        env.fregister("*", procedure("k_mul".to_string(), k_mul));
        env.fregister("concat", procedure("k_concat".to_string(), k_concat));
        env.fregister("funcall", procedure("k_funcall".to_string(), k_funcall));
        env.fregister("cons", procedure("k_cons".to_string(), k_cons));
        env.fregister("car", procedure("k_car".to_string(), k_car));
        env.fregister("cdr", procedure("k_cdr".to_string(), k_cdr));
        env.fregister("list", procedure("k_list".to_string(),k_list));
        env.fregister("equal?", procedure("k_equal_p".to_string(), k_equal_p));
        env.fregister("string-to-number", procedure("k_string_to_number",k_string_to_number));
        env.fregister("current-time-string", procedure("k_current_time_string", k_current_time_string));
        env.fregister("skk-calc", procedure("k_skk_calc", k_skk_calc));
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
        // self.plocal.push_front(HashMap::new());
    }

    pub fn end_local(&mut self) {
        self.local.pop_front();
        self.flocal.pop_front();
        // self.plocal.pop_front();
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

    // pub fn pregister(&mut self, name: String, value: Box<Fn(&mut Env, Expr) -> Result<Expr, String>>) {
    //     match self.plocal.front_mut() {
    //         Some(l) => l.insert(name, value),
    //         None => self.pglobal.insert(name, value)
    //     };
    // }

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

    // pub fn pfind(&self, name: &String)  -> Option<&Box<Fn(&mut Env, Expr) -> Result<Expr, String>>> {
    //     for map in self.plocal.iter() {
    //         match map.get(name) {
    //             Some(v) => return Some(v),
    //             None => ()
    //         }
    //     };
    //     self.pglobal.get(name)
    // }
}
