use std::collections::HashMap;
use std::collections::LinkedList;

use expr::Expr;


pub struct Env<'a, 'b> {
    global: HashMap<&'a str, Expr>,
    local: LinkedList<HashMap<&'b str, Expr>>,
}


impl <'a, 'b>Env<'a, 'b> {
    pub fn new() -> Env<'a, 'b> {
        Env {
            global: HashMap::new(),
            local: LinkedList::new()
        }
    }
}
