use std::str::Chars;
use std::iter::Peekable;
use std::rc::Rc;

use mdo::option::{bind, ret};
use expr::Expr;


fn next_nonwhitespaces(mut input: &mut Peekable<Chars>, first: char) -> Option<char> {
    match first.is_whitespace(){
        false => return Some(first),
        true =>()
    }
    while input.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
        input.next();
    }
    input.next()
}

fn is_delimiter(c: char) -> bool {
    c.is_whitespace() || "()\"'".contains(c)
}


fn read_uint(mut input: &mut Peekable<Chars>, first: char, radix: u32) -> Option<Expr> {
    let mut acc = String::new();
    let mut c = first;
    acc.push(first);
    while input.peek().unwrap_or(&' ').is_digit(radix) {
        match input.next() {
            Some(x) => c = x,
            None => break
        }
        acc.push(c);
    }
    Some(Expr::Int(isize::from_str_radix(&acc[..], radix).unwrap()))
}

fn read_int(mut input: &mut Peekable<Chars>, first: char, radix: u32) -> Option<Expr> {
    match first {
        '0'...'9' => read_uint(input, first, radix),
        _ => mdo!{
            c =<< input.next();
            ret match first {
                '+' => read_uint(input, c, radix),
                '-' => read_uint(input, c, radix).map(|e| match e {Expr::Int(i) => Expr::Int(-i),_ =>e}),
                _   => None
            }
        }
    }
}

fn read_symbol(mut input: &mut Peekable<Chars>, first: char) -> Option<Expr> {
    let mut sym = first.to_string();
    while input.peek().map(|c| !is_delimiter(*c)).unwrap_or(false) {
        sym.push(input.next().unwrap());
    }
    Some(Expr::Sym(sym))
}

fn read_plus(mut input: &mut Peekable<Chars>, first: char) -> Option<Expr> {
    let c = match input.peek() {
        None => return None,
        Some(c) => c.clone()
    };
    match c.is_digit(10) {
        true => read_int(input, first, 10),
        false => read_symbol(input, first)
    }
}


fn read_hyphen(mut input: &mut Peekable<Chars>, first: char) -> Option<Expr> {
    let c = match input.peek() {
        None => return None,
        Some(c) => c.clone()
    };
    match c.is_digit(10) {
        true => read_int(input, first, 10),
        false => read_symbol(input, first)
    }
}

fn read_string(mut input: &mut Peekable<Chars>, first: char) -> Option<Expr> {
    let mut sym = String::new();
    // :TODO: treat escapes
    while input.peek().map(|c| *c != '"').unwrap_or(false) {
        sym.push(input.next().unwrap());
    }
    Some(Expr::Str(sym))
}

fn read_list(mut input: &mut Peekable<Chars>, first: char) -> Option<Expr> {
    let c = next_nonwhitespaces(input, ' ');
    let car =  match c {
        Some(c) => match c {
            ')' => return Some(Expr::Nil),
            _ => read_aux(input, c)
        },
        None => return None  
    };
    match input.peek() {
        None => return None,
        Some(c) => ()
    }
    let cdr = read_list(input, '(');
    match (car, cdr) {
        (Some(car), Some(cdr)) => Some(Expr::cons(car, cdr)),
        _ => None
    }
        
}

fn read_quote(mut input: &mut Peekable<Chars>, first: char)  -> Option<Expr> {
    mdo!{
        v =<< read_aux(input, ' ');
        ret ret(Expr::list2(Expr::Sym("quote".to_string()), v))
    }
}

fn read_function(mut input: &mut Peekable<Chars>, first: char) -> Option<Expr> {
    mdo!{
        v =<< read_aux(input, ' ');
        ret ret(Expr::list2(Expr::Sym("function".to_string()), v))
    }    
}

fn read_dispatch(mut input: &mut Peekable<Chars>, first: char) -> Option<Expr> {
    mdo!{
        v =<< input.next();
        ret match v {
            '\'' => read_function(input, '\''),
            v => panic!("unknown reader macro #{:?}", v)
        }
    }
}

fn read_aux(mut input: &mut Peekable<Chars>, first: char) -> Option<Expr> {
    let first =  match next_nonwhitespaces(input, first) {
        Some(c) => c,
        None => return None
    };
    match first {
        '0'...'9' => read_uint(input, first, 10),
        '-' => read_hyphen(input, first),
        '+' => read_plus(input, first),
        '(' => read_list(input, first),
        '"' => read_string(input, first),
        '\'' => read_quote(input, first),
        '#' => read_dispatch(input, first),
        _   => read_symbol(input, first)
    }
}

pub fn read(s: &str) -> Expr {
    let mut input = s.chars().peekable();
    match read_aux(&mut input, ' ') {
        Some(ex) => ex,
        None => Expr::EOF
    }
}



#[test]
fn test_read_empty(){
    assert!(read("") == (Expr::EOF));
    assert!(read("(a b") == (Expr::EOF));
}

#[test]
fn test_read_int() {
    assert!(read("0") == (Expr::Int(0)));
    assert!(read("10") == (Expr::Int(10)));
    assert!(read("-10") == (Expr::Int(-10)));
    assert!(read("+10") == (Expr::Int(10)));
}

#[test]
fn test_read_list(){
    assert!(read("()") == (Expr::Nil));
    assert!(read("(1)") == (Expr::cons(Expr::Int(1), Expr::Nil)));
    assert!(read("(1 2)") == (Expr::cons(Expr::Int(1), Expr::cons(Expr::Int(2),Expr::Nil))));
}


#[test]
fn test_read_symbol(){
    assert!(read("symbol") == (Expr::Sym("symbol".to_string())));
    assert!(read("+symbol") == (Expr::Sym("+symbol".to_string())));
    assert!(read("-symbol") == (Expr::Sym("-symbol".to_string())));
    assert!(read("sym-bol") == (Expr::Sym("sym-bol".to_string())));
    assert!(read("symbol2") == (Expr::Sym("symbol2".to_string())));
}


#[test]
fn test_read_string(){
    assert!(read("\"string\"") == (Expr::Str("string".to_string())));
    assert!(read("\"str()ing\"") == (Expr::Str("str()ing".to_string())));
    assert!(read("\"str123ing\"") == (Expr::Str("str123ing".to_string())));
    assert!(read("\"()string\"") == (Expr::Str("()string".to_string())));
    assert!(read("\"123string\"") == (Expr::Str("123string".to_string())));
}

#[test]
fn test_read_quote(){
    assert!(read("'1") == (Expr::list2(Expr::Sym("quote".to_string()), Expr::Int(1))));
    assert!(read("'symbol") == (Expr::list2(Expr::Sym("quote".to_string()), Expr::Sym("symbol".to_string()))));
    assert!(read("'\"string\"") == (Expr::list2(Expr::Sym("quote".to_string()), Expr::Str("string".to_string()))));
    assert!(read("'(1 2)") == (Expr::list2(Expr::Sym("quote".to_string()), Expr::list2(Expr::Int(1), Expr::Int(2)))))
}

#[test]
fn test_read_function(){
    assert!(read("#'1") == (Expr::list2(Expr::Sym("function".to_string()), Expr::Int(1))));
    assert!(read("#'symbol") == (Expr::list2(Expr::Sym("function".to_string()), Expr::Sym("symbol".to_string()))));
    assert!(read("#'\"string\"") == (Expr::list2(Expr::Sym("function".to_string()), Expr::Str("string".to_string()))));
    assert!(read("#'(1 2)") == (Expr::list2(Expr::Sym("function".to_string()), Expr::list2(Expr::Int(1), Expr::Int(2)))))
}
