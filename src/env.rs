use std::collections::HashMap;
use std::collections::LinkedList;

use expr::Expr;


pub struct Env {
    global: HashMap<String, Expr>,
    local: LinkedList<HashMap<String, Expr>>,
    fglobal: HashMap<String, Expr>,
    flocal: LinkedList<HashMap<String, Expr>>,
}


impl Env {
    pub fn new() -> Env {
        Env {
            global: HashMap::new(),
            local: LinkedList::new(),
            fglobal: HashMap::new(),
            flocal: LinkedList::new()
        }
    }
    pub fn new_local(&mut self) {
        self.local.push_front(HashMap::new());
        self.flocal.push_front(HashMap::new());
    }

    pub fn end_local(&mut self) {
        self.local.pop_front();
        self.flocal.pop_front();
    }

    pub fn register(&mut self, name: String, value: Expr) {
        match self.local.front_mut() {
            Some(l) => l.insert(name, value),
            None => self.global.insert(name, value)
        };
    }

    pub fn fregister(&mut self, name: String, value: Expr) {
        match self.flocal.front_mut() {
            Some(l) => l.insert(name, value),
            None => self.global.insert(name, value)
        };
    }

    pub fn find(&mut self, name: &String)  -> Option<&Expr> {
        for m in self.local.iter() {
            match m.get(name) {
                Some(v) => return Some(v),
                None => ()
            }
        };
        self.global.get(name)
    }

    pub fn ffind(&mut self, name: &String)  -> Option<&Expr> {
        for m in self.flocal.iter() {
            match m.get(name) {
                Some(v) => return Some(v),
                None => ()
            }
        };
        self.fglobal.get(name)
    }
}
