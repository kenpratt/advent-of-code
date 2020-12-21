use std::iter;
use std::fmt;
use std::slice;

use crate::lexer::*;

pub fn parse(tokens: &Vec<Token>, use_operator_precedence: bool) -> Result<Expression, ParseError> {
    Parser::parse(tokens, use_operator_precedence)
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    Integer(usize),
    Operation(char, Box<Expression>, Box<Expression>),
}

pub struct ParseError {
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: {}", self.message)
    }
}
impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: {}", self.message)
    }
}

macro_rules! parse_error {
    ($($arg:tt)*) => (
        return Err(ParseError { message: format!($($arg)*)})
    )
}

struct Parser<'a> {
    tokens: iter::Peekable<slice::Iter<'a, Token>>,
    use_operator_precedence: bool,
}

impl<'a> Parser<'a> {
    fn parse(tokens: &Vec<Token>, use_operator_precedence: bool) -> Result<Expression, ParseError> {
        let mut parser = Parser {
            tokens: tokens.iter().peekable(),
            use_operator_precedence: use_operator_precedence,
        };
        parser.parse_expression(1)
    }

    fn parse_expression(&mut self, min_precedence: u8) -> Result<Expression, ParseError> {
        let mut left = self.parse_term()?;
    
        loop {
            match self.tokens.peek() {
                Some(Token::Operator(operator)) => {
                    if min_precedence > self.precedence_for_operator(operator) {
                        break;
                    } else {
                        self.tokens.next(); // advance
                        let right = self.parse_expression(min_precedence + 1)?;

                        let new_left = Expression::Operation(
                            *operator,
                            Box::new(left),
                            Box::new(right),
                        );

                        left = new_left;
                    }
                },
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expression, ParseError> {
        match self.tokens.next() {
            Some(Token::Integer(value)) => Ok(Expression::Integer(*value)),
            Some(Token::OpenParen) => {
                let inner = self.parse_expression(1)?;
                match self.tokens.next() {
                    Some(Token::CloseParen) => Ok(inner),
                    _ => parse_error!("Unmatched open paren in term parsing"),
                }
            },
            Some(token) => parse_error!("Unexpected token in term parsing: {:?}", token),
            None => parse_error!("Unexpected end of input in term parsing"),
        }
    }

    fn precedence_for_operator(&self, operator: &char) -> u8 {
        if self.use_operator_precedence && operator == &'+' {
            2
        } else {
            1
        }
    }
}

#[test]
fn test_parser_simple() {
    assert_eq!(
        parse(&vec![Token::Integer(1), Token::Operator('+'), Token::Integer(2)], false).unwrap(),
        Expression::Operation('+', Box::new(Expression::Integer(1)), Box::new(Expression::Integer(2))),
    );
}

#[test]
fn test_parser_single_term() {
    assert_eq!(
        parse(&vec![Token::Integer(1)], false).unwrap(),
        Expression::Integer(1),
    );
}

#[test]
fn test_parser_multiple_operations_no_parens_no_precedence() {
    assert_eq!(
        // 1 + 2 * 3 + 4
        // => [[[1 + 2] * 3] + 4]
        parse(&vec![Token::Integer(1), Token::Operator('+'), Token::Integer(2), Token::Operator('*'), Token::Integer(3), Token::Operator('+'), Token::Integer(4)], false).unwrap(),
        Expression::Operation(
            '+',
            Box::new(Expression::Operation(
                '*',
                Box::new(Expression::Operation(
                    '+',
                    Box::new(Expression::Integer(1)),
                    Box::new(Expression::Integer(2)),
                )),                
                Box::new(Expression::Integer(3)),
            )),
            Box::new(Expression::Integer(4)),
        ),
    );
}

#[test]
fn test_parser_multiple_operations_no_parens_with_precedence() {
    assert_eq!(
        // 1 + 2 * 3 + 4
        // => [[1 + 2] * [3 + 4]]
        parse(&vec![Token::Integer(1), Token::Operator('+'), Token::Integer(2), Token::Operator('*'), Token::Integer(3), Token::Operator('+'), Token::Integer(4)], true).unwrap(),
        Expression::Operation(
            '*',
            Box::new(Expression::Operation(
                '+',           
                Box::new(Expression::Integer(1)),
                Box::new(Expression::Integer(2)),
            )),
            Box::new(Expression::Operation(
                '+',           
                Box::new(Expression::Integer(3)),
                Box::new(Expression::Integer(4)),
            )),
        ),
    );
}

#[test]
fn test_parser_simple_parens() {
    assert_eq!(
        parse(&vec![Token::OpenParen, Token::Integer(1), Token::Operator('+'), Token::Integer(2), Token::CloseParen], false).unwrap(),
        Expression::Operation('+', Box::new(Expression::Integer(1)), Box::new(Expression::Integer(2))),
    );
}

#[test]
fn test_parser_noop_parens_no_precedence() {
    assert_eq!(
        // (1 + 2) * 3
        parse(&vec![Token::OpenParen, Token::Integer(1), Token::Operator('+'), Token::Integer(2), Token::CloseParen, Token::Operator('*'), Token::Integer(3)], false).unwrap(),
        Expression::Operation(
            '*',
            Box::new(Expression::Operation(
                '+',
                Box::new(Expression::Integer(1)),
                Box::new(Expression::Integer(2)),
            )),
            Box::new(Expression::Integer(3)),
        ),
    );
}

#[test]
fn test_parser_meaningful_parens_with_precedence() {
    assert_eq!(
        // 1 + (2 * 3)
        parse(&vec![Token::Integer(1), Token::Operator('+'), Token::OpenParen, Token::Integer(2), Token::Operator('*'), Token::Integer(3), Token::CloseParen], true).unwrap(),
        Expression::Operation(
            '+',
            Box::new(Expression::Integer(1)),
            Box::new(Expression::Operation(
                '*',
                Box::new(Expression::Integer(2)),
                Box::new(Expression::Integer(3)),
            )),
        ),
    );
}

#[test]
fn test_parser_multiple_operations_parens_in_middle_no_precedence() {
    assert_eq!(
        // 1 + (2 * 3) + 4
        // => [[1 + [2 * 3]] + 4]
        parse(&vec![Token::Integer(1), Token::Operator('+'), Token::OpenParen, Token::Integer(2), Token::Operator('*'), Token::Integer(3), Token::CloseParen, Token::Operator('+'), Token::Integer(4)], false).unwrap(),
        Expression::Operation(
            '+',
            Box::new(Expression::Operation(
                '+',
                Box::new(Expression::Integer(1)),
                Box::new(Expression::Operation(
                    '*',
                    Box::new(Expression::Integer(2)),
                    Box::new(Expression::Integer(3)),
                )),                
            )),
            Box::new(Expression::Integer(4)),
        ),
    );
}