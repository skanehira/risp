use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub enum Symbol {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Atom {
    Number(Number),
    String(String),
    Sym(Symbol),
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Atom::String(s) => write!(f, "{}", s),
            Atom::Number(num) => write!(f, "{}", num),
            Atom::Sym(sym) => match sym {
                Symbol::Add => write!(f, "+"),
                Symbol::Sub => write!(f, "-"),
                Symbol::Mul => write!(f, "*"),
                Symbol::Div => write!(f, "/"),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Number {
    Int(isize),
    Float(f64),
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Number::Int(num) => write!(f, "{}", num.to_string()),
            Number::Float(num) => write!(f, "{}", num.to_string()),
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, r: Self) -> Number {
        match (self, r) {
            (Number::Int(l), Number::Int(r)) => (Number::Int(l + r)),
            (Number::Float(l), Number::Float(r)) => (Number::Float(l + r)),
            (Number::Int(l), Number::Float(r)) => (Number::Float(l as f64 + r)),
            (Number::Float(l), Number::Int(r)) => (Number::Float(l + r as f64)),
        }
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, r: Self) -> Number {
        match (self, r) {
            (Number::Int(l), Number::Int(r)) => (Number::Int(l - r)),
            (Number::Float(l), Number::Float(r)) => (Number::Float(l - r)),
            (Number::Int(l), Number::Float(r)) => (Number::Float(l as f64 - r)),
            (Number::Float(l), Number::Int(r)) => (Number::Float(l - r as f64)),
        }
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, r: Self) -> Number {
        match (self, r) {
            (Number::Int(l), Number::Int(r)) => (Number::Int(l * r)),
            (Number::Float(l), Number::Float(r)) => (Number::Float(l * r)),
            (Number::Int(l), Number::Float(r)) => (Number::Float(l as f64 * r)),
            (Number::Float(l), Number::Int(r)) => (Number::Float(l * r as f64)),
        }
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, r: Self) -> Number {
        match (self, r) {
            (Number::Int(l), Number::Int(r)) => (Number::Int(l / r)),
            (Number::Float(l), Number::Float(r)) => (Number::Float(l / r)),
            (Number::Int(l), Number::Float(r)) => (Number::Float(l as f64 / r)),
            (Number::Float(l), Number::Int(r)) => (Number::Float(l / r as f64)),
        }
    }
}

#[derive(Debug)]
pub enum Cell {
    Cons(Atom, Option<Box<Cell>>),
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Cons(atom, cell) => match cell {
                Some(cell) => write!(f, "({} {})", atom, cell),
                None => write!(f, "{}", atom),
            },
        }
    }
}
