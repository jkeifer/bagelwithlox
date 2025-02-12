use crate::ast::Operator;

use super::ast::{AST, Expr};
use super::tokenizer::{Tokens, Token};


//pub type AST = ();


struct Parser<'a> {
    tokens: &'a Tokens<'a>,
    current: usize,
}

impl<'a> Parser<'a> {
    fn new (tokens: &'a Tokens) -> Parser<'a> {
        Parser {
          tokens,
          current: 0,
        }
    }

    //fn next(&mut self) -> Option<&Token> {
    //    let token = self.tokens.get(self.current);
    //    self.current += 1;
    //    token
    //}

    fn peek(&mut self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&mut self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() { self.current += 1 };
        self.previous()
    }

    fn is_at_end(&mut self) -> bool {
        match self.peek() {
            Token::Eof{ pos: _ } => true,
            _ => false,
        }
    }

    fn parse(&mut self) -> Result<AST, String> {
        let expr = self.expression()?;

        if self.current < self.tokens.len() {
            return Err("Failed to parse all tokens".to_string());
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let expr = self.comparison()?;

        if let Some(op) = match self.peek() {
            Token::BangEqual{ pos: _ } => Some(Operator::NotEqual),
            Token::EqualEqual{ pos: _ } => Some(Operator::Equal),
            _ => None,
        } {
            self.advance();
            Ok(Expr::BinOp { op, left: Box::new(expr), right: Box::new(self.comparison()?) })
        } else {
            Ok(expr)
        }
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let expr = self.term()?;

        if let Some(op) = match self.peek() {
            Token::Greater{ pos: _ } => Some(Operator::Greater),
            Token::GreaterEqual{ pos: _ } => Some(Operator::GreaterEqual),
            Token::Less{ pos: _ } => Some(Operator::Less),
            Token::LessEqual{ pos: _ } => Some(Operator::LessEqual),
            _ => None,
        } {
            self.advance();
            Ok(Expr::BinOp { op, left: Box::new(expr), right: Box::new(self.term()?) })
        } else {
            Ok(expr)
        }
    }

    fn term(&mut self) -> Result<Expr, String> {
        let expr = self.factor()?;

        if let Some(op) = match self.peek() {
            Token::Minus{ pos: _ } => Some(Operator::Sub),
            Token::Plus{ pos: _ } => Some(Operator::Add),
            _ => None,
        } {
            self.advance();
            Ok(Expr::BinOp { op, left: Box::new(expr), right: Box::new(self.factor()?) })
        } else {
            Ok(expr)
        }
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let expr = self.unary()?;

        if let Some(op) = match self.peek() {
            Token::Slash{ pos: _ } => Some(Operator::Div),
            Token::Star{ pos: _ } => Some(Operator::Mul),
            _ => None,
        } {
            Ok(Expr::BinOp { op, left: Box::new(expr), right: Box::new(self.unary()?) })
        } else {
            Ok(expr)
        }
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if let Some(op) = match self.peek() {
            Token::Bang{ pos: _ } => Some(Operator::Not),
            Token::Minus{ pos: _ } => Some(Operator::Negate),
            _ => None,
        } {
            self.advance();
            Ok(Expr::UnaryOp { op, operand: Box::new(self.unary()?) })
        } else {
            primary()?
        }
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if let Some(expr) = match self.peek() {
            Token::False { pos: _ } => Some(Expr::Bool { value: false }),
            Token::True { pos: _ } => Some(Expr::Bool { value: true }),
            Token::Nil{ pos: _ } => Some(Expr::Nil),
            Token::Number { pos: _, lexeme: _, value } => Some(Expr::Numb{ value: *value }),
            Token::Str { pos: _, lexeme: _, value } => Some(Expr::Str { value }),
            _ => None,
        } {
            self.advance();
            return Ok(expr);
        }

        self.group()
    }

    fn group(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Token::LeftParen { pos: _ } => {
                self.advance();
            },
            _ => { return Err(String::from("Failed to parse")); },
        }

        let expr = self.expression()?;
        match self.peek() {
            Token::RightParen { pos: _ } => (),
            _ => { return Err(String::from("Expect ')' after expression")); },
        }

        Ok(Expr::Group { expr: Box::new(expr) })
    }

    //fn parse_expression(&mut self) -> Result<Expr, String> {
    //    let left = Box::new(self.parse_term()?);
    //    let op = match self.currentext() {
    //        Some(token) => token,
    //        _ => return Ok(left),
    //    };
    //    match op {
    //        op if is_binary_operator => Ok(Expr::BinOp{
    //            op,
    //            left,
    //            right: Box::new(self.parse_term()?),
    //        }),
    //        Token::MINUS => Ok(Expr::Sub(Box::new(left), Box::new(self.parse_term()?))),
    //        Token::TIMES => Ok(Expr::Mul(Box::new(left), Box::new(self.parse_term()?))),
    //        Token::ASSIGN => Ok(Expr::Assign(Box::new(left), Box::new(self.parse_expression()?))),
    //        other => Err(format!("Unknown token: {}", other)),
    //    }
    //}

    //fn parse_term(&mut self) -> Result<Expression, Error> {
    //    let token = match self.currentext() {
    //        Some(token) => token,
    //        _ => return Err(
    //            Error::new(
    //                ErrorKind::InvalidInput,
    //                format!("Expected a term"),
    //            ),
    //        ),
    //    };
    //    match token {
    //        Token::NUM(val) => Ok(Expression::Number(*val)),
    //        Token::NAME(val) => Ok(Expression::Variable(val.to_owned())),
    //        Token::LPAREN => {
    //            let e = self.parse_expression();

    //            let token = match self.currentext() {
    //                Some(token) => token,
    //                _ => return Err(
    //                    Error::new(
    //                        ErrorKind::InvalidInput,
    //                        format!("Expected a )"),
    //                    ),
    //                ),
    //            };
    //            match token {
    //                Token::RPAREN => (),
    //                _ => return Err(
    //                    Error::new(
    //                        ErrorKind::InvalidInput,
    //                        format!("Expected a )"),
    //                    ),
    //                ),
    //            }

    //            e
    //        },
    //        _ => return Err(
    //            Error::new(
    //                ErrorKind::InvalidInput,
    //                format!("Expected a term"),
    //            ),
    //        ),
    //    }
    //}
}


pub fn parse(tokens: &Tokens) -> Result<AST, String> {
    //Parser::new(tokens).parse()
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
    }
}


