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
    Concat,
    Funcall
}


impl Expr {
    pub fn cons(car: Expr, cdr: Expr) -> Expr {
        Expr::Cons(Rc::new(car), Rc::new(cdr))
    }

    pub fn list1(a1: Expr) -> Expr {
        Expr::cons(a1, Expr::Nil)
    }
    pub fn list2(a1: Expr, a2: Expr) -> Expr {
        Expr::cons(a1, Expr::list1(a2))
    }
}
