#[derive(Debug, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Asterfisk,
    Slash,
    Lparen,
    Rparen,
    Eof,
    True,
    Nil,
    Illegal(String),
    Number(f64),
    String(String),
    Literal(String),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Self::Plus => "+".to_string(),
            Self::Minus => "-".to_string(),
            Self::Asterfisk => "*".to_string(),
            Self::Slash => "/".to_string(),
            Self::Lparen => "(".to_string(),
            Self::Rparen => ")".to_string(),
            Self::Eof => "EOF".to_string(),
            Self::True => "T".to_string(),
            Self::Nil => "NIL".to_string(),
            Self::Illegal(s) => format!("ILLEGAL({})", s),
            Self::Number(num) => num.to_string(),
            Self::String(s) => String::from(s),
            Self::Literal(s) => String::from(s),
        };

        write!(f, "{}", s)
    }
}
