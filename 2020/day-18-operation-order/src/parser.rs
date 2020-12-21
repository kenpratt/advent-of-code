use std::iter;
use std::fmt;
use std::slice;

use crate::lexer::*;

pub fn parse(tokens: &Vec<Token>) -> Result<Expression, ParseError> {
    Parser::parse(tokens)
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
    //tokens: slice::Iter<'a, Token>,
    tokens: iter::Peekable<iter::Rev<slice::Iter<'a, Token>>>,
}

impl<'a> Parser<'a> {
    fn parse(tokens: &Vec<Token>) -> Result<Expression, ParseError> {
        let mut parser = Parser { tokens: tokens.iter().rev().peekable() };
        Ok(parser.parse_expression(0)?.unwrap())
    }

    // parses in reverse due to left-hand-side evalution
    fn parse_expression(&mut self, depth: u32) -> Result<Option<Expression>, ParseError> {
        match self.tokens.next() {
            Some(token) => {
                match *token {
                    Token::Integer(ref val) => {
                        let right = Expression::Integer(val.clone());
                        Ok(Some(self.parse_rest_of_expression(depth, right)?))
                    },
                    Token::CloseParen => {
                        let right = self.parse_expression(depth + 1)?.unwrap();
                        Ok(Some(self.parse_rest_of_expression(depth, right)?))
                    },
                    Token::OpenParen => {
                        if depth > 0 {
                            Ok(None)
                        } else {
                            parse_error!("Unexpected close paren, depth: {}", depth)
                        }
                    },
                    _ => {
                        parse_error!("Unexpected token in parse_expression: {:?}", token)
                    },                    
                }
            },
            None => {
                if depth == 0 {
                    Ok(None)
                } else {
                    parse_error!("Unexpected end of input, depth: {}", depth)
                }
            }
        }
    }

    fn parse_rest_of_expression(&mut self, depth: u32, right: Expression) -> Result<Expression, ParseError> {
        let next = self.tokens.next();
        match next {
            Some(Token::Operator(op)) => {
                let left = self.parse_expression(depth)?.unwrap();
                Ok(Expression::Operation(
                    op.clone(),
                    Box::new(left),
                    Box::new(right),
                ))
            },
            Some(Token::OpenParen) | None => {
                Ok(right)
            },
            _ => {
                parse_error!("Unexpected token for rest of expression: {:?}", next)
            },            
        }   
    }
}

#[test]
fn test_parser_simple() {
    assert_eq!(
        parse(&vec![Token::Integer(1), Token::Operator('+'), Token::Integer(2)]).unwrap(),
        Expression::Operation('+', Box::new(Expression::Integer(1)), Box::new(Expression::Integer(2))),
    );
}

#[test]
fn test_parser_single_term() {
    assert_eq!(
        parse(&vec![Token::Integer(1)]).unwrap(),
        Expression::Integer(1),
    );
}

#[test]
fn test_parser_multiple_operations_no_parens() {
    assert_eq!(
        // 1 + 2 * 3 + 4
        // => [[1 + 2] * 3] + 4
        parse(&vec![Token::Integer(1), Token::Operator('+'), Token::Integer(2), Token::Operator('*'), Token::Integer(3), Token::Operator('+'), Token::Integer(4)]).unwrap(),
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
fn test_parser_simple_parens() {
    assert_eq!(
        parse(&vec![Token::OpenParen, Token::Integer(1), Token::Operator('+'), Token::Integer(2), Token::CloseParen]).unwrap(),
        Expression::Operation('+', Box::new(Expression::Integer(1)), Box::new(Expression::Integer(2))),
    );
}

#[test]
fn test_parser_noop_parens() {
    assert_eq!(
        // (1 + 2) * 3
        parse(&vec![Token::OpenParen, Token::Integer(1), Token::Operator('+'), Token::Integer(2), Token::CloseParen, Token::Operator('*'), Token::Integer(3)]).unwrap(),
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
fn test_parser_meaningful_parens() {
    assert_eq!(
        // 1 + (2 * 3)
        parse(&vec![Token::Integer(1), Token::Operator('+'), Token::OpenParen, Token::Integer(2), Token::Operator('*'), Token::Integer(3), Token::CloseParen]).unwrap(),
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
fn test_parser_multiple_operations_parens_in_middle() {
    assert_eq!(
        // 1 + (2 * 3) + 4
        // => [1 + [2 * 3]] + 4
        parse(&vec![Token::Integer(1), Token::Operator('+'), Token::OpenParen, Token::Integer(2), Token::Operator('*'), Token::Integer(3), Token::CloseParen, Token::Operator('+'), Token::Integer(4)]).unwrap(),
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
