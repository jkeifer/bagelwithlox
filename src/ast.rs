use std::fmt;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Sub,
    Add,
    Mul,
    Div,
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


#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    ENumb{ value: f64 },
    EStr{ value: &'a str },
    EBool{ value: bool },
    ENil,
    EBinOp{ op: Operator, left: Box<Expr<'a>>, right: Box<Expr<'a>> },
    EUnaryOp{ op: Operator, operand: Box<Expr<'a>> },
    EGroup{ expr: Box<Expr<'a>> },
    EVar{ name: &'a str },
    EAssign{ name: &'a str, expr: Box<Expr<'a>>},
}

impl<'a> fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expr::*;
        write!(f, "{}", match self {
            ENumb{ value } => format!("{}", value),
            EStr{ value } => format!("\"{}\"", value),
            EBool{ value } => format!("{}", value),
            ENil => String::from("nil"),
            EBinOp{ op, left, right} => format!(
                "({} {} {})",
                left,
                op,
                right,
            ),
            EUnaryOp{ op, operand} => format!(
                "{}{}",
                op,
                operand,
            ),
            EGroup{ expr } => format!("({})", expr),
            EVar{ name } => format!("var {}", name),
            EAssign{ name, expr } => format!(
                "{} = {}",
                name,
                expr,
            ),
        })
    }
}


#[derive(Debug, PartialEq)]
pub enum Stmt<'a> {
    SPrint(Expr<'a>),
    SVar{ name: &'a str, value: Option<Expr<'a>> },
    SExprStmt(Expr<'a>),
    SBlock(Stmts<'a>),
}

pub type Stmts<'a> = Vec<Stmt<'a>>;


#[derive(Debug, PartialEq)]
pub struct AST<'a> {
    pub top : Stmts<'a>
}

impl<'a> AST<'a> {
    pub fn new() -> AST<'a> {
        AST { top: vec![] }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_() {
        use Expr::*;
        use Operator::*;
        let e = EBinOp{
            op: Mul,
            left: Box::new(EUnaryOp{
                op: Negate,
                operand: Box::new(ENumb { value: 123.0 }),
            }),
            right: Box::new(EGroup{
                expr: Box::new(ENumb{
                    value: 45.67,
                }),
            }),
        };
        assert_eq!(format!("{}", e), "(-123 * (45.67))");
    }
}
