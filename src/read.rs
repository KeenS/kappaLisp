use std::iter::Peekable;
use std::ops::Deref;
use std::str::{Chars, FromStr};

use expr::{Error, Expr, Kfloat, Kint, Result};
use util::*;

fn next_nonwhitespaces(input: &mut Peekable<Chars>, first: char) -> Option<char> {
    match first.is_whitespace() {
        false => return Some(first),
        true => (),
    }
    while input.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
        input.next();
    }
    input.next()
}

fn peek_nonwhitespaces(input: &mut Peekable<Chars>, first: char) -> Option<char> {
    match first.is_whitespace() {
        false => return Some(first),
        true => (),
    }
    while input.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
        input.next();
    }
    input.peek().map(|c| c.clone())
}

fn is_delimiter(c: char) -> bool {
    c.is_whitespace() || "()\"'".contains(c)
}

fn read_uint(input: &mut Peekable<Chars>, first: char, radix: u32) -> Option<Kint> {
    let mut acc = String::new();
    acc.push(first);
    while input.peek().unwrap_or(&' ').is_digit(radix) {
        let c = match input.next() {
            Some(x) => x,
            None => break,
        };
        acc.push(c);
    }
    Some(Kint::from_str_radix(&acc[..], radix).unwrap())
}

fn read_int(input: &mut Peekable<Chars>, first: char, radix: u32) -> Option<Kint> {
    match first {
        '0'...'9' => read_uint(input, first, radix),
        _ => {
            let c = input.next()?;
            match first {
                '+' => read_uint(input, c, radix),
                '-' => Some(-1 * read_uint(input, c, radix)?),
                _ => None,
            }
        }
    }
}

fn read_number(input: &mut Peekable<Chars>, first: char, radix: u32) -> Option<Expr> {
    let i = read_int(input, first, radix)?;
    match input.peek() {
        Some(&'.') => {
            let mut acc = String::new();
            match first {
                '-' => {
                    acc.push('-');
                    acc.push('0')
                }
                _ => acc.push('0'),
            }
            acc.push(input.next()?);
            while input.peek().unwrap_or(&' ').is_digit(radix) {
                let c = match input.next() {
                    Some(x) => x,
                    None => break,
                };
                acc.push(c);
            }
            // FIXME: ignoring radix
            let f = Kfloat::from_str(&acc[..]).unwrap();
            Some(Expr::Float((i as Kfloat) + f))
        }
        _ => Some(Expr::Int(i)),
    }
}

fn read_symbol(input: &mut Peekable<Chars>, first: char) -> Option<Expr> {
    let mut sym = first.to_string();
    while input.peek().map(|c| !is_delimiter(*c)).unwrap_or(false) {
        sym.push(input.next().unwrap());
    }
    if sym == "nil" {
        Some(knil())
    } else {
        Some(ksym(sym))
    }
}

fn read_keyword(input: &mut Peekable<Chars>, first: char) -> Option<Expr> {
    debug_assert_eq!(first, ':');
    let mut kw = String::new();
    while input.peek().map(|c| !is_delimiter(*c)).unwrap_or(false) {
        kw.push(input.next().unwrap());
    }
    Some(kkw(kw))
}

fn read_plus(input: &mut Peekable<Chars>, first: char) -> Option<Expr> {
    let c = input.peek()?.clone();
    match c.is_digit(10) {
        true => read_number(input, first, 10),
        false => read_symbol(input, first),
    }
}

fn read_hyphen(input: &mut Peekable<Chars>, first: char) -> Option<Expr> {
    let c = input.peek()?.clone();
    match c.is_digit(10) {
        true => read_number(input, first, 10),
        false => read_symbol(input, first),
    }
}

fn read_string(input: &mut Peekable<Chars>, _: char) -> Option<Expr> {
    let mut string = String::new();
    // :TODO: treat escapes
    loop {
        let c = input.next()?;
        match c == '"' {
            true => return Some(kstr(string)),
            false => string.push(c),
        }
    }
}

fn read_list(input: &mut Peekable<Chars>, _: char) -> Option<Expr> {
    let c = next_nonwhitespaces(input, ' ')?;
    let car = match c {
        ')' => return Some(knil()),
        _ => read_aux(input, c),
    };

    let c = peek_nonwhitespaces(input, ' ')?;
    let cdr = if c == '.' {
        let _ = next_nonwhitespaces(input, ' ')?; // == 'c'
        match read_list(input, '(')? {
            Expr::Cons(ref e, ref nil) => {
                if nil.deref() == &knil() {
                    Some(e.deref().clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        read_list(input, '(')
    };
    match (car, cdr) {
        (Some(car), Some(cdr)) => Some(kcons(car, cdr)),
        _ => None,
    }
}

fn read_quote(input: &mut Peekable<Chars>, _: char) -> Option<Expr> {
    let v = read_aux(input, ' ')?;
    Some(klist!(ksym("quote"), v))
}

fn read_function(input: &mut Peekable<Chars>, _: char) -> Option<Expr> {
    let v = read_aux(input, ' ')?;
    Some(klist!(ksym("function"), v))
}

fn read_dispatch(input: &mut Peekable<Chars>, _: char) -> Option<Expr> {
    let v = input.next()?;
    match v {
        '\'' => read_function(input, '\''),
        v => panic!("unknown reader macro #{:?}", v),
    }
}

fn read_aux(input: &mut Peekable<Chars>, first: char) -> Option<Expr> {
    let first = next_nonwhitespaces(input, first)?;
    match first {
        '0'...'9' => read_number(input, first, 10),
        '-' => read_hyphen(input, first),
        '+' => read_plus(input, first),
        '(' => read_list(input, first),
        '"' => read_string(input, first),
        '\'' => read_quote(input, first),
        '#' => read_dispatch(input, first),
        ':' => read_keyword(input, first),
        _ => read_symbol(input, first),
    }
}

pub fn read_in(input: &mut Peekable<Chars>) -> Option<Expr> {
    read_aux(input, ' ')
}

pub fn read(s: &str) -> Result<Expr> {
    let mut input = s.chars().peekable();
    match read_aux(&mut input, ' ') {
        Some(e) => Ok(e),
        None => Err(Error::ReadError),
    }
}
