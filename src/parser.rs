use std::iter::Peekable;
use super::ast::{AST, Expr, Operator};
use super::tokenizer::{Tokens, Token};


pub fn parse<'a>(tokens: &'a Tokens<'a>) -> Result<AST<'a>, String> {
    let mut token_iter = tokens.iter().peekable();

    let expr = expression(&mut token_iter)?;

    if let Some(_) = token_iter.peek() {
        return Err(String::from("Failed to parse all tokens"));
    }

    Ok(AST::new(expr ))
}


fn expression<'a, 'b, I>(token_iter: &'a mut Peekable<I>) -> Result<Expr<'b>, String>
where
    I: Iterator<Item = &'b Token<'b>>
{
    equality(token_iter)
}


fn equality<'a, 'b, I>(token_iter: &'a mut Peekable<I>) -> Result<Expr<'b>, String>
where
    I: Iterator<Item = &'b Token<'b>>
{
    let expr = comparison(token_iter)?;

    if let Some(op) = match token_iter.peek() {
        Some(Token::BangEqual{ pos: _ }) => Some(Operator::NotEqual),
        Some(Token::EqualEqual{ pos: _ }) => Some(Operator::Equal),
        _ => None,
    } {
        token_iter.next();
        Ok(Expr::BinOp { op, left: Box::new(expr), right: Box::new(comparison(token_iter)?) })
    } else {
        Ok(expr)
    }
}


fn comparison<'a, 'b, I>(token_iter: &'a mut Peekable<I>) -> Result<Expr<'b>, String>
where
    I: Iterator<Item = &'b Token<'b>>
{
    let expr = term(token_iter)?;

    if let Some(op) = match token_iter.peek() {
        Some(Token::Greater{ pos: _ }) => Some(Operator::Greater),
        Some(Token::GreaterEqual{ pos: _ }) => Some(Operator::GreaterEqual),
        Some(Token::Less{ pos: _ }) => Some(Operator::Less),
        Some(Token::LessEqual{ pos: _ }) => Some(Operator::LessEqual),
        _ => None,
    } {
        token_iter.next();
        Ok(Expr::BinOp { op, left: Box::new(expr), right: Box::new(term(token_iter)?) })
    } else {
        Ok(expr)
    }
}


fn term<'a, 'b, I>(token_iter: &mut Peekable<I>) -> Result<Expr<'b>, String>
where
    I: Iterator<Item = &'b Token<'b>>
{
    let expr = factor(token_iter)?;

    if let Some(op) = match token_iter.peek() {
        Some(Token::Minus{ pos: _ }) => Some(Operator::Sub),
        Some(Token::Plus{ pos: _ }) => Some(Operator::Add),
        _ => None,
    } {
        token_iter.next();
        Ok(Expr::BinOp { op, left: Box::new(expr), right: Box::new(factor(token_iter)?) })
    } else {
        Ok(expr)
    }
}


fn factor<'a, 'b, I>(token_iter: &'a mut Peekable<I>) -> Result<Expr<'b>, String>
where
    I: Iterator<Item = &'b Token<'b>>
{
    let expr = unary(token_iter)?;

    if let Some(op) = match token_iter.peek() {
        Some(Token::Slash{ pos: _ }) => Some(Operator::Div),
        Some(Token::Star{ pos: _ }) => Some(Operator::Mul),
        _ => None,
    } {
        token_iter.next();
        Ok(Expr::BinOp { op, left: Box::new(expr), right: Box::new(unary(token_iter)?) })
    } else {
        Ok(expr)
    }
}


fn unary<'a, 'b, I>(token_iter: &'a mut Peekable<I>) -> Result<Expr<'b>, String>
where
    I: Iterator<Item = &'b Token<'b>>
{
    if let Some(op) = match token_iter.peek() {
        Some(Token::Bang{ pos: _ }) => Some(Operator::Not),
        Some(Token::Minus{ pos: _ }) => Some(Operator::Negate),
        _ => None,
    } {
        token_iter.next();
        Ok(Expr::UnaryOp { op, operand: Box::new(unary(token_iter)?) })
    } else {
        primary(token_iter)
    }
}


fn primary<'a, 'b, I>(token_iter: &'a mut Peekable<I>) -> Result<Expr<'b>, String>
where
    I: Iterator<Item = &'b Token<'b>>
{
    if let Some(expr) = match token_iter.peek() {
        Some(Token::False { pos: _ }) => Some(Expr::Bool { value: false }),
        Some(Token::True { pos: _ }) => Some(Expr::Bool { value: true }),
        Some(Token::Nil{ pos: _ }) => Some(Expr::Nil),
        Some(Token::Number { pos: _, lexeme: _, value }) => Some(Expr::Numb{ value: *value }),
        Some(Token::Str { pos: _, lexeme: _, value }) => Some(Expr::Str { value }),
        _ => None,
    } {
        token_iter.next();
        return Ok(expr);
    }

    group(token_iter)
}


fn group<'a, 'b, I>(token_iter: &'a mut Peekable<I>) -> Result<Expr<'b>, String>
where
    I: Iterator<Item = &'b Token<'b>>
{
    match token_iter.peek() {
        Some(Token::LeftParen { pos: _ }) => {
            token_iter.next();
        },
        other => {
            return Err(format!("Failed to parse: {:#?}", other));
        },
    }

    let expr = expression(token_iter)?;
    match token_iter.peek() {
        Some(Token::RightParen { pos: _ }) => { token_iter.next(); },
        _ => { return Err(String::from("Expect ')' after expression")); },
    }

    Ok(Expr::Group { expr: Box::new(expr) })
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::tokenizer::FilePosition;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_add() {
        let tokens = vec![
            Token::Number{ value: 11.12, lexeme: "11.12", pos: FilePosition::new(2, 26) },
            Token::Plus{ pos: FilePosition::new(1, 9) },
            Token::Number{ value: 12.0, lexeme: "12", pos: FilePosition::new(2, 26) },
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
            Token::Number{ value: 11.12, lexeme: "11.12", pos: FilePosition::new(2, 26) },
            Token::Plus{ pos: FilePosition::new(1, 9) },
            Token::Number{ value: 12.0, lexeme: "12", pos: FilePosition::new(2, 26) },
            Token::Star{ pos: FilePosition::new(1, 9) },
            Token::Number{ value: 3.0, lexeme: "12", pos: FilePosition::new(2, 26) },
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
            Token::Number{ value: 11.12, lexeme: "11.12", pos: FilePosition::new(2, 26) },
            Token::Star{ pos: FilePosition::new(1, 9) },
            Token::Number{ value: 12.0, lexeme: "12", pos: FilePosition::new(2, 26) },
            Token::Plus{ pos: FilePosition::new(1, 9) },
            Token::Number{ value: 3.0, lexeme: "12", pos: FilePosition::new(2, 26) },
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
            Token::Number{ value: 11.12, lexeme: "11.12", pos: FilePosition::new(2, 26) },
            Token::Star{ pos: FilePosition::new(1, 9) },
            Token::LeftParen { pos: FilePosition::new(1, 9) },
            Token::Number{ value: 12.0, lexeme: "12", pos: FilePosition::new(2, 26) },
            Token::Plus{ pos: FilePosition::new(1, 9) },
            Token::Number{ value: 3.0, lexeme: "12", pos: FilePosition::new(2, 26) },
            Token::RightParen { pos: FilePosition::new(1, 9) },
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
