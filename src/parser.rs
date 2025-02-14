use prev_iter::PrevPeekable;
use crate::ast::Stmts;
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


pub fn parse<'a>(tokens: &'a Tokens<'a>) -> Result<AST, ParseError> {
    let mut ast = AST::new();
    let mut token_iter = PrevPeekable::new(tokens.iter());

    while let Some(result) = declaration(&mut token_iter) {
        ast.top.push(result?);
    }

    Ok(ast)
}


pub fn parse_expr<'a>(tokens: &'a Tokens<'a>) -> Result<Expr, ParseError> {
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


fn declaration<'a, I>(token_iter: &mut PrevPeekable<I>) -> Option<Result<Stmt, ParseError>>
where
    I: Iterator<Item = &'a Token<'a>>
{
    match token_iter.peek()?.get_type() {
        Var => Some(var_declaration(token_iter)),
        _ => statement(token_iter),
    }
}


fn var_declaration<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Stmt, ParseError>
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
    Ok(Stmt::SVar{ name: id.lexeme.to_string(), value: init })
}


fn statement<'a, I>(token_iter: &mut PrevPeekable<I>) -> Option<Result<Stmt, ParseError>>
where
    I: Iterator<Item = &'a Token<'a>>
{
    Some(match token_iter.peek()?.get_type() {
        Print => print_statement(token_iter),
        Equal => assignment_statement(token_iter),
        _ => expression_statement(token_iter),
    })
}


fn _for_initializer<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Option<Stmt>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let token = match token_iter.peek() {
        Some(token) => token,
        None => {
            token_iter.next();
            let last = token_iter.prev().unwrap();
            return Err(ParseError::new(
                last.get_position(),
                "incomplete for statement".to_string(),
            ));
        },
    };

    match token.get_type() {
        SemiColon => {
            token_iter.next();
            return Ok(None)
        },
        Var => Ok(Some(var_declaration(token_iter)?)),
        _ => Ok(Some(expression_statement(token_iter)?)),
    }
}


fn _for_condition<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let token = match token_iter.peek() {
        Some(token) => token,
        None => {
            token_iter.next();
            let last = token_iter.prev().unwrap();
            return Err(ParseError::new(
                last.get_position(),
                "incomplete for statement".to_string(),
            ));
        },
    };

    let result = match token.get_type() {
        SemiColon => {
            token_iter.next();
            return Ok(Expr::EBool { value: true })
        },
        _ => Ok(expression(token_iter)?),
    };

    dbg!(1);
    expect(token_iter, SemiColon)?;

    result
}


fn _for_increment<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Option<Expr>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let token = match token_iter.peek() {
        Some(token) => token,
        None => {
            token_iter.next();
            let last = token_iter.prev().unwrap();
            return Err(ParseError::new(
                last.get_position(),
                "incomplete for statement".to_string(),
            ));
        },
    };

    match token.get_type() {
        RightParen => {
            token_iter.next();
            return Ok(None)
        },
        _ => Ok(Some(expression(token_iter)?)),
    }
}


fn for_expression<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    token_iter.next();
    expect(token_iter, LeftParen)?;
    let init = _for_initializer(token_iter)?;
    let cond = _for_condition(token_iter)?;
    let incr = _for_increment(token_iter)?;
    expect(token_iter, RightParen)?;
    let mut body = block(token_iter)?;

    if let Some(expr) = incr {
        body = Expr::EBlock(vec![Stmt::SExprStmt(body), Stmt::SExprStmt(expr)]);
    }

    body = Expr::EWhile(Box::new(cond), Box::new(body));

    if let Some(stmt) = init {
        body = Expr::EBlock(vec![stmt, Stmt::SExprStmt(body)]);
    }


    Ok(body)
}


fn else_expression<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    token_iter.next(); // should already have matched else

    // need to make sure we have a next token or it's an error
    let token = match token_iter.peek() {
        Some(token) => token,
        None => {
            let last = token_iter.prev().unwrap();
            return Err(ParseError::new(
                last.get_position(),
                "expected if or code block after else".to_string(),
            ));
        },
    };

    // next should be if or block or it's an error
    match token.get_type() {
        If => if_expression(token_iter),
        LeftBrace => block(token_iter),
        other => {
            dbg!("boo");
            return Err(ParseError::new(
            token.pos,
            format!("expected if or code block after else, found {}", other),
        ));
        },
    }
}


fn if_expression<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    token_iter.next();
    let cond = expression(token_iter)?;
    let then = block(token_iter)?;

    let else_ = match token_iter.peek() {
        Some(token) =>  match token.get_type() {
            Else => Some(Box::new(else_expression(token_iter)?)),
            _ => None,
        },
        None => None,
    };

    Ok(Expr::EIf(Box::new(cond), Box::new(then), else_))
}


fn print_statement<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Stmt, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    token_iter.next();
    let expr = expression(token_iter)?;
    dbg!(2);
    expect(token_iter, SemiColon)?;
    Ok(Stmt::SPrint(expr))
}


fn while_expression<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    token_iter.next();
    let cond = expression(token_iter)?;
    let body = block(token_iter)?;

    Ok(Expr::EWhile(Box::new(cond), Box::new(body)))
}


fn _next_is<'a, I>(token_iter: &mut PrevPeekable<I>, ttype: TokenType) -> bool
where
    I: Iterator<Item = &'a Token<'a>>
{
    match token_iter.peek() {
        Some(token) => return *token.get_type() == ttype,
        None => false,
    }
}


fn block<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    expect(token_iter, LeftBrace)?;
    let mut stmts = Stmts::new();

    while !_next_is(token_iter, RightBrace) {
        match declaration(token_iter) {
            Some(result) => stmts.push(result?),
            None => break,
        };
    }

    expect(token_iter, RightBrace)?;
    Ok(Expr::EBlock(stmts))
}


fn expression_statement<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Stmt, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let expr = expression(token_iter)?;

    // consume a semi colon
    if _next_is(token_iter, SemiColon) {
        token_iter.next();
    }
    //expect(token_iter, SemiColon)?;
    Ok(Stmt::SExprStmt(expr))
}


fn expression<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    // need to make sure we have a next token or it's an error
    let token = match token_iter.peek() {
        Some(token) => token,
        None => return Ok(Expr::ENil),
    };

    match token.get_type() {
        For => for_expression(token_iter),
        If => if_expression(token_iter),
        While => while_expression(token_iter),
        LeftBrace => block(token_iter),
        _ => assignment(token_iter),
    }
}


fn assignment_statement<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Stmt, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    token_iter.next();
    let stmt = Stmt::SExprStmt(assignment(token_iter)?);
    expect(token_iter, SemiColon)?;
    Ok(stmt)
}


fn assignment<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let expr = or(token_iter)?;

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


fn _is_or<'a, I>(token_iter: &mut PrevPeekable<I>) -> bool
where
    I: Iterator<Item = &'a Token<'a>>
{
    match token_iter.peek() {
        Some(token) => *token.get_type() == Or,
        None => false,
    }
}


fn or<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let mut expr = and(token_iter)?;

    while _is_or(token_iter) {
        token_iter.next();
        expr = Expr::ELogicalOp {
            op: Operator::Or,
            left: Box::new(expr),
            right: Box::new(and(token_iter)?)
        };
    }

    Ok(expr)
}


fn _is_and<'a, I>(token_iter: &mut PrevPeekable<I>) -> bool
where
    I: Iterator<Item = &'a Token<'a>>
{
    match token_iter.peek() {
        Some(token) => *token.get_type() == And,
        None => false,
    }
}


fn and<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let mut expr = equality(token_iter)?;

    while _is_and(token_iter) {
        token_iter.next();
        expr = Expr::ELogicalOp {
            op: Operator::And,
            left: Box::new(expr),
            right: Box::new(equality(token_iter)?)
        };
    }

    Ok(expr)
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


fn equality<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr, ParseError>
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


fn comparison<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr, ParseError>
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


fn term<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr, ParseError>
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


fn factor<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr, ParseError>
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


fn unary<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    match _unary(token_iter) {
        Some(op) => {
            token_iter.next();
            Ok(Expr::EUnaryOp { op, operand: Box::new(unary(token_iter)?) })
        },
        None => call(token_iter),
    }
}


fn _function_args<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Vec<Expr>, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let mut args = Vec::new();

    while !_next_is(token_iter, RightParen) {
        args.push(expression(token_iter)?);
        expect(token_iter, Comma)?;
    }

    // TODO: handle error if too many args?
    // if args.len() > 255 {

    Ok(args)
}


fn call<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr, ParseError>
where
    I: Iterator<Item = &'a Token<'a>>
{
    let mut expr = primary(token_iter)?;

    loop {
        if _next_is(token_iter, LeftParen) {
            token_iter.next(); // because we know we have left paren
            let args = _function_args(token_iter)?;
            expect(token_iter, RightParen)?;
            expr = Expr::ECall { func: Box::new(expr), args };
        } else {
            break
        }
    }

    Ok(expr)
}


fn _primary<'b, I>(token_iter: &mut PrevPeekable<I>) -> Option<Expr>
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
            Some(LiteralValue::LString(value)) => Some(Expr::EStr { value: value.to_string() }),
            _ => None,
        },
        Identifier => {
            Some(Expr::EVar { name: token.lexeme.to_string() })
        },
        _ => None,
    }
}


fn primary<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr, ParseError>
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


fn group<'a, I>(token_iter: &mut PrevPeekable<I>) -> Result<Expr, ParseError>
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


// TODO: this erorr message approach falls apart with statements...
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
