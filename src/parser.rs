use super::ast::*;
use super::lexer::*;
use super::token::*;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Result<Expr, ExprErr> {
        let token = self.lexer.next_token();

        match token {
            Token::Number(num) => Ok(Expr::Number(num)),
            Token::String(s) => Ok(Expr::String(s)),
            Token::Literal(symbol) => Ok(Expr::Symbol(symbol)),
            Token::Asterfisk => Ok(Expr::Symbol("*".to_string())),
            Token::Minus => Ok(Expr::Symbol("-".to_string())),
            Token::Plus => Ok(Expr::Symbol("+".to_string())),
            Token::Slash => Ok(Expr::Symbol("/".to_string())),
            Token::True => Ok(Expr::True),
            Token::Nil => Ok(Expr::Nil),
            Token::Illegal(token) => Err(ExprErr::Cause(format!("invalid token: {}", token))),
            Token::Eof | Token::Rparen => Ok(Expr::Nil),
            Token::Lparen => {
                let mut list = Vec::<Expr>::new();
                loop {
                    match self.parse() {
                        Ok(expr) => {
                            if expr == Expr::Nil {
                                return Ok(Expr::List(list));
                            }
                            list.push(expr);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let tests = vec![
            "(+ -10 5)",
            "(+ (* 1 2) 3)",
            "(+ (/ 2 (- 10 (* 1 1))))",
            "1",
            "hello",
            "(+ 1 2 (* 1 3))",
            "t",
            "nil",
        ];
        for test in tests {
            let l = Lexer::new(String::from(test));
            let mut p = Parser::new(l);
            let expr = p.parse().unwrap();
            assert_eq!(expr.to_string(), test.to_uppercase());
        }
    }
}
