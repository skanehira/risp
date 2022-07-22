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
            Token::NUMBER(num) => {
                return Ok(Expr::Number(num));
            }
            Token::STRING(s) => {
                return Ok(Expr::String(s));
            }
            Token::LITERAL(symbol) => {
                return Ok(Expr::Symbol(symbol));
            }
            Token::ASTERISK => {
                return Ok(Expr::Symbol("*".to_string()));
            }
            Token::MINUS => {
                return Ok(Expr::Symbol("-".to_string()));
            }
            Token::PLUS => {
                return Ok(Expr::Symbol("+".to_string()));
            }
            Token::SLASH => {
                return Ok(Expr::Symbol("/".to_string()));
            }
            Token::ILLEGAL(token) => {
                return Err(ExprErr::Cause(format!("invalid token: {}", token)));
            }
            Token::EOF | Token::RPAREN => {
                return Ok(Expr::Nil);
            }
            Token::LPAREN => {
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
        ];
        for test in tests {
            let l = Lexer::new(String::from(test));
            let mut p = Parser::new(l);
            let expr = p.parse().unwrap();
            assert_eq!(expr.to_string(), test.to_uppercase());
        }
    }
}
