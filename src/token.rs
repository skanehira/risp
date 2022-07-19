#[derive(Debug, PartialEq)]
pub enum Token {
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    LPAREN,
    RPAREN,
    EOF,
    ILLEGAL,
    INT(isize),
    FLOAT(f64),
    STRING(String),
}
