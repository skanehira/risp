use std::fs::File;
use std::io::BufRead;
use std::{env, io};

use eval::{Evaluator, ExprEnv};
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
mod ast;
mod eval;
mod lexer;
mod parser;
mod token;

fn eval(evaluator: &mut Evaluator, env: &mut ExprEnv, line: &str) -> String {
    let l = lexer::Lexer::new(line.into());
    let mut p = parser::Parser::new(l);
    match p.parse() {
        Ok(expr) => match evaluator.eval(&expr, env) {
            Ok(result) => result.to_string(),
            Err(e) => e.to_string(),
        },
        Err(e) => e.to_string(),
    }
}

fn main() -> Result<()> {
    let mut evaluator = eval::Evaluator::new();
    let mut env: ExprEnv = eval::default_env();

    if atty::is(atty::Stream::Stdin) {
        let args = env::args().collect::<Vec<String>>();
        if args.len() == 1 {
            let mut rl = Editor::<()>::new()?;
            _ = rl.load_history("history.txt");
            loop {
                let readline = rl.readline("risp>> ");
                match readline {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str());
                        let result = eval(&mut evaluator, &mut env, &line);
                        println!("{}", result);
                    }
                    Err(ReadlineError::Interrupted) => {
                        break;
                    }
                    Err(ReadlineError::Eof) => {
                        break;
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                        break;
                    }
                }
            }
            return rl.save_history("history.txt");
        } else {
            let arg = args.get(1);
            if let Some(filename) = arg {
                let file = File::open(filename)?;
                for line in io::BufReader::new(file).lines() {
                    let result = eval(&mut evaluator, &mut env, &line?);
                    println!("{}", result);
                }
            }
        }
    } else {
        let stdin = io::stdin();
        for line in stdin.lines() {
            let result = eval(&mut evaluator, &mut env, &line?);
            println!("{}", result);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(tests: Vec<(&str, &str)>) {
        let mut evaluator = eval::Evaluator::new();
        let mut env = eval::default_env();
        for (i, test) in tests.iter().enumerate() {
            let l = lexer::Lexer::new(test.0.to_string());
            let mut p = parser::Parser::new(l);
            let expr = p.parse().unwrap();
            let result = evaluator.eval(&expr, &mut env).unwrap();

            assert_eq!(
                result.to_string(),
                test.1,
                "test[{}] fail: got={}, want={}",
                i,
                result,
                test.1
            );
        }
    }

    #[test]
    fn eval_basic_atom() {
        test(vec![
            ("1.5", "1.5"),
            ("10.5", "10.5"),
            ("1", "1"),
            ("10", "10"),
            ("-10", "-10"),
            ("\"hello world\"", "hello world"),
            ("\"hello1234\"", "hello1234"),
            ("\"123\"", "123"),
        ]);
    }

    #[test]
    fn eval_calc() {
        test(vec![
            ("(* 10 10)", "100"),
            ("(- 20 10)", "10"),
            ("(+ -10 5)", "-5"),
            ("(* 10 5)", "50"),
            ("(/ 10 5)", "2"),
            ("(* 1 2 3 4)", "24"),
            ("(+ 1 2 3 4)", "10"),
            ("(+ 1 2 (* 1 3))", "6"),
            ("(+ (* 1 2) (- 3 4))", "1"),
            ("(+ (* 1 2) 1)", "3"),
            ("(+ 1 (- 10 (* 10 50)))", "-489"),
        ]);
    }

    #[test]
    fn eval_symbol() {
        test(vec![
            ("(setq a 10)", "10"),
            ("(setq b (+ 1 2))", "3"),
            ("a", "10"),
            ("b", "3"),
            ("(+ 5 a)", "15"),
            ("(+ 5 a (+ 1 (+ 1 1)))", "18"),
            ("(+ a 5)", "15"),
            ("(+ b (+ 5 4))", "12"),
            ("(+ (+ 5 4) a)", "19"),
        ]);
    }

    #[test]
    fn eval_func() {
        test(vec![
            ("(defun add (a b) (+ a b))", "ADD"),
            ("(add 10 5)", "15"),
            ("(defun div (a b) (/ a b))", "DIV"),
            ("(div 10 5)", "2"),
            ("(setq a 5)", "5"),
            ("(add a 5)", "10"),
            ("(defun half (x) (/ x 2))", "HALF"),
            ("(defun medium (x y) (half (+ x y)))", "MEDIUM"),
            ("(medium 2 4)", "3"),
        ])
    }
}
