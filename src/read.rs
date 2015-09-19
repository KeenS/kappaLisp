use std::str::Chars;
use std::iter::Peekable;

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

fn read_hyphen(mut input: &mut Peekable<Chars>, first: char) -> Option<Expr> {
    let c = match input.peek() {
        None => return None,
        Some(c) => c.clone()
    };
    match c.is_digit(10) {
            true => read_int(input, first, 10),
            false => None
    }
}

fn read_list(mut input: &mut Peekable<Chars>, first: char) -> Option<Expr> {
    let c = next_nonwhitespaces(input, first);
    let car =  match c {
        Some(c) => match c {
            ')' => return Some(Expr::Nil),
            _ => read_aux(input, c)
        },
        None => return None  
    };
    let c = input.next();
    let cdr = c.and_then(|c| read_list(input, c));
    match (car, cdr) {
        (Some(car_), Some(cdr_)) => Some(Expr::Cons(Box::new(car_), Box::new(cdr_))),
        _ => None
    }
        
}

fn read_aux(mut input: &mut Peekable<Chars>, first: char) -> Option<Expr> {
    let first =  match next_nonwhitespaces(input, first) {
        Some(c) => c,
        None => return None
    };
    match first {
        '0'...'9' => read_uint(input, first, 10),
        '-' => read_int(input, first, 10),
        '+' => read_int(input, first, 10),
        '(' => match input.next() {
            Some(c) => read_list(input, c),
            None => None
        },
        _   => unreachable!()
    }
}

fn read(s: &str) -> Option<Expr> {
    let mut input = s.chars().peekable();
    read_aux(&mut input, ' ')
}



#[test]
fn test_read_empty(){
    assert!(read("") == None)
}

#[test]
fn test_read_int() {
    assert!(read("0") == Some(Expr::Int(0)));
    assert!(read("10") == Some(Expr::Int(10)));
    assert!(read("-10") == Some(Expr::Int(-10)));
}

#[test]
fn test_read_list(){
    assert!(read("()") == Some(Expr::Nil));
    assert!(read("(1)") == Some(Expr::Cons(Box::new(Expr::Int(1)), Box::new(Expr::Nil))));
    assert!(read("(1 2)") == Some(Expr::Cons(Box::new(Expr::Int(1)), Box::new(Expr::Cons(Box::new(Expr::Int(2)), Box::new(Expr::Nil))))));
}
