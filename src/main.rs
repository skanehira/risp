use anyhow::Result;
use std::{
    io::{stdin, BufRead},
    iter::Peekable,
    str::Chars,
};

fn main() -> Result<()> {
    let stdin = stdin();
    let reader = stdin.lock();
    repl(reader)?;

    Ok(())
}

struct Evaluator {
    //env: HashMap<String, Box<dyn Any>>,
    stack: Vec<String>,
}

impl Evaluator {
    fn new() -> Evaluator {
        //let env = HashMap::<String, Box<dyn Any>>::new();
        let stack = Vec::<String>::new();
        Evaluator { stack }
    }

    fn eval(&mut self, chars: &mut Peekable<Chars>) -> String {
        loop {
            let ch = chars.next();
            match ch {
                Some(ch) => match ch {
                    '(' => {
                        let result = self.eval(chars);
                        self.stack.push(result);
                    }
                    ')' => {
                        let result = self.inner_eval();
                        self.stack.push(result);
                        break;
                    }
                    '0'..='9' => {
                        let number = self.read_as_number(ch, chars);
                        self.stack.push(number);
                    }
                    '+' | '-' => match chars.peek().unwrap() {
                        '0'..='9' => {
                            let number = self.read_as_number(ch, chars);
                            self.stack.push(number);
                        }
                        _ => self.stack.push(ch.to_string()),
                    },
                    '*' | '/' => {
                        self.stack.push(ch.to_string());
                    }
                    _ => {
                        if ch.is_whitespace() {
                            continue;
                        }
                        break;
                    }
                },
                None => break,
            }
        }

        self.stack.pop().unwrap()
    }

    fn inner_eval(&mut self) -> String {
        let mut args = Vec::<String>::new();
        loop {
            match self.stack.pop() {
                Some(value) => match value.as_str() {
                    "+" | "-" | "*" | "/" => {
                        let result = self.calc(value, args);
                        return result;
                    }
                    _ => match value.chars().next().unwrap() {
                        '-' | '+' | '0'..='9' => args.push(value),
                        _ => panic!("token is not number: {}", value),
                    },
                },
                None => panic!("not found valid operator, stack={:?}", self.stack),
            }
        }
    }

    fn calc(&mut self, op: String, mut args: Vec<String>) -> String {
        dbg!(&self.stack, &op, &args);
        let mut num = args.pop().unwrap().parse::<isize>().unwrap();

        for arg in args {
            let v = arg.parse::<isize>().unwrap();
            num = match op.as_str() {
                "+" => num + v,
                "-" => num - v,
                "*" => num * v,
                "/" => num / v,
                _ => break,
            }
        }

        num.to_string()
    }

    fn read_as_number(&mut self, ch: char, chars: &mut Peekable<Chars>) -> String {
        let mut str = String::new();
        str.push(ch);
        loop {
            if let Some(ch) = chars.peek() {
                if *ch == ')' {
                    break;
                }
            }
            match chars.next() {
                Some(ch) => match ch {
                    '0'..='9' | '.' | '+' | '-' => {
                        str.push(ch);
                    }
                    ')' => {}
                    _ => break,
                },
                None => break,
            }
        }
        str
    }
}

fn repl<R: BufRead>(reader: R) -> Result<()> {
    let mut evaluator = Evaluator::new();
    for line in reader.lines() {
        let line = line?;
        if line == "" {
            continue;
        }
        let result = evaluator.eval(&mut line.chars().peekable());
        println!("{}", result);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let mut evaluator = Evaluator::new();
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
            let got = evaluator.eval(&mut test.0.chars().peekable());
            let want = test.1;
            assert_eq!(got, want)
        }
    }
}
