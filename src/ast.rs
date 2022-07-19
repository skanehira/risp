#[derive(Debug)]
pub enum Symbol {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Atom {
    Int(isize),
    String(String),
    Sym(Symbol),
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Atom::Int(i) => write!(f, "{}", i),
            Atom::String(s) => write!(f, "{}", s),
            Atom::Sym(sym) => match sym {
                Symbol::Add => write!(f, "+"),
                Symbol::Sub => write!(f, "-"),
                Symbol::Mul => write!(f, "*"),
                Symbol::Div => write!(f, "/"),
            },
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
