use eval::ExprEnv;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
mod ast;
mod eval;
mod lexer;
mod parser;
mod token;

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    _ = rl.load_history("history.txt");
    let mut evaluator = eval::Evaluator::new();
    let mut env: ExprEnv = eval::default_env();

    loop {
        let readline = rl.readline("risp>> ");
        match readline {
            Ok(line) => {
                let l = lexer::Lexer::new(line.clone());
                let mut p = parser::Parser::new(l);

                let result = match p.parse() {
                    Ok(expr) => match evaluator.eval(&expr, &mut env) {
                        Ok(result) => result.to_string(),
                        Err(e) => e.to_string(),
                    },
                    Err(e) => e.to_string(),
                };
                rl.add_history_entry(line.as_str());
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
    rl.save_history("history.txt")
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
                result.to_string(),
                test.1
            );
        }
    }

    #[test]
    fn eval_int() {
        test(vec![("1", "1"), ("10", "10"), ("-10", "-10")]);
    }

    #[test]
    fn eval_float() {
        test(vec![("1.5", "1.5"), ("10.5", "10.5")]);
    }

    #[test]
    fn eval_string() {
        test(vec![
            ("\"hello world\"", "hello world"),
            ("\"hello1234\"", "hello1234"),
            ("\"123\"", "123"),
        ]);
    }

    #[test]
    fn eval_int_float() {
        test(vec![
            ("(+ 1.5 10)", "11.5"),
            ("(- 10.5 0.5)", "10"),
            ("(* 10 10)", "100"),
            ("(/ 20 10)", "2"),
        ]);
    }

    #[test]
    fn eval_calc() {
        test(vec![
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
        ])
    }
}
