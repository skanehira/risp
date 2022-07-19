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
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '*' => Token::ASTERISK,
            '/' => Token::SLASH,
            '+' => match self.peek() {
                '0'..='9' => self.read_as_number(),
                _ => Token::PLUS,
            },
            '-' => match self.peek() {
                '0'..='9' => self.read_as_number(),
                _ => Token::MINUS,
            },
            '0'..='9' => self.read_as_number(),
            '"' => self.read_as_string(),
            '\0' => Token::EOF,
            _ => Token::ILLEGAL,
        };
        self.read();

        token
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

        Token::STRING(s)
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
        if s.contains(".") {
            Token::FLOAT(s.parse::<f64>().unwrap())
        } else {
            Token::INT(s.parse::<isize>().unwrap())
        }
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
    fn read_string() {
        let mut lexer = Lexer::new(String::from(r#""hello""#));
        assert_eq!(lexer.next_token(), Token::STRING(String::from("hello")));
    }

    #[test]
    fn read_number() {
        let tests = vec![
            ("1", Token::INT(1)),
            ("1.5", Token::FLOAT(1.5)),
            ("2.345", Token::FLOAT(2.345)),
        ];
        for test in tests {
            let mut lexer = Lexer::new(test.0.to_string());
            assert_eq!(lexer.next_token(), test.1);
        }
    }

    #[test]
    fn basic_arithemetic() {
        let mut lexer = Lexer::new(String::from("(+ 1 2)"));
        assert_eq!(lexer.next_token(), Token::LPAREN);
        assert_eq!(lexer.next_token(), Token::PLUS);
        assert_eq!(lexer.next_token(), Token::INT(1));
        assert_eq!(lexer.next_token(), Token::INT(2));
        assert_eq!(lexer.next_token(), Token::RPAREN);
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn nested_arithmetic() {
        let mut lexer = Lexer::new(String::from("(+ (- 30 2) (* (/ 4 2) 3))"));
        let wants = vec![
            Token::LPAREN,
            Token::PLUS,
            Token::LPAREN,
            Token::MINUS,
            Token::INT(30),
            Token::INT(2),
            Token::RPAREN,
            Token::LPAREN,
            Token::ASTERISK,
            Token::LPAREN,
            Token::SLASH,
            Token::INT(4),
            Token::INT(2),
            Token::RPAREN,
            Token::INT(3),
            Token::RPAREN,
            Token::RPAREN,
            Token::EOF,
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
