use std::str;
use std::fmt;

pub fn tokenize(s: &str) -> Result<Vec<Token>, SyntaxError> {
    Lexer::tokenize(s)
}

#[derive(PartialEq, Debug)]
pub enum Token {
    OpenParen,
    CloseParen,
    Integer(usize),
    Operator(char),
}

struct Lexer<'a> {
    chars: str::Chars<'a>,
    current: Option<char>,
    tokens: Vec<Token>,
    column: u32,
}

pub struct SyntaxError {
    message: String,
    column: u32,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SyntaxError: {} (column: {})", self.message, self.column)
    }
}
impl fmt::Debug for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SyntaxError: {} (column: {})", self.message, self.column)
    }
}

macro_rules! syntax_error {
    ($lexer:ident, $($arg:tt)*) => (
        return Err(SyntaxError { message: format!($($arg)*), column: $lexer.column })
    )
}

impl<'a> Lexer<'a> {
    fn tokenize(s: &str) -> Result<Vec<Token>, SyntaxError> {
        let mut lexer = Lexer { chars: s.chars(), current: None, tokens: Vec::new(), column: 0 };
        lexer.run()?;
        Ok(lexer.tokens)
    }

    fn current(&self) -> Option<char> {
        self.current
    }

    fn advance(&mut self) {
        self.column += 1;
        self.current = self.chars.next();
    }

    fn run(&mut self) -> Result<(), SyntaxError> {
        self.advance();
        loop {
            match self.current() {
                Some(c) => {
                    match c {
                        _ if c.is_whitespace() => {
                            self.advance();
                        },
                        '(' => {
                            self.tokens.push(Token::OpenParen);
                            self.advance();
                        },
                        ')' => {
                            self.tokens.push(Token::CloseParen);
                            self.advance();
                        },
                        '+' | '*' => {
                            self.tokens.push(Token::Operator(c));
                            self.advance();
                            self.parse_delimiter()?;
                        },
                        '0'..='9' => {
                            // don't advance -- let parse_number advance as needed
                            let val = self.parse_number()?;
                            self.tokens.push(Token::Integer(val));
                            self.parse_delimiter()?;
                        },
                        _ => {
                            syntax_error!(self, "Unexpected character: {}", c);
                        },
                    }
                },
                None => break
            }
        };
        Ok(())
    }

    fn parse_number(&mut self) -> Result<usize, SyntaxError> {
        let mut s = String::new();
        loop {
            match self.current() {
                Some(c) => {
                    match c {
                        '0'..='9' => {
                            s.push(c);
                            self.advance();
                        },
                        _ => break
                    }
                },
                None => break
            }
        }
        match s.parse() {
            Ok(value) => Ok(value),
            Err(_) => { syntax_error!(self, "Not a number: {}", self.current().unwrap()); },
        }
    }

    fn parse_delimiter(&mut self) -> Result<(), SyntaxError> {
        match self.current() {
            Some(c) => {
                match c {
                    _ if c.is_whitespace() => (),
                    ')' => {
                        self.tokens.push(Token::CloseParen);
                        self.advance();
                    },
                    _ => syntax_error!(self, "Unexpected character when looking for a delimiter: {}", c),
                }
            },
            None => ()
        };
        Ok(())
    }
}

#[test]
fn test_lexer_simple_lexing() {
    assert_eq!(
        tokenize("1 + 2").unwrap(),
        vec![Token::Integer(1), Token::Operator('+'), Token::Integer(2)],
    );
}

#[test]
fn test_lexer_multi_digit_integers() {
    assert_eq!(
        tokenize("21 + 325").unwrap(),
        vec![Token::Integer(21), Token::Operator('+'), Token::Integer(325)],
    );
}

#[test]
fn test_lexer_multiplication() {
    assert_eq!(
        tokenize("1 * 2").unwrap(),
        vec![Token::Integer(1), Token::Operator('*'), Token::Integer(2)],
    );
}

#[test]
fn test_lexer_expression() {
    assert_eq!(
        tokenize("1 + (2 + 3)").unwrap(),
        vec![Token::Integer(1), Token::Operator('+'), Token::OpenParen, Token::Integer(2), Token::Operator('+'), Token::Integer(3), Token::CloseParen],
    );
}
