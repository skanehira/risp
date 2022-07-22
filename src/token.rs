#[derive(Debug, PartialEq)]
pub enum Token {
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    LPAREN,
    RPAREN,
    EOF,
    TRUE,
    NIL,
    ILLEGAL(String),
    NUMBER(f64),
    STRING(String),
    LITERAL(String),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Self::PLUS => "+".to_string(),
            Self::MINUS => "-".to_string(),
            Self::ASTERISK => "*".to_string(),
            Self::SLASH => "/".to_string(),
            Self::LPAREN => "(".to_string(),
            Self::RPAREN => ")".to_string(),
            Self::EOF => "EOF".to_string(),
            Self::TRUE => "T".to_string(),
            Self::NIL => "NIL".to_string(),
            Self::ILLEGAL(s) => format!("ILLEGAL({})", s),
            Self::NUMBER(num) => num.to_string(),
            Self::STRING(s) => String::from(s),
            Self::LITERAL(s) => String::from(s),
        };

        write!(f, "{}", s)
    }
}
