use std::{fmt, ops::{Deref, DerefMut}};


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

    pub fn is_logical_operator(&self) -> bool {
        use Operator::*;
        match self {
            And
            | Or
                => true,
            _ => false,
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    ENumb{ value: f64 },
    EStr{ value: String },
    EBool{ value: bool },
    ENil,
    EBinOp{ op: Operator, left: Box<Expr>, right: Box<Expr> },
    EUnaryOp{ op: Operator, operand: Box<Expr> },
    EGroup{ expr: Box<Expr> },
    EVar{ name: String },
    EAssign{ name: String, expr: Box<Expr>},
    ELogicalOp{ op: Operator, left: Box<Expr>, right: Box<Expr> },
    ECall{ func: Box<Expr>, args: Vec<Expr> },
}

impl fmt::Display for Expr {
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
            ELogicalOp{ op, left, right} => format!(
                "({} {} {})",
                left,
                op,
                right,
            ),
            ECall{ func, args } => format!(
                "{}({:?})",
                func,
                args,
            ),
        })
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    SPrint(Expr),
    SVar(String, Option<Expr>),
    SExpr(Expr),
    SFun(String, Vec<String>, Box<Stmt>),
    SReturn(Expr),
    SBlock(Vec<Stmt>),
    SIf(Expr, Box<Stmt>, Option<Box<Stmt>>),
    SWhile(Expr, Box<Stmt>),
    SEmpty,
}


#[derive(Clone, Debug, PartialEq)]
pub enum Interpretable {
    IStmt(Stmt),
    IExpr(Expr),
}


#[derive(Clone, Debug, PartialEq)]
pub struct Interpretables(Vec<Interpretable>);

impl Deref for Interpretables {
    type Target = Vec<Interpretable>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Interpretables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Interpretables {
    pub fn new() -> Interpretables {
        Interpretables(Vec::new())
    }
}


#[derive(Debug, PartialEq)]
pub struct AST {
    pub top: Interpretables
}

impl AST {
    pub fn new() -> AST {
        AST { top: Interpretables::new() }
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
