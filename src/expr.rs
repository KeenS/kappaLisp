#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Expr {
    Int(isize),
    Cons(Box<Expr>, Box<Expr>),
    Nil,
    Sym(String),
    Str(String),
    EOF
}

