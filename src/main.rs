use anyhow::Result;
use std::io::{stdin, BufRead};

mod ast;
mod eval;
mod lexer;
mod parser;
mod token;

fn main() -> Result<()> {
    let stdin = stdin();
    let reader = stdin.lock();
    repl(reader)?;

    Ok(())
}

fn repl<R: BufRead>(reader: R) -> Result<()> {
    let evaluator = eval::Evaluator::new();
    for line in reader.lines() {
        let line = line?;
        if line == "" {
            continue;
        }
        let l = lexer::Lexer::new(line);
        let mut p = parser::Parser::new(l);
        for expr in p.parse() {
            let result = evaluator.eval(Box::new(expr));
            println!("{}", result);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let evaluator = eval::Evaluator::new();
        let tests = vec![
            ("(+ 1 (- 10 (* 10 50)))", "-489"),
            ("(+ -10 5)", "-5"),
            ("(* 10 5)", "50"),
            ("(/ 10 5)", "2"),
            ("(* 1 2 3 4)", "24"),
            ("(+ 1 2 3 4)", "10"),
            ("(+ 1 2 (* 1 3))", "6"),
        ];
        for test in tests {
            let l = lexer::Lexer::new(test.0.to_string());
            let mut p = parser::Parser::new(l);
            let cells = p.parse();
            let cell = cells.into_iter().next().unwrap();
            let got = evaluator.eval(Box::new(cell));
            let want = test.1;
            assert_eq!(got, want)
        }
    }
}
