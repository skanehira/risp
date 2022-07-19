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
            '\0' => Token::EOF,
            _ => Token::ILLEGAL,
        };
        self.read();

        token
    }

    fn read_as_number(&mut self) -> Token {
        let mut chars = Vec::<char>::new();

        loop {
            chars.push(self.ch);
            match self.peek() {
                '0'..='9' => {
                    self.read();
                }
                _ => {
                    break;
                }
            }
        }

        let s: String = chars.iter().collect();
        Token::Int(s.parse::<isize>().unwrap())
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
    fn basic_arithemetic() {
        let mut lexer = Lexer::new(String::from("(+ 1 2)"));
        assert_eq!(lexer.next_token(), Token::LPAREN);
        assert_eq!(lexer.next_token(), Token::PLUS);
        assert_eq!(lexer.next_token(), Token::Int(1));
        assert_eq!(lexer.next_token(), Token::Int(2));
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
            Token::Int(30),
            Token::Int(2),
            Token::RPAREN,
            Token::LPAREN,
            Token::ASTERISK,
            Token::LPAREN,
            Token::SLASH,
            Token::Int(4),
            Token::Int(2),
            Token::RPAREN,
            Token::Int(3),
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
