use std::str::Chars;
use std::iter::Peekable;

use expr::Expr;
use util::*;

macro_rules! try_opt {
    ($e: expr) => (
        match $e {
            Some(e) => e,
            None => return None
        }
        )
}


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

fn peek_nonwhitespaces(mut input: &mut Peekable<Chars>, first: char) -> Option<char> {
    match first.is_whitespace(){
        false => return Some(first),
        true =>()
    }
    while input.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
        input.next();
    }
    input.peek().map(|c| c.clone())
}


fn is_delimiter(c: char) -> bool {
    c.is_whitespace() || "()\"'".contains(c)
}


fn read_uint(mut input: &mut Peekable<Chars>, first: char, radix: u32) -> Option<Expr> {
    let mut acc = String::new();
    let mut c;
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
        _ =>{
            let c = try_opt!(input.next());
            match first {
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
    if sym == "nil" {
        Some(Expr::Nil)
    } else {        
        Some(Expr::Sym(sym))
    }
}

fn read_plus(mut input: &mut Peekable<Chars>, first: char) -> Option<Expr> {
    let c = try_opt!(input.peek()).clone();
    match c.is_digit(10) {
        true => read_int(input, first, 10),
        false => read_symbol(input, first)
    }
}


fn read_hyphen(mut input: &mut Peekable<Chars>, first: char) -> Option<Expr> {
    let c = try_opt!(input.peek()).clone();
    match c.is_digit(10) {
        true => read_int(input, first, 10),
        false => read_symbol(input, first)
    }
}

fn read_string(mut input: &mut Peekable<Chars>, _: char) -> Option<Expr> {
    let mut string = String::new();
    // :TODO: treat escapes
    loop {
        let c = try_opt!(input.next());
        match c == '"' {
            true =>  return Some(Expr::Str(string)),
            false => string.push(c)
        }
    };
}

fn read_list(mut input: &mut Peekable<Chars>, _: char) -> Option<Expr> {
    let c = try_opt!(next_nonwhitespaces(input, ' '));
    let car =  match c {
        ')' => return Some(Expr::Nil),
        _ => read_aux(input, c)
    };

    let c = try_opt!(peek_nonwhitespaces(input, ' '));
    let cdr = if c == '.' {
        let _ = try_opt!(next_nonwhitespaces(input, ' '));
        read_aux(input, ' ')
    } else {
        read_list(input, '(')
    };
    match (car, cdr) {
        (Some(car), Some(cdr)) => Some(cons(car, cdr)),
        _ => None
    }
    
}

fn read_quote(mut input: &mut Peekable<Chars>, _: char)  -> Option<Expr> {
    let v =  try_opt!(read_aux(input, ' '));
    Some(list2(Expr::Sym("quote".to_string()), v))
}

fn read_function(mut input: &mut Peekable<Chars>, _: char) -> Option<Expr> {
    let v = try_opt!(read_aux(input, ' '));
    Some(list2(Expr::Sym("function".to_string()), v))
}

fn read_dispatch(mut input: &mut Peekable<Chars>, _: char) -> Option<Expr> {
    let v = try_opt!(input.next());
    match v {
        '\'' => read_function(input, '\''),
        v => panic!("unknown reader macro #{:?}", v)
    }
}

fn read_aux(mut input: &mut Peekable<Chars>, first: char) -> Option<Expr> {
    let first =  try_opt!(next_nonwhitespaces(input, first)) ;
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

pub fn read_in(mut input: &mut Peekable<Chars>) -> Option<Expr> {
    read_aux(input, ' ')
}


pub fn read(s: &str) -> Expr {
    let mut input = s.chars().peekable();
    match read_aux(&mut input, ' ') {
        Some(e) => e,
        None => Expr::EOF
    }
}



#[test]
fn test_read_empty(){
    assert_eq!(read(""), (Expr::EOF));
    assert_eq!(read("(a b"), (Expr::EOF));
}

#[test]
fn test_read_nil() {
    assert_eq!(read("nil"), (Expr::Nil));
}

#[test]
fn test_read_int() {
    assert_eq!(read("0"), (Expr::Int(0)));
    assert_eq!(read("10"), (Expr::Int(10)));
    assert_eq!(read("-10"), (Expr::Int(-10)));
    assert_eq!(read("+10"), (Expr::Int(10)));
}

#[test]
fn test_read_list(){
    assert_eq!(read("()"), (Expr::Nil));
    assert_eq!(read("(1)"), (list1(Expr::Int(1))));
    assert_eq!(read("(1 2)"), (list2(Expr::Int(1), Expr::Int(2))));
    assert_eq!(read("(1 . 2)"), (cons(Expr::Int(1), Expr::Int(2))));
    assert_eq!(read("(1 2 . 3)"), (cons(Expr::Int(1), cons(Expr::Int(2), Expr::Int(3)))));
}


#[test]
fn test_read_symbol(){
    assert_eq!(read("symbol"), (Expr::Sym("symbol".to_string())));
    assert_eq!(read("+symbol"), (Expr::Sym("+symbol".to_string())));
    assert_eq!(read("-symbol"), (Expr::Sym("-symbol".to_string())));
    assert_eq!(read("sym-bol"), (Expr::Sym("sym-bol".to_string())));
    assert_eq!(read("symbol2"), (Expr::Sym("symbol2".to_string())));
}


#[test]
fn test_read_string(){
    assert_eq!(read("\"string\""), (Expr::Str("string".to_string())));
    assert_eq!(read("\"str()ing\""), (Expr::Str("str()ing".to_string())));
    assert_eq!(read("\"str123ing\""), (Expr::Str("str123ing".to_string())));
    assert_eq!(read("\"()string\""), (Expr::Str("()string".to_string())));
    assert_eq!(read("\"123string\""), (Expr::Str("123string".to_string())));
    assert_eq!(read("(\"string\")"), (list1(Expr::Str("string".to_string()))));
}

#[test]
fn test_read_quote(){
    assert_eq!(read("'1"), (list2(Expr::Sym("quote".to_string()), Expr::Int(1))));
    assert_eq!(read("'symbol"), (list2(Expr::Sym("quote".to_string()), Expr::Sym("symbol".to_string()))));
    assert_eq!(read("'\"string\""), (list2(Expr::Sym("quote".to_string()), Expr::Str("string".to_string()))));
    assert_eq!(read("'(1 2)"), (list2(Expr::Sym("quote".to_string()), list2(Expr::Int(1), Expr::Int(2)))))
}

#[test]
fn test_read_function(){
    assert_eq!(read("#'1"), (list2(Expr::Sym("function".to_string()), Expr::Int(1))));
    assert_eq!(read("#'symbol"), (list2(Expr::Sym("function".to_string()), Expr::Sym("symbol".to_string()))));
    assert_eq!(read("#'\"string\""), (list2(Expr::Sym("function".to_string()), Expr::Str("string".to_string()))));
    assert_eq!(read("#'(1 2)"), (list2(Expr::Sym("function".to_string()), list2(Expr::Int(1), Expr::Int(2)))))
}
