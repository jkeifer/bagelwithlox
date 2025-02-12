use std::fmt;
use super::ast::{AST, Expr, Operator};
use super::environment::Environment;


#[derive(Clone, Debug, PartialEq)]
enum LoxValue {
    Numb(f64),
    Str(String),
    Bool(bool),
    Nil,
}

impl fmt::Display for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LoxValue::*;
        write!(f, "{}", match self {
            Numb(_) => "Number",
            Str(_) => "String",
            Bool(_) => "Bool",
            Nil => "Nil",
        })
    }
}

impl LoxValue {
    fn _is_truthy(&self) -> bool {
        use LoxValue::*;
        match self {
            Numb(v) => *v != 0.0,
            Str(v) => v.len() > 0,
            Bool(v) => *v,
            Nil => false,
        }
    }

    fn is_truthy(&self) -> LoxValue {
        use LoxValue::*;
        match self {
            Bool(v) => Bool(*v),
            _ => Bool(self._is_truthy()),
        }
    }

    fn not(&self) -> Result<LoxValue, String> {
        use LoxValue::*;
        match self {
            Bool(v) => Ok(Bool(!v)),
            _ => self.is_truthy().not(),
        }
    }

    fn negate(&self) -> Result<LoxValue, String> {
        use LoxValue::*;
        match self {
            Numb(v) => Ok(Numb(-v)),
            _ => Err(format!("Cannot negate {}", self)),
        }
    }

    fn sub(&self, b: &LoxValue) -> Result<LoxValue, String> {
        use LoxValue::*;
        match (self, b) {
            (Numb(a), Numb(b)) => Ok(Numb(a - b)),
            (a, b) => Err(format!("Cannot subtract {} from {}", *a, b)),
        }
    }

    fn add(&self, b: &LoxValue) -> Result<LoxValue, String> {
        use LoxValue::*;
        match (self, b) {
            (Numb(a), Numb(b)) => Ok(Numb(a + b)),
            (Str(a), Str(b)) => Ok(Str(a.to_string() + &b)),
            (a, b) => Err(format!("Cannot add {} from {}", a, b)),
        }
    }

    fn mul(&self, b: &LoxValue) -> Result<LoxValue, String> {
        use LoxValue::*;
        match (self, b) {
            (Numb(a), Numb(b)) => Ok(Numb(a * b)),
            (Str(a), Numb(b)) => Ok(Str(a.repeat(*b as usize))),
            (Numb(a), Str(b)) => Ok(Str(b.repeat(*a as usize))),
            (a, b) => Err(format!("Cannot multiply {} from {}", a, b)),
        }
    }

    fn div(&self, b: &LoxValue) -> Result<LoxValue, String> {
        use LoxValue::*;
        match (self, b) {
            (Numb(a), Numb(b)) => Ok(Numb(a / b)),
            (a, b) => Err(format!("Cannot divide {} from {}", a, b)),
        }
    }

    fn neq(&self, b: &LoxValue) -> Result<LoxValue, String> {
        use LoxValue::*;
        match (self, b) {
            (Numb(a), Numb(b)) => Ok(Bool(a != b)),
            (Str(a), Str(b)) => Ok(Bool(a != b)),
            (Bool(a), Bool(b)) => Ok(Bool(a != b)),
            _ => Ok(Bool(true)),
        }
    }

    fn eq(&self, b: &LoxValue) -> Result<LoxValue, String> {
        use LoxValue::*;
        match (self, b) {
            (Numb(a), Numb(b)) => Ok(Bool(a == b)),
            (Str(a), Str(b)) => Ok(Bool(a == b)),
            (Bool(a), Bool(b)) => Ok(Bool(a == b)),
            _ => Ok(Bool(false)),
        }
    }

    fn gt(&self, b: &LoxValue) -> Result<LoxValue, String> {
        use LoxValue::*;
        match (self, b) {
            (Numb(a), Numb(b)) => Ok(Bool(a > b)),
            (Str(a), Str(b)) => Ok(Bool(a > b)),
            (Bool(a), Bool(b)) => Ok(Bool(a > b)),
            _ => Ok(Bool(false)),
        }
    }

    fn ge(&self, b: &LoxValue) -> Result<LoxValue, String> {
        use LoxValue::*;
        match (self, b) {
            (Numb(a), Numb(b)) => Ok(Bool(a >= b)),
            (Str(a), Str(b)) => Ok(Bool(a >= b)),
            (Bool(a), Bool(b)) => Ok(Bool(a >= b)),
            _ => Ok(Bool(false)),
        }
    }

    fn lt(&self, b: &LoxValue) -> Result<LoxValue, String> {
        use LoxValue::*;
        match (self, b) {
            (Numb(a), Numb(b)) => Ok(Bool(a < b)),
            (Str(a), Str(b)) => Ok(Bool(a < b)),
            (Bool(a), Bool(b)) => Ok(Bool(a < b)),
            _ => Ok(Bool(false)),
        }
    }

    fn le(&self, b: &LoxValue) -> Result<LoxValue, String> {
        use LoxValue::*;
        match (self, b) {
            (Numb(a), Numb(b)) => Ok(Bool(a <= b)),
            (Str(a), Str(b)) => Ok(Bool(a <= b)),
            (Bool(a), Bool(b)) => Ok(Bool(a <= b)),
            _ => Ok(Bool(false)),
        }
    }

    fn and(&self, b: &LoxValue) -> Result<LoxValue, String> {
        use LoxValue::*;
        Ok(Bool(match self {
            Bool(v) => *v,
            _ => self._is_truthy(),
        } && match self {
            Bool(v) => *v,
            _ => b._is_truthy(),
        }))
    }

    fn or(&self, b: &LoxValue) -> Result<LoxValue, String> {
        use LoxValue::*;
        Ok(Bool(match self {
            Bool(v) => *v,
            _ => self._is_truthy(),
        } || match self {
            Bool(v) => *v,
            _ => b._is_truthy(),
        }))
    }
}


fn eval_bin_op(
    op: &Operator,
    left: &LoxValue,
    right: &LoxValue,
    env: &mut Environment,
) -> Result<LoxValue, String> {
    use Operator::*;
    match op {
        Sub => left.sub(right),
        Add => left.add(right),
        Mul => left.mul(right),
        Div => left.div(right),
        Assign => {
            // TODO: env insert
            //env.insert(left, right);
            Ok(right.clone())
        },
        NotEqual => left.neq(right),
        Equal => left.eq( right),
        Greater => left.gt(right),
        GreaterEqual => left.ge(right),
        Less => left.lt(right),
        LessEqual => left.le(right),
        And => left.and(right),
        Or => left.or(right),
        _ => Err(format!("Unsupported binary operation: {}", op)),
    }
}


fn eval_unary_op(
    op: &Operator,
    operand: &LoxValue,
) -> Result<LoxValue, String> {
    use Operator::*;
    match op {
        Not => operand.not(),
        Negate => operand.negate(),
        _ => Err(format!("Unsupported unary operation: {}", op)),
    }
}


fn eval(expr: &Expr,  env: &mut Environment) -> Result<LoxValue, String> {
    match expr {
        Expr::Numb { value } => Ok(LoxValue::Numb(*value)),
        Expr::Str { value } => Ok(LoxValue::Str(value.to_string())),
        Expr::Bool { value } => Ok(LoxValue::Bool(*value)),
        Expr::Nil => Ok(LoxValue::Nil),
        Expr::BinOp { op, left, right } => {
            eval_bin_op(
                &op,
                &eval(left.as_ref(), env)?,
                &eval(right.as_ref(), env)?,
                env,
            )
        },
        Expr::UnaryOp { op, operand } => {
            eval_unary_op(
                &op,
                &eval(operand.as_ref(), env)?,
            )
        },
        Expr::Group { expr } => eval(expr.as_ref(), env),
    }
}


pub fn evaluate(ast: AST, env: &mut Environment) -> Result<String, String> {
    match eval(&ast.top, env) {
        Ok(val) => Ok(format!("{}", val)),
        Err(e) => Err(e),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use LoxValue::*;

    fn run_expr(text : &str) -> LoxValue {
        let mut env = Environment::new();
        let src = crate::source::Source::from_string(
            text.to_string(),
        );
        let tokens = crate::tokenizer::tokenize(&src).unwrap();
        let ast = crate::parser::parse(&tokens).unwrap();
        eval(&ast.top, &mut env).unwrap()
    }

    #[test]
    fn literals() {
        assert_eq!(run_expr("2"), Numb(2.0));
        assert_eq!(run_expr("true"), Bool(true));
        assert_eq!(run_expr("false"), Bool(false));
        assert_eq!(run_expr("nil"), Nil);
        assert_eq!(run_expr("\"hello\""), Str(String::from("hello")));
    }

    #[test]
    fn binops() {
        assert_eq!(run_expr("2+3"), Numb(5.0));
        assert_eq!(run_expr("2*3"), Numb(6.0));
        assert_eq!(run_expr("2-3"), Numb(-1.0));
        assert_eq!(run_expr("3/2"), Numb(1.5));
        assert_eq!(run_expr("\"hello\"+\"world\""), Str(String::from("helloworld")));
    }

    #[test]
    fn compare() {
        assert_eq!(run_expr("2<3"), Bool(true));
        assert_eq!(run_expr("3<=3"), Bool(true));
        assert_eq!(run_expr("2>3"), Bool(false));
        assert_eq!(run_expr("3>=3"), Bool(true));
        assert_eq!(run_expr("3==3"), Bool(true));
        assert_eq!(run_expr("3!=3"), Bool(false));
        assert_eq!(run_expr("\"x\" == \"x\""), Bool(true));
    }

    #[test]
    fn group() {
        assert_eq!(run_expr("2 + (3*4)"), Numb(14.0));
    }

    #[test]
    fn unary() {
        assert_eq!(run_expr("-3 + 4"), Numb(1.0));
        assert_eq!(run_expr("!true"), Bool(false));
        assert_eq!(run_expr("!123"), Bool(false));
    }
}
