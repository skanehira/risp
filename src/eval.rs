use crate::ast::{Expr, ExprErr};
use std::collections::HashMap;

pub type ExprEnv = HashMap<String, Expr>;
pub struct Evaluator {}

fn parse_list_of_floats(args: &[Expr]) -> Result<Vec<f64>, ExprErr> {
    args.iter()
        .map(|x| match x {
            Expr::Number(num) => Ok(*num),
            _ => Err(ExprErr::Cause(format!("{} is not number", x))),
        })
        .collect()
}

macro_rules! basic_op {
    ($fn: expr) => {
        |args: &[Expr]| -> Result<Expr, ExprErr> {
            let floats = parse_list_of_floats(args)?;
            let (first, rest) = floats
                .split_first()
                .ok_or_else(|| ExprErr::Cause("expected at least one number".to_string()))?;
            Ok(Expr::Number(rest.iter().fold(*first, $fn)))
        }
    };
}

pub fn default_env() -> ExprEnv {
    let mut env: ExprEnv = HashMap::new();
    env.insert("+".to_string(), Expr::Func(basic_op!(|sum, x| sum + x)));
    env.insert("-".to_string(), Expr::Func(basic_op!(|sum, x| sum - x)));
    env.insert("*".to_string(), Expr::Func(basic_op!(|sum, x| sum * x)));
    env.insert("/".to_string(), Expr::Func(basic_op!(|sum, x| sum / x)));
    env
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }

    fn print_env(&mut self, env: ExprEnv) -> String {
        env.clone()
            .iter()
            .map(|x| format!("{}={}", x.0, x.1))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn eval(&mut self, expr: &Expr, env: &mut ExprEnv) -> Result<Expr, ExprErr> {
        match expr {
            Expr::String(_) => Ok(expr.clone()),
            Expr::Number(_) => Ok(expr.clone()),
            Expr::Nil => Ok(expr.clone()),
            Expr::Symbol(sym) => match env.get(sym) {
                Some(expr) => Ok(expr.clone()),
                None => Err(ExprErr::Cause(format!(
                    "not found symbol: {}, env: {}",
                    sym,
                    self.print_env(env.clone()),
                ))),
            },
            Expr::List(list) => {
                let (first, reset) = list
                    .split_first()
                    .ok_or_else(|| ExprErr::Cause("expected at least one number".to_string()))?;
                match self.eval_builtin(first, reset, env) {
                    Some(expr) => expr,
                    None => {
                        let expr = self.eval(first, env)?;
                        match expr {
                            Expr::Func(f) => f(self.eval_args(reset, env)?.as_slice()),
                            _ => unreachable!(),
                        }
                    }
                }
            }
            _ => Err(ExprErr::Cause(format!("invalid expr: {}", expr))),
        }
    }

    pub fn eval_args(&mut self, args: &[Expr], env: &mut ExprEnv) -> Result<Vec<Expr>, ExprErr> {
        args.iter().map(|x| self.eval(x, env)).collect()
    }

    pub fn eval_builtin(
        &mut self,
        first: &Expr,
        args: &[Expr],
        env: &mut ExprEnv,
    ) -> Option<Result<Expr, ExprErr>> {
        match first {
            Expr::Symbol(symbol) => match symbol.as_str() {
                "SETQ" => Some(self.eval_setq(args, env)),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn eval_setq(&mut self, args: &[Expr], env: &mut ExprEnv) -> Result<Expr, ExprErr> {
        let first = args
            .first()
            .ok_or(ExprErr::Cause("expected first arg".to_string()))?;
        let key = match first {
            Expr::Symbol(s) => Ok(s.clone()),
            _ => Err(ExprErr::Cause("first arg must be symbol".to_string())),
        }?;

        let second = args
            .get(1)
            .ok_or(ExprErr::Cause("expected second arg".to_string()))?;
        let value = self.eval(second, env)?;

        env.insert(key, value.clone());

        Ok(value.clone())
    }
}
