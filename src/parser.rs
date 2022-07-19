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

    pub fn parse(&mut self) -> Vec<Cell> {
        let mut exprs = Vec::<Cell>::new();
        loop {
            match self.inner_parse() {
                Some(expr) => {
                    exprs.push(*expr);
                }
                None => {
                    break;
                }
            }
        }
        exprs
    }

    // Cons(+, Cons(1, Cons(2, None)))
    // Cons(1, None)
    // Cons("hello", None)
    fn inner_parse(&mut self) -> Option<Box<Cell>> {
        match self.lexer.next_token() {
            Token::INT(i) => Some(Box::new(Cell::Cons(Atom::Int(i), self.inner_parse()))),
            Token::STRING(s) => Some(Box::new(Cell::Cons(Atom::String(s), None))),
            Token::LPAREN => {
                let sym = match self.lexer.next_token() {
                    Token::PLUS => Symbol::Add,
                    Token::MINUS => Symbol::Sub,
                    Token::ASTERISK => Symbol::Mul,
                    Token::SLASH => Symbol::Div,
                    _ => unreachable!(),
                };

                let cell = Cell::Cons(Atom::Sym(sym), self.inner_parse());
                Some(Box::new(cell))
            }
            Token::EOF | Token::RPAREN => None,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse(input: String) -> Vec<Cell> {
        let lexer = Lexer::new(input);
        let mut p = Parser::new(lexer);
        let cells = p.parse();
        cells
    }

    #[test]
    fn symbol_int() {
        let cell = parse(String::from("1"));
        assert_eq!(cell[0].to_string(), "1");
    }

    #[test]
    fn symbol_string() {
        let cell = parse(String::from("\"hello world\""));
        assert_eq!(cell[0].to_string(), "hello world");
    }

    #[test]
    fn list_expression() {
        let cell = parse(String::from("(+ 1 2)"));
        assert_eq!(cell[0].to_string(), "(+ (1 2))");
    }

    #[test]
    fn list_nested_expression() {
        let cell = parse(String::from("(+ 1 (- 10 (* 1 (/ 2 1))))"));
        assert_eq!(cell[0].to_string(), "(+ (1 (- (10 (* (1 (/ (2 1))))))))");
    }
}
