use std::collections::HashMap;
use std::collections::LinkedList;

use expr::{Expr, Proc, Prim};


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
        env.fregister("+".to_string(), Proc::Prim(Prim::Add));
        env.fregister("-".to_string(), Proc::Prim(Prim::Sub));
        env.fregister("/".to_string(), Proc::Prim(Prim::Div));
        env.fregister("*".to_string(), Proc::Prim(Prim:: Mul));
        env.fregister("concat".to_string(), Proc::Prim(Prim::Concat));
        env.fregister("funcall".to_string(), Proc::Prim(Prim::Funcall));
        env.fregister("car".to_string(), Proc::Prim(Prim::Car));
        env.fregister("cdr".to_string(), Proc::Prim(Prim::Cdr));
        env.fregister("current-time-string".to_string(), Proc::Prim(Prim::CurrentTimeString));
        env.fregister("skk-calc".to_string(), Proc::Prim(Prim::SkkCalc));
        env.fregister("skk-gadget-units-conversion".to_string(),  Proc::Prim(Prim::SkkGadgetUnitsConversion));
        env
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

    pub fn register(&mut self, name: String, value: Expr) {
        match self.local.front_mut() {
            Some(l) => l.insert(name, value),
            None => self.global.insert(name, value)
        };
    }

    pub fn fregister(&mut self, name: String, value: Proc) {
        match self.flocal.front_mut() {
            Some(l) => l.insert(name, value),
            None => self.fglobal.insert(name, value)
        };
    }

    // pub fn pregister(&mut self, name: String, value: Box<Fn(&mut Env, Expr) -> Result<Expr, String>>) {
    //     match self.plocal.front_mut() {
    //         Some(l) => l.insert(name, value),
    //         None => self.pglobal.insert(name, value)
    //     };
    // }

    pub fn find(&self, name: &String)  -> Result<&Expr, String> {
        for m in self.local.iter() {
            match m.get(name) {
                Some(v) => return Ok(v),
                None => ()
            }
        };
        match self.global.get(name) {
            Some(v) => Ok(v),
            None => Err(format!("Variable {:} isn't bound", name))
        }
    }

    pub fn ffind(&self, name: &String)  -> Result<&Proc, String> {
        for m in self.flocal.iter() {
            match m.get(name) {
                Some(v) => return Ok(v),
                None => ()
            }
        };
        match self.fglobal.get(name) {
            Some(v) => Ok(v),
            None => Err(format!("Function {:} isn't bound", name))
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
