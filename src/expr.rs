#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Expr {
    Int(isize),
    Cons(Box<Expr>, Box<Expr>),
    Nil,
    EOF
}

