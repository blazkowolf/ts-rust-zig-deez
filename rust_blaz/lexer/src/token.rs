use std::fmt::Display;

pub mod literal {
    pub const ILLEGAL: &str = "";
    pub const EOF: &str = "\0";
    pub const ASSIGN: &str = "=";
    pub const PLUS: &str = "+";
    pub const MINUS: &str = "-";
    pub const BANG: &str = "!";
    pub const ASTERISK: &str = "*";
    pub const SLASH: &str = "/";
    pub const LESS_THAN: &str = "<";
    pub const GREATER_THAN: &str = ">";
    pub const EQUAL: &str = "==";
    pub const NOT_EQUAL: &str = "!=";
    pub const COMMA: &str = ",";
    pub const SEMICOLON: &str = ";";
    pub const LEFT_PAREN: &str = "(";
    pub const RIGHT_PAREN: &str = ")";
    pub const LEFT_BRACE: &str = "{";
    pub const RIGHT_BRACE: &str = "}";
    pub const FUNCTION: &str = "fn";
    pub const LET: &str = "let";
    pub const TRUE: &str = "true";
    pub const FALSE: &str = "false";
    pub const IF: &str = "if";
    pub const ELSE: &str = "else";
    pub const RETURN: &str = "return";
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,
    // Identifiers + literals
    Ident(String),
    Int(String),
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Token {
    #[must_use]
    pub fn token_type(&self) -> &str {
        match self {
            Self::Illegal => "Illegal",
            Self::Eof => "Eof",
            Self::Ident(_) => "Identifier",
            Self::Int(_) => "Integer",
            Self::Assign => "Assign",
            Self::Plus => "Plus",
            Self::Minus => "Minus",
            Self::Bang => "Bang",
            Self::Asterisk => "Asterisk",
            Self::Slash => "Slash",
            Self::LessThan => "LessThan",
            Self::GreaterThan => "GreaterThan",
            Self::Equal => "Equal",
            Self::NotEqual => "NotEqual",
            Self::Comma => "Comma",
            Self::Semicolon => "Semicolon",
            Self::LParen => "LeftParen",
            Self::RParen => "RightParen",
            Self::LBrace => "LeftBrace",
            Self::RBrace => "RightBrace",
            Self::Function => "Function",
            Self::Let => "Let",
            Self::True => "True",
            Self::False => "False",
            Self::If => "If",
            Self::Else => "Else",
            Self::Return => "Return",
        }
    }

    #[must_use]
    pub fn literal(&self) -> &str {
        use literal::*;

        match self {
            Self::Illegal => ILLEGAL,
            Self::Eof => EOF,
            Self::Ident(value) | Self::Int(value) => value,
            Self::Assign => ASSIGN,
            Self::Plus => PLUS,
            Self::Minus => MINUS,
            Self::Bang => BANG,
            Self::Asterisk => ASTERISK,
            Self::Slash => SLASH,
            Self::LessThan => LESS_THAN,
            Self::GreaterThan => GREATER_THAN,
            Self::Equal => EQUAL,
            Self::NotEqual => NOT_EQUAL,
            Self::Comma => COMMA,
            Self::Semicolon => SEMICOLON,
            Self::LParen => LEFT_PAREN,
            Self::RParen => RIGHT_PAREN,
            Self::LBrace => LEFT_BRACE,
            Self::RBrace => RIGHT_BRACE,
            Self::Function => FUNCTION,
            Self::Let => LET,
            Self::True => TRUE,
            Self::False => FALSE,
            Self::If => IF,
            Self::Else => ELSE,
            Self::Return => RETURN,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use literal::*;

        write!(
            f,
            "{{type: {}, literal: \"{}\"}}",
            self.token_type(),
            self.literal()
        )
    }
}

impl From<char> for Token {
    fn from(ch: char) -> Self {
        match ch {
            '=' => Self::Assign,
            ';' => Self::Semicolon,
            '(' => Self::LParen,
            ')' => Self::RParen,
            ',' => Self::Comma,
            '+' => Self::Plus,
            '-' => Self::Minus,
            '!' => Self::Bang,
            '*' => Self::Asterisk,
            '/' => Self::Slash,
            '<' => Self::LessThan,
            '>' => Self::GreaterThan,
            '{' => Self::LBrace,
            '}' => Self::RBrace,
            '\0' => Self::Eof,
            _ => Self::Illegal,
        }
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        use literal::{ELSE, EQUAL, FALSE, FUNCTION, IF, LET, NOT_EQUAL, RETURN, TRUE};

        match value.as_str() {
            EQUAL => Self::Equal,
            NOT_EQUAL => Self::NotEqual,
            FUNCTION => Self::Function,
            LET => Self::Let,
            TRUE => Self::True,
            FALSE => Self::False,
            IF => Self::If,
            ELSE => Self::Else,
            RETURN => Self::Return,
            _ => {
                if value.chars().all(|b| b.is_ascii_digit()) {
                    Self::Int(value)
                } else {
                    Self::Ident(value)
                }
            }
        }
    }
}
