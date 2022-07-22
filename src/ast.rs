use std::rc::Rc;

#[derive(Debug)]
pub enum ExprErr {
    Cause(String),
}

impl std::string::ToString for ExprErr {
    fn to_string(&self) -> String {
        match self {
            ExprErr::Cause(s) => String::from(s),
        }
    }
}

#[derive(Clone)]
pub struct Lambda {
    pub args: Vec<String>,
    pub body: Rc<Expr>,
}

#[derive(Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<Expr>),
    True,
    Nil,
    Func(fn(&[Expr]) -> Result<Expr, ExprErr>),
    Lambda(Lambda),
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Symbol(a), Expr::Symbol(b)) => a == b,
            (Expr::Number(a), Expr::Number(b)) => a == b,
            (Expr::String(a), Expr::String(b)) => a == b,
            (Expr::List(a), Expr::List(b)) => a == b,
            (Expr::Nil, Expr::Nil) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Expr::List(exprs) => {
                let xs = exprs.iter().map(|x| x.to_string()).collect::<Vec<String>>();
                format!("({})", xs.join(" "))
            }
            Expr::Number(num) => num.to_string(),
            Expr::String(s) => s.to_string(),
            Expr::Symbol(sym) => sym.to_string(),
            Expr::Nil => "NIL".to_string(),
            Expr::Func(_) => "FUNCTION".to_string(),
            Expr::Lambda(_) => "LAMBDA".to_string(),
            Expr::True => "T".to_string(),
        };

        write!(f, "{}", s)
    }
}
