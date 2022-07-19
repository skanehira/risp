use super::ast::{
    Atom,
    Atom::{Int, Sym},
    Cell,
    Cell::Cons,
    Symbol,
    Symbol::*,
};

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }

    pub fn eval(&self, cell: Box<Cell>) -> String {
        match *cell {
            Cons(value, list) => match value {
                Sym(op) => match op {
                    Add | Sub | Mul | Div => {
                        Atom::Int(self.op_calc(&op, list).unwrap()).to_string()
                    }
                },
                _ => unreachable!(),
            },
        }
    }

    fn op_calc(&self, op: &Symbol, list: Option<Box<Cell>>) -> Option<isize> {
        match list {
            Some(list) => match *list {
                Cons(value, list) => match value {
                    Int(v) => match list {
                        Some(list) => {
                            let vv = self.op_calc(op, Some(list)).unwrap();
                            let result = match op {
                                Add => v + vv,
                                Sub => v - vv,
                                Mul => v * vv,
                                Div => v / vv,
                            };
                            Some(result)
                        }
                        None => Some(v),
                    },
                    Sym(op) => self.op_calc(&op, list),
                },
            },
            None => None,
        }
    }
}
