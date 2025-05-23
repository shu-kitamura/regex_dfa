use std::{fmt::Display, str::Chars};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    Character(char),
    UnionOperator,
    LeftParen,
    RightParen,
    StarOperator,
    EndOfFile,
}

pub struct Lexer<'a> {
    chars: Chars<'a>,
}

impl Lexer<'_> {
    pub fn new(string: &str) -> Lexer {
        Lexer {
            chars: string.chars(),
        }
    }

    pub fn scan(&mut self) -> Token {
        let Some(char) = self.chars.next() else {
            return Token::EndOfFile;
        };
        match char {
            '\\' => Token::Character(self.chars.next().unwrap()),
            '|' => Token::UnionOperator,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '*' => Token::StarOperator,
            _ => Token::Character(char),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Token::Character(_) => "Character",
            Token::UnionOperator => "|",
            Token::StarOperator => "*",
            Token::LeftParen => "(",
            Token::RightParen => ")",
            Token::EndOfFile => "EOF",
        };
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::*;

    #[test]
    fn scan() {
        let mut lexer = Lexer::new(r"a|(bc)*");
        assert_eq!(lexer.scan(), Token::Character('a'));
        assert_eq!(lexer.scan(), Token::UnionOperator);
        assert_eq!(lexer.scan(), Token::LeftParen);
        assert_eq!(lexer.scan(), Token::Character('b'));
        assert_eq!(lexer.scan(), Token::Character('c'));
        assert_eq!(lexer.scan(), Token::RightParen);
        assert_eq!(lexer.scan(), Token::StarOperator);
        assert_eq!(lexer.scan(), Token::EndOfFile);
    }

    #[test]
    fn scan_with_escape() {
        let mut lexer = Lexer::new(r"a|\|\\(\)");
        assert_eq!(lexer.scan(), Token::Character('a'));
        assert_eq!(lexer.scan(), Token::UnionOperator);
        assert_eq!(lexer.scan(), Token::Character('|'));
        assert_eq!(lexer.scan(), Token::Character('\\'));
        assert_eq!(lexer.scan(), Token::LeftParen);
        assert_eq!(lexer.scan(), Token::Character(')'));
        assert_eq!(lexer.scan(), Token::EndOfFile);
    }

    #[test]
    fn with_empty() {
        let mut lexer = Lexer::new(r#""#);
        assert_eq!(lexer.scan(), Token::EndOfFile);
    }
}
