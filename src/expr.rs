use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Expr {
    Int(isize),
    Cons(Rc<Expr>, Rc<Expr>),
    Nil,
    Sym(String),
    Str(String),
    Lambda(Rc<Expr>, Rc<Expr>),
    FLambda(Prim),
    EOF
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Prim {
    Add,
    Sub,
    Div,
    Mul,
    Concat
}


impl Expr {
    pub fn cons(car: Expr, cdr: Expr) -> Expr {
        Expr::Cons(Rc::new(car), Rc::new(cdr))
    }
}
