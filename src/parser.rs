use std::iter::Peekable;
use crate::source::FilePosition;
use super::source::SourceError;
use super::ast::{AST, Expr, Operator};
use super::tokenizer::{Tokens, Token};
use super::tokenizer::TokenType::*;


const PARSE_ERROR: &'static str = "ParseError";


#[derive(Debug)]
pub struct ParseError {
    pos: Option<FilePosition>,
    msg: String,
}

impl SourceError for ParseError {
    fn get_message(&self) -> &str {
        &self.msg
    }

    fn get_position(&self) -> Option<FilePosition> {
        self.pos
    }

    fn get_type(&self) -> &str {
        PARSE_ERROR
    }
}

impl ParseError {
    fn new(pos: FilePosition, msg: String) -> ParseError {
        ParseError {
            pos: Some(pos),
            msg,
        }
    }
}


pub fn parse<'a>(tokens: &'a Tokens<'a>) -> Result<AST<'a>, ParseError> {
    let mut token_iter = tokens.iter().peekable();

    let expr = expression(&mut token_iter)?;

    match token_iter.peek() {
        Some(token) if *token.get_type() == Eof => (),
        None => (),
        _ => {
            return Err(ParseError {
                pos:None,
                msg: "Failed to parse all tokens".to_string(),
            });
        },
    }

    Ok(AST::new(expr ))
}


fn expression<'a, 'b, I>(token_iter: &'a mut Peekable<I>) -> Result<Expr<'b>, ParseError>
where
    I: Iterator<Item = &'b Token<'b>>
{
    equality(token_iter)
}


fn _equality<'a, 'b, I>(token_iter: &'a mut Peekable<I>) -> Option<Operator>
where
    I: Iterator<Item = &'b Token<'b>>
{
    let token = token_iter.peek()?;
    match token.get_type() {
        BangEqual => Some(Operator::NotEqual),
        EqualEqual => Some(Operator::Equal),
        _ => None,
    }
}


fn equality<'a, I>(token_iter: &mut Peekable<I>) -> Result<Expr<'a>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let expr = comparison(token_iter)?;

    match _equality(token_iter) {
        Some(op) => {
            token_iter.next();
            Ok(Expr::BinOp { op, left: Box::new(expr), right: Box::new(comparison(token_iter)?) })
        },
        None => Ok(expr),
    }
}


fn _comparison<'a, 'b, I>(token_iter: &'a mut Peekable<I>) -> Option<Operator>
where
    I: Iterator<Item = &'b Token<'b>>
{
    let token = token_iter.peek()?;
    match token.get_type() {
        Greater => Some(Operator::Greater),
        GreaterEqual => Some(Operator::GreaterEqual),
        Less => Some(Operator::Less),
        LessEqual => Some(Operator::LessEqual),
        _ => None,
    }
}


fn comparison<'a, I>(token_iter: &mut Peekable<I>) -> Result<Expr<'a>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let expr = term(token_iter)?;

    match _comparison(token_iter) {
        Some(op) => {
            token_iter.next();
            Ok(Expr::BinOp { op, left: Box::new(expr), right: Box::new(term(token_iter)?) })
        },
        None => Ok(expr),
    }
}


fn _term<'b, I>(token_iter: &mut Peekable<I>) -> Option<Operator>
where
    I: Iterator<Item = &'b Token<'b>>
{
    let token = token_iter.peek()?;
    match token.get_type() {
        Minus => Some(Operator::Sub),
        Plus => Some(Operator::Add),
        _ => None,
    }
}


fn term<'a, 'b, I>(token_iter: &mut Peekable<I>) -> Result<Expr<'b>, ParseError>
where
    I: Iterator<Item = &'b Token<'b>>
{
    let expr = factor(token_iter)?;

    match _term(token_iter) {
        Some(op) => {
            token_iter.next();
            Ok(Expr::BinOp { op, left: Box::new(expr), right: Box::new(factor(token_iter)?) })
        },
        None => Ok(expr),
    }
}


fn _factor<'b, I>(token_iter: &mut Peekable<I>) -> Option<Operator>
where
    I: Iterator<Item = &'b Token<'b>>
{
    let token = token_iter.peek()?;
    match token.get_type() {
        Slash => Some(Operator::Div),
        Star => Some(Operator::Mul),
        _ => None,
    }
}


fn factor<'a, 'b, I>(token_iter: &mut Peekable<I>) -> Result<Expr<'b>, ParseError>
where
    I: Iterator<Item = &'b Token<'b>>
{
    let expr = unary(token_iter)?;

    match _factor(token_iter) {
        Some(op) => {
            token_iter.next();
            Ok(Expr::BinOp { op, left: Box::new(expr), right: Box::new(unary(token_iter)?) })
        },
        None => Ok(expr),
    }
}


fn _unary<'b, I>(token_iter: &mut Peekable<I>) -> Option<Operator>
where
    I: Iterator<Item = &'b Token<'b>>
{
    let token = token_iter.peek()?;
    match token.get_type() {
        Bang => Some(Operator::Not),
        Minus => Some(Operator::Negate),
        _ => None,
    }
}


fn unary<'a, 'b, I>(token_iter: &'a mut Peekable<I>) -> Result<Expr<'b>, ParseError>
where
    I: Iterator<Item = &'b Token<'b>>
{
    match _unary(token_iter) {
        Some(op) => {
            token_iter.next();
            Ok(Expr::UnaryOp { op, operand: Box::new(unary(token_iter)?) })
        },
        None => primary(token_iter),
    }
}


fn _primary<'b, I>(token_iter: &mut Peekable<I>) -> Option<Expr<'b>>
where
    I: Iterator<Item = &'b Token<'b>>
{
    let token = token_iter.peek()?;
    match token.get_type() {
        False => Some(Expr::Bool { value: false }),
        True => Some(Expr::Bool { value: true }),
        Nil => Some(Expr::Nil),
        Number { lexeme: _, value } => Some(Expr::Numb{ value: *value }),
        Str { lexeme: _, value } => Some(Expr::Str { value }),
        _ => None,
    }
}


fn primary<'a, 'b, I>(token_iter: &'a mut Peekable<I>) -> Result<Expr<'b>, ParseError>
where
    I: Iterator<Item = &'b Token<'b>>
{
    match _primary(token_iter) {
        Some(expr) => {
            token_iter.next();
            return Ok(expr);
        },
        None => group(token_iter),
    }
}


fn group<'a, 'b, I>(token_iter: &'a mut Peekable<I>) -> Result<Expr<'b>, ParseError>
where
    I: Iterator<Item = &'b Token<'b>>
{
    match token_iter.peek() {
        Some(token) if *token.get_type() == LeftParen => {
            token_iter.next();
        },
        Some(token) => {
            return Err(
                ParseError::new(
                    token.get_position(),
                    format!("could not parse token type '{}'", token.get_type()),
                ),
            );
        },
        None => {
            return Err(
                ParseError {
                    pos: None,
                    msg: "invalid expression".to_string(),
                },
            )
        },
    }

    let expr = expression(token_iter)?;
    match token_iter.peek() {
        Some(token) if *token.get_type() == RightParen => { token_iter.next(); },
        Some(other) => {
            return Err(ParseError::new(
                other.get_position(),
                "Expected ')' after expression".to_string(),
            ));
        },
        None => {
            return Err(
                ParseError {
                    pos: None,
                    msg: "Missing ')' at EOF".to_string(),
                }
            );
        },
    }

    Ok(Expr::Group { expr: Box::new(expr) })
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::FilePosition;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_add() {
        let tokens = vec![
            Token::new(Number{ value: 11.12, lexeme: "11.12"}, FilePosition::new(2, 26)),
            Token::new(Plus, FilePosition::new(1, 9)),
            Token::new(Number{ value: 12.0, lexeme: "12"}, FilePosition::new(2, 26)),
        ];

        let ast = parse(&tokens).unwrap();

        assert_eq!(
            ast,
            AST::new(
                Expr::BinOp {
                    op: Operator::Add,
                    left: Box::new(Expr::Numb { value: 11.12 }),
                    right: Box::new(Expr::Numb { value: 12.0 }),
                },
            ),
        );
    }

    #[test]
    fn test_precidence_mul_over_add_1() {
        let tokens = vec![
            Token::new(Number{ value: 11.12, lexeme: "11.12"}, FilePosition::new(2, 26)),
            Token::new(Plus, FilePosition::new(1, 9)),
            Token::new(Number{ value: 12.0, lexeme: "12"}, FilePosition::new(2, 26)),
            Token::new(Star, FilePosition::new(1, 9)),
            Token::new(Number{ value: 3.0, lexeme: "12"}, FilePosition::new(2, 26)),
        ];

        let ast = parse(&tokens).unwrap();

        assert_eq!(
            ast,
            AST::new(
                Expr::BinOp {
                    op: Operator::Add,
                    left: Box::new(Expr::Numb { value: 11.12 }),
                    right: Box::new(
                        Expr::BinOp {
                            op: Operator::Mul,
                            left: Box::new(Expr::Numb { value: 12.0 }),
                            right: Box::new(Expr::Numb { value: 3.0 }),
                        },
                    ),
                },
            ),
        );
    }

    #[test]
    fn test_precidence_mul_over_add_2() {
        let tokens = vec![
            Token::new(Number{ value: 11.12, lexeme: "11.12"}, FilePosition::new(2, 26)),
            Token::new(Star, FilePosition::new(1, 9)),
            Token::new(Number{ value: 12.0, lexeme: "12"}, FilePosition::new(2, 26)),
            Token::new(Plus, FilePosition::new(1, 9)),
            Token::new(Number{ value: 3.0, lexeme: "12"}, FilePosition::new(2, 26)),
        ];

        let ast = parse(&tokens).unwrap();

        assert_eq!(
            ast,
            AST::new(
                Expr::BinOp {
                    op: Operator::Add,
                    left: Box::new(
                        Expr::BinOp {
                            op: Operator::Mul,
                            left: Box::new(Expr::Numb { value: 11.12 }),
                            right: Box::new(Expr::Numb { value: 12.0 }),
                        },
                    ),
                    right: Box::new(Expr::Numb { value: 3.0 }),
                },
            ),
        );
    }

    #[test]
    fn test_grouping() {
        let tokens = vec![
            Token::new(Number{ value: 11.12, lexeme: "11.12"}, FilePosition::new(2, 26)),
            Token::new(Star, FilePosition::new(1, 9)),
            Token::new(LeftParen, FilePosition::new(1, 9)),
            Token::new(Number{ value: 12.0, lexeme: "12"}, FilePosition::new(2, 26)),
            Token::new(Plus, FilePosition::new(1, 9)),
            Token::new(Number{ value: 3.0, lexeme: "12"}, FilePosition::new(2, 26)),
            Token::new(RightParen, FilePosition::new(1, 9)),
            Token::new(Eof, FilePosition::new(12, 1)),
        ];

        let ast = parse(&tokens).unwrap();

        assert_eq!(
            ast,
            AST::new(
                Expr::BinOp {
                    op: Operator::Mul,
                    left: Box::new(Expr::Numb { value: 11.12 }),
                    right: Box::new(
                        Expr::Group{
                            expr: Box::new(
                                Expr::BinOp {
                                    op: Operator::Add,
                                    left: Box::new(Expr::Numb { value: 12.0 }),
                                    right: Box::new(Expr::Numb { value: 3.0 }),
                                },
                            ),
                        },
                    ),
                },
            ),
        );
    }
}
