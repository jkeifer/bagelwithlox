use std::fmt;


pub enum Operator {
    Sub,
    Add,
    Mul,
    Div,
    Assign,
    NotEqual,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
    Not,
    Negate,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Operator::*;
        write!(f, "{}", match self {
            Sub | Negate => "-",
            Add => "+",
            Mul => "*",
            Div => "/",
            Assign => "=",
            NotEqual => "!=",
            Equal => "=",
            Greater => ">",
            GreaterEqual => ">=",
            Less => "<",
            LessEqual => "<=",
            And => "and",
            Or => "or",
            Not => "!",
        })
    }
}

impl Operator {
    pub fn is_binary_operator(&self) -> bool {
        use Operator::*;
        match self {
            Sub
            | Add
            | Mul
            | Div
            | Assign
            | NotEqual
            | Equal
            | Greater
            | GreaterEqual
            | Less
            | LessEqual
            | And
            | Or
                => true,
            _ => false,
        }
    }

    pub fn is_unary_operator(&self) -> bool {
        use Operator::*;
        match self {
            Not | Negate => true,
            _ => false,
        }
    }
}


pub enum Expr<'a> {
    Numb{ value: f64 },
    Str{ value: &'a str },
    Bool{ value: bool },
    Nil,
    BinOp{ op: Operator, left: Box<Expr<'a>>, right: Box<Expr<'a>> },
    UnaryOp{ op: Operator, operand: Box<Expr<'a>> },
    Group{ expr: Box<Expr<'a>> },
}

impl<'a> fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expr::*;
        write!(f, "{}", match self {
            Numb{ value } => format!("{}", value),
            Str{ value } => format!("\"{}\"", value),
            Bool{ value } => format!("{}", value),
            Nil => String::from("nil"),
            BinOp{ op, left, right} => format!(
                "({} {} {})",
                left,
                op,
                right,
            ),
            UnaryOp{ op, operand} => format!(
                "{}{}",
                op,
                operand,
            ),
            Group{ expr } => format!("({})", expr),
        })
    }
}

pub type AST<'a> = Expr<'a>;

//        match token.token_type {
//            TokenType::Minus
//            TokenType::Plus
//            TokenType::Star
//            TokenType::Slash
//            TokenType::BangEqual
//            TokenType::Equal
//            TokenType::EqualEqual
//            TokenType::Greater
//            TokenType::GreaterEqual
//            TokenType::Less
//            TokenType::LessEqual
//            TokenType::And
//            TokenType::Or
//            TokenType::Bang

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_() {
        use Expr::*;
        use Operator::*;
        let e = BinOp{
            op: Mul,
            left: Box::new(UnaryOp{
                op: Negate,
                operand: Box::new(Numb { value: 123.0 }),
            }),
            right: Box::new(Group{
                expr: Box::new(Numb{
                    value: 45.67,
                }),
            }),
        };
        assert_eq!(format!("{}", e), "(-123 * (45.67))");
    }
}
