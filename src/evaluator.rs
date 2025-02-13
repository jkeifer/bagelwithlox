use super::ast::{AST, Expr, Operator};
use super::environment::Environment;
use super::value::LoxValue;


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
    use Expr::*;
    use LoxValue::*;
    match expr {
        Numb { value } => Ok(VNumb(*value)),
        Str { value } => Ok(VStr(value.to_string())),
        Bool { value } => Ok(VBool(*value)),
        Nil => Ok(VNil),
        BinOp { op, left, right } => {
            eval_bin_op(
                &op,
                &eval(left.as_ref(), env)?,
                &eval(right.as_ref(), env)?,
                env,
            )
        },
        UnaryOp { op, operand } => {
            eval_unary_op(
                &op,
                &eval(operand.as_ref(), env)?,
            )
        },
        Group { expr } => eval(expr.as_ref(), env),
    }
}


pub fn evaluate(ast: AST, env: &mut Environment) -> Result<String, String> {
    match eval(&ast.top, env) {
        Ok(val) => Ok(format!("{}", val.value_string())),
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
        assert_eq!(run_expr("2"), VNumb(2.0));
        assert_eq!(run_expr("true"), VBool(true));
        assert_eq!(run_expr("false"), VBool(false));
        assert_eq!(run_expr("nil"), VNil);
        assert_eq!(run_expr("\"hello\""), VStr(String::from("hello")));
    }

    #[test]
    fn binops() {
        assert_eq!(run_expr("2+3"), VNumb(5.0));
        assert_eq!(run_expr("2*3"), VNumb(6.0));
        assert_eq!(run_expr("2-3"), VNumb(-1.0));
        assert_eq!(run_expr("3/2"), VNumb(1.5));
        assert_eq!(run_expr("\"hello\"+\"world\""), VStr(String::from("helloworld")));
    }

    #[test]
    fn compare() {
        assert_eq!(run_expr("2<3"), VBool(true));
        assert_eq!(run_expr("3<=3"), VBool(true));
        assert_eq!(run_expr("2>3"), VBool(false));
        assert_eq!(run_expr("3>=3"), VBool(true));
        assert_eq!(run_expr("3==3"), VBool(true));
        assert_eq!(run_expr("3!=3"), VBool(false));
        assert_eq!(run_expr("\"x\" == \"x\""), VBool(true));
    }

    #[test]
    fn group() {
        assert_eq!(run_expr("2 + (3*4)"), VNumb(14.0));
    }

    #[test]
    fn unary() {
        assert_eq!(run_expr("-3 + 4"), VNumb(1.0));
        assert_eq!(run_expr("!true"), VBool(false));
        assert_eq!(run_expr("!123"), VBool(false));
    }
}
