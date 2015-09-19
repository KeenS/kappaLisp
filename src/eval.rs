use expr::Expr;

fn k_add_aux(x: &Expr, y: &Expr) -> Expr {
    match (x, y) {
        (&Expr::Int(x), &Expr::Int(y)) => Expr::Int(x + y),
        _ => panic!("non int args {:?} and {:?} are given to +", x, y)
    }
}

fn k_add(args:& Expr) -> Expr {
    let mut res = Expr::Int(0);
    let mut head = args;
    while head != &Expr::Nil {
        match head {
            &Expr::Cons(ref car,ref cdr) => {
                res = k_add_aux(&res, &car);
                head = &cdr
            }
            _ => panic!("invalid argument {:?} to function", args)
        }
    };
    res
}


fn funcall(f: &Expr, args: &Expr) -> Expr {
    match f {
        &Expr::Sym(ref sym) => match &sym[..] {
            "+" => k_add(args),
            _ => panic!("unknown function {:?}", f)
        },
        _ => panic!("not a function {:?}", f)
    }
}

fn eval(expr: &Expr) -> Expr {
    match expr {
        &Expr::Nil => Expr::Nil,
        &Expr::EOF => Expr::EOF,
        &Expr::Str(ref s) => Expr::Str(s.clone()),
        &Expr::Int(ref i) => Expr::Int(i.clone()),
        &Expr::Sym(_) => panic!("symbol evaluation is not supported"),
        &Expr::Cons(ref f, ref args) => funcall(&f, &args)
    }
}


#[test]
fn test_atom(){
    assert!(eval(&Expr::Int(0)) == Expr::Int(0));
    assert!(eval(&Expr::Nil) == Expr::Nil);
    assert!(eval(&Expr::EOF) == Expr::EOF);
    assert!(eval(&Expr::Str("string".to_string())) == Expr::Str("string".to_string()));
}

#[test]
fn test_add(){
    assert!(eval(&Expr::Cons(Box::new(Expr::Sym("+".to_string())),
                             Box::new(Expr::Cons(Box::new(Expr::Int(1)),
                                                 Box::new(Expr::Nil)))))
            == Expr::Int(1));
    assert!(eval(&Expr::Cons(Box::new(Expr::Sym("+".to_string())),
                             Box::new(Expr::Cons(Box::new(Expr::Int(1)),
                                                 Box::new(Expr::Cons(Box::new(Expr::Int(2)),
                                                            Box::new(Expr::Nil)))))))
            == Expr::Int(3));
    assert!(eval(&Expr::Cons(Box::new(Expr::Sym("+".to_string())),
                             Box::new(Expr::Cons(Box::new(Expr::Int(1)),
                                                 Box::new(Expr::Cons(Box::new(Expr::Int(2)),
                                                                     Box::new(Expr::Cons(Box::new(Expr::Int(3)),
                                                                                         Box::new(Expr::Nil)))))))))
            == Expr::Int(6));
}
