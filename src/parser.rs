use prev_iter::PrevPeekable;
use crate::source::FilePosition;
use crate::tokenizer::LiteralValue;
use super::source::SourceError;
use super::ast::{AST, Expr, Operator, Stmt};
use super::tokenizer::{Tokens, Token, TokenType};
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
    let mut ast = AST::new();
    let mut token_iter = PrevPeekable::new(tokens.iter());

    while let Some(token) = token_iter.peek() {
        ast.top.push(declaration(token, &mut token_iter)?);
    }

    Ok(ast)
}


pub fn parse_expr<'a>(tokens: &'a Tokens<'a>) -> Result<Expr<'a>, ParseError> {
    let mut token_iter = PrevPeekable::new(tokens.iter());

    let expr = expression(&mut token_iter)?;

    match token_iter.peek() {
        None => (),
        _ => {
            return Err(ParseError {
                pos:None,
                msg: "Failed to parse all tokens".to_string(),
            });
        },
    }

    Ok(expr)
}


fn declaration<'a, I>(token: &'a Token, token_iter: &mut PrevPeekable<I>) -> Result<Stmt<'a>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    match token.get_type() {
        Var => var_declaration(token_iter),
        _ => statement(token, token_iter),
    }
}


fn var_declaration<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Stmt<'a>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    token_iter.next();

    let id = expect(token_iter, Identifier)?;
    let init = match token_iter.peek() {
        Some(token) => match token.get_type() {
            Equal => {
                token_iter.next();
                Some(expression(token_iter)?)
            },
            _ => None,
        },
        None => None,
    };

    expect(token_iter, SemiColon)?;
    Ok(Stmt::SVar{ name: id.lexeme, value: init })
}


fn statement<'a, I>(token: &'a Token, token_iter: &mut PrevPeekable<I>) -> Result<Stmt<'a>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    match token.get_type() {
        Print => print_statement(token_iter),
        _ => expression_statement(token_iter),
    }
}


fn print_statement<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Stmt<'a>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    token_iter.next();
    let expr = expression(token_iter)?;
    expect(token_iter, SemiColon)?;
    Ok(Stmt::SPrint(expr))
}


fn expression_statement<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Stmt<'a>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let expr = expression(token_iter)?;
    expect(token_iter, SemiColon)?;
    Ok(Stmt::SExprStmt(expr))
}


fn expression<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr<'a>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    assignment(token_iter)
}


fn assignment<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr<'a>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let expr = equality(token_iter)?;

    let token = match token_iter.peek() {
        Some(v) => v,
        None => return Ok(expr),
    };

    match (expr, token.get_type()) {
        (Expr::EVar { name }, Equal) => {
            token_iter.next();
            Ok(Expr::EAssign { name, expr: Box::new(assignment(token_iter)?) })
        },
        (_, Equal) => Err(ParseError::new(
            token.pos,
            "Invalid assignment target".to_string(),
        )),
        (expr, _) => Ok(expr),
    }
}


fn _equality<'a, I>(token_iter: &mut PrevPeekable<I>) -> Option<Operator>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let token = token_iter.peek()?;
    match token.get_type() {
        BangEqual => Some(Operator::NotEqual),
        EqualEqual => Some(Operator::Equal),
        _ => None,
    }
}


fn equality<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr<'a>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let expr = comparison(token_iter)?;

    match _equality(token_iter) {
        Some(op) => {
            token_iter.next();
            Ok(Expr::EBinOp { op, left: Box::new(expr), right: Box::new(comparison(token_iter)?) })
        },
        None => Ok(expr),
    }
}


fn _comparison<'a, I>(token_iter: &mut PrevPeekable<I>) -> Option<Operator>
where
    I: Iterator<Item = &'a Token<'a>>
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


fn comparison<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr<'a>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let expr = term(token_iter)?;

    match _comparison(token_iter) {
        Some(op) => {
            token_iter.next();
            Ok(Expr::EBinOp { op, left: Box::new(expr), right: Box::new(term(token_iter)?) })
        },
        None => Ok(expr),
    }
}


fn _term<'a, I>(token_iter: &mut PrevPeekable<I>) -> Option<Operator>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let token = token_iter.peek()?;
    match token.get_type() {
        Minus => Some(Operator::Sub),
        Plus => Some(Operator::Add),
        _ => None,
    }
}


fn term<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr<'a>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let expr = factor(token_iter)?;

    match _term(token_iter) {
        Some(op) => {
            token_iter.next();
            Ok(Expr::EBinOp { op, left: Box::new(expr), right: Box::new(factor(token_iter)?) })
        },
        None => Ok(expr),
    }
}


fn _factor<'a, I>(token_iter: &mut PrevPeekable<I>) -> Option<Operator>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let token = token_iter.peek()?;
    match token.get_type() {
        Slash => Some(Operator::Div),
        Star => Some(Operator::Mul),
        _ => None,
    }
}


fn factor<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr<'a>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let expr = unary(token_iter)?;

    match _factor(token_iter) {
        Some(op) => {
            token_iter.next();
            Ok(Expr::EBinOp { op, left: Box::new(expr), right: Box::new(unary(token_iter)?) })
        },
        None => Ok(expr),
    }
}


fn _unary<'a, I>(token_iter: &mut PrevPeekable<I>) -> Option<Operator>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let token = token_iter.peek()?;
    match token.get_type() {
        Bang => Some(Operator::Not),
        Minus => Some(Operator::Negate),
        _ => None,
    }
}


fn unary<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr<'a>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    match _unary(token_iter) {
        Some(op) => {
            token_iter.next();
            Ok(Expr::EUnaryOp { op, operand: Box::new(unary(token_iter)?) })
        },
        None => primary(token_iter),
    }
}


fn _primary<'b, I>(token_iter: &mut PrevPeekable<I>) -> Option<Expr<'b>>
where
    I: Iterator<Item = &'b Token<'b>>
{
    let token = token_iter.peek()?;
    match token.get_type() {
        False => Some(Expr::EBool { value: false }),
        True => Some(Expr::EBool { value: true }),
        Nil => Some(Expr::ENil),
        Number => match token.literal {
            Some(LiteralValue::LNumber(value)) => Some(Expr::ENumb { value }),
            _ => None,
        },
        Str => match token.literal {
            Some(LiteralValue::LString(value)) => Some(Expr::EStr { value }),
            _ => None,
        },
        Identifier => {
            Some(Expr::EVar { name: token.lexeme })
        },
        _ => None,
    }
}


fn primary<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr<'a>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    match _primary(token_iter) {
        Some(expr) => {
            token_iter.next();
            return Ok(expr);
        },
        None => group(token_iter),
    }
}


fn group<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr<'a>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
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
    expect(token_iter, RightParen)?;

    Ok(Expr::EGroup { expr: Box::new(expr) })
}


fn expect<'a, 'b, I>(
    token_iter: &mut PrevPeekable<I>,
    ttype: TokenType,
) -> Result<&'a Token<'a>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{

    let make_err = |t: Option<&Token>| -> Result<&'a Token<'a>, ParseError> {
        let tstr = match ttype.lexeme() {
            Some(v) => format!("'{}'", v),
            None => format!("{}", ttype),
        };

        Err(match t {
            Some(token) => ParseError::new(
                token.get_position(),
                format!("expected {} after expression", tstr),
            ),
            None => ParseError {
                pos: None,
                msg: format!("missing {} at EOF", tstr),
            },
        })
    };

    // We have to get next first so prev is the last token.
    // In other words we can't see current without making it prev.
    let next = token_iter.next();
    let prev = token_iter.prev_peek();
    match (prev, next) {
        (_, Some(token)) if *token.get_type() == ttype => Ok(token),
        (Some(token), None) => make_err(Some(token)),
        (_, Some(token)) => make_err(Some(token)),
        (None, None) => make_err(None),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::FilePosition;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_add() {
        let tokens = vec![
            Token::new_literal(
                Number,
                FilePosition::new(2, 26),
                "11.12",
                LiteralValue::LNumber(11.12),
            ),
            Token::nol(Plus, FilePosition::new(1, 9)),
            Token::new_literal(
                Number,
                FilePosition::new(2, 26),
                "12",
                LiteralValue::LNumber(12.0),
            ),
        ];

        let expr = parse_expr(&tokens).unwrap();

        assert_eq!(
            expr,
            Expr::EBinOp {
                op: Operator::Add,
                left: Box::new(Expr::ENumb { value: 11.12 }),
                right: Box::new(Expr::ENumb { value: 12.0 }),
            },
        );
    }

    #[test]
    fn test_precidence_mul_over_add_1() {
        let tokens = vec![
            Token::new_literal(
                Number,
                FilePosition::new(2, 26),
                "11.12",
                LiteralValue::LNumber(11.12),
            ),
            Token::nol(Plus, FilePosition::new(1, 9)),
            Token::new_literal(
                Number,
                FilePosition::new(2, 26),
                "12",
                LiteralValue::LNumber(12.0),
            ),
            Token::nol(Star, FilePosition::new(1, 9)),
            Token::new_literal(
                Number,
                FilePosition::new(2, 26),
                "3",
                LiteralValue::LNumber(3.0),
            ),
        ];

        let expr = parse_expr(&tokens).unwrap();

        assert_eq!(
            expr,
            Expr::EBinOp {
                op: Operator::Add,
                left: Box::new(Expr::ENumb { value: 11.12 }),
                right: Box::new(
                    Expr::EBinOp {
                        op: Operator::Mul,
                        left: Box::new(Expr::ENumb { value: 12.0 }),
                        right: Box::new(Expr::ENumb { value: 3.0 }),
                    },
                ),
            },
        );
    }

    #[test]
    fn test_precidence_mul_over_add_2() {
        let tokens = vec![
            Token::new_literal(
                Number,
                FilePosition::new(2, 26),
                "11.12",
                LiteralValue::LNumber(11.12),
            ),
            Token::nol(Star, FilePosition::new(1, 9)),
            Token::new_literal(
                Number,
                FilePosition::new(2, 26),
                "12",
                LiteralValue::LNumber(12.0),
            ),
            Token::nol(Plus, FilePosition::new(1, 9)),
            Token::new_literal(
                Number,
                FilePosition::new(2, 26),
                "3",
                LiteralValue::LNumber(3.0),
            ),
        ];

        let expr = parse_expr(&tokens).unwrap();

        assert_eq!(
            expr,
            Expr::EBinOp {
                op: Operator::Add,
                left: Box::new(
                    Expr::EBinOp {
                        op: Operator::Mul,
                        left: Box::new(Expr::ENumb { value: 11.12 }),
                        right: Box::new(Expr::ENumb { value: 12.0 }),
                    },
                ),
                right: Box::new(Expr::ENumb { value: 3.0 }),
            },
        );
    }

    #[test]
    fn test_grouping() {
        let tokens = vec![
            Token::new_literal(
                Number,
                FilePosition::new(2, 26),
                "11.12",
                LiteralValue::LNumber(11.12),
            ),
            Token::nol(Star, FilePosition::new(1, 9)),
            Token::nol(LeftParen, FilePosition::new(1, 9)),
            Token::new_literal(
                Number,
                FilePosition::new(2, 26),
                "12",
                LiteralValue::LNumber(12.0),
            ),
            Token::nol(Plus, FilePosition::new(1, 9)),
            Token::new_literal(
                Number,
                FilePosition::new(2, 26),
                "3",
                LiteralValue::LNumber(3.0),
            ),
            Token::nol(RightParen, FilePosition::new(1, 9)),
        ];

        let expr = parse_expr(&tokens).unwrap();

        assert_eq!(
            expr,
            Expr::EBinOp {
                op: Operator::Mul,
                left: Box::new(Expr::ENumb { value: 11.12 }),
                right: Box::new(
                    Expr::EGroup{
                        expr: Box::new(
                            Expr::EBinOp {
                                op: Operator::Add,
                                left: Box::new(Expr::ENumb { value: 12.0 }),
                                right: Box::new(Expr::ENumb { value: 3.0 }),
                            },
                        ),
                    },
                ),
            },
        );
    }
}
