use anyhow::Result;
use std::io::{stdin, BufRead, Write};

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

fn prompt() {
    print!(">");
    _ = std::io::stdout().flush();
}

fn repl<R: BufRead>(reader: R) -> Result<()> {
    let evaluator = eval::Evaluator::new();
    prompt();
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
            prompt();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert(input: &str, want: &str) {
        let evaluator = eval::Evaluator::new();
        let l = lexer::Lexer::new(input.to_string());
        let mut p = parser::Parser::new(l);
        let cells = p.parse();
        let cell = cells.into_iter().next().unwrap();
        let got = evaluator.eval(Box::new(cell));
        assert_eq!(got, want);
    }

    #[test]
    fn eval_int() {
        let tests = vec![("1", "1"), ("10", "10"), ("-10", "-10")];
        for test in tests {
            assert(test.0, test.1);
        }
    }

    #[test]
    fn eval_calc() {
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
            assert(test.0, test.1);
        }
    }
}
