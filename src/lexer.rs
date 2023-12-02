use super::token::Token;

#[derive(Debug)]
pub struct Lexer {
    ch: char,
    input: String,
    length: usize,
    read_position: usize,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let length = input.len() - 1;
        let mut lexer = Self {
            ch: '\0',
            input,
            length,
            read_position: 0,
            position: 0,
        };
        lexer.read();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        while self.ch.is_whitespace() {
            self.read();
        }
        let token = match self.ch {
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            '*' => Token::Asterfisk,
            '/' => Token::Slash,
            '+' => match self.peek() {
                '0'..='9' => self.read_as_number(),
                _ => Token::Plus,
            },
            '-' => match self.peek() {
                '0'..='9' => self.read_as_number(),
                _ => Token::Minus,
            },
            '0'..='9' => self.read_as_number(),
            '"' => self.read_as_string(),
            'a'..='z' | 'A'..='Z' => self.read_as_literal(),
            '\0' => Token::Eof,
            _ => Token::Illegal(self.ch.to_string()),
        };
        self.read();

        token
    }

    fn read_as_literal(&mut self) -> Token {
        if self.ch == 't' && self.peek() == '\0' {
            return Token::True;
        }
        let mut s = String::from("");
        loop {
            s.push(self.ch);
            match self.peek() {
                'a'..='z' | 'A'..='Z' => self.read(),
                _ => break,
            }
        }

        if s.to_uppercase() == "NIL" {
            return Token::Nil;
        }

        Token::Literal(s.to_uppercase())
    }

    fn read_as_string(&mut self) -> Token {
        let mut s = String::from("");
        loop {
            self.read();
            s.push(self.ch);
            if self.peek() == '"' {
                self.read();
                break;
            }
        }

        Token::String(s)
    }

    fn read_as_number(&mut self) -> Token {
        let mut chars = Vec::<char>::new();

        loop {
            chars.push(self.ch);
            match self.peek() {
                '0'..='9' | '.' => {
                    self.read();
                }
                _ => {
                    break;
                }
            }
        }

        let s: String = chars.iter().collect();
        Token::Number(s.parse::<f64>().unwrap())
    }

    fn read(&mut self) {
        self.ch = if self.read_position > self.length {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek(&mut self) -> char {
        if self.read_position > self.length {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }
}

#[cfg(test)]
mod test {
    use super::Token;
    use super::*;

    #[test]
    fn read_invalid_token() {
        let mut lexer = Lexer::new(String::from("^"));
        assert_eq!(lexer.next_token(), Token::Illegal(String::from("^")));
    }

    #[test]
    fn read_string() {
        let mut lexer = Lexer::new(String::from(r#""hello""#));
        assert_eq!(lexer.next_token(), Token::String(String::from("hello")));
    }

    #[test]
    fn read_literal() {
        let mut lexer = Lexer::new(String::from("(setq a 2)"));
        assert_eq!(lexer.next_token(), Token::Lparen);
        assert_eq!(lexer.next_token(), Token::Literal(String::from("SETQ")));
        assert_eq!(lexer.next_token(), Token::Literal(String::from("A")));
        assert_eq!(lexer.next_token(), Token::Number(2.0));
        assert_eq!(lexer.next_token(), Token::Rparen);
    }

    #[test]
    fn read_var() {
        let mut lexer = Lexer::new(String::from("(+ a 2 a)"));
        assert_eq!(lexer.next_token(), Token::Lparen);
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Literal(String::from("A")));
        assert_eq!(lexer.next_token(), Token::Number(2.0));
        assert_eq!(lexer.next_token(), Token::Literal(String::from("A")));
        assert_eq!(lexer.next_token(), Token::Rparen);
    }

    #[test]
    fn read_number() {
        let tests = vec![
            ("1", Token::Number(1.0)),
            ("1.5", Token::Number(1.5)),
            ("2.345", Token::Number(2.345)),
        ];
        for test in tests {
            let mut lexer = Lexer::new(test.0.to_string());
            assert_eq!(lexer.next_token(), test.1);
        }
    }

    #[test]
    fn basic_arithemetic() {
        let mut lexer = Lexer::new(String::from("(+ 1 2)"));
        assert_eq!(lexer.next_token(), Token::Lparen);
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Number(1.0));
        assert_eq!(lexer.next_token(), Token::Number(2.0));
        assert_eq!(lexer.next_token(), Token::Rparen);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn nested_arithmetic() {
        let mut lexer = Lexer::new(String::from("(+ (- 30 2) (* (/ 4 2) 3))"));
        let wants = vec![
            Token::Lparen,
            Token::Plus,
            Token::Lparen,
            Token::Minus,
            Token::Number(30.0),
            Token::Number(2.0),
            Token::Rparen,
            Token::Lparen,
            Token::Asterfisk,
            Token::Lparen,
            Token::Slash,
            Token::Number(4.0),
            Token::Number(2.0),
            Token::Rparen,
            Token::Number(3.0),
            Token::Rparen,
            Token::Rparen,
            Token::Eof,
        ];
        for (i, want) in wants.iter().enumerate() {
            let token = lexer.next_token();
            assert_eq!(
                token, *want,
                "unexpected token[{}]: got={:?}, want={:?}",
                i, token, *want
            );
        }
    }
}
