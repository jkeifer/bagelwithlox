use std::rc::Rc;
use super::ast::{Expr, Operator, Stmt};
use super::environment::Environment;
use super::value::LoxValue;


fn eval_bin_op(
    op: &Operator,
    left: &LoxValue,
    right: &LoxValue,
) -> Result<LoxValue, String> {
    use Operator::*;
    match op {
        Sub => left.sub(right),
        Add => left.add(right),
        Mul => left.mul(right),
        Div => left.div(right),
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


pub fn eval(expr: &Expr, env: &Environment) -> Result<Rc<LoxValue>, String> {
    use Expr::*;
    use LoxValue::*;
    match expr {
        ENumb { value } => Ok(Rc::new(VNumb(*value))),
        EStr { value } => Ok(Rc::new(VStr(value.to_string()))),
        EBool { value } => Ok(Rc::new(VBool(*value))),
        ENil => Ok(Rc::new(VNil)),
        EBinOp { op, left, right } => {
            Ok(Rc::new(
                eval_bin_op(
                    &op,
                    &*eval(left.as_ref(), env)?,
                    &*eval(right.as_ref(), env)?,
                )?,
            ))
        },
        EUnaryOp { op, operand } => {
            Ok(Rc::new(
                eval_unary_op(
                    &op,
                    &*eval(operand.as_ref(), env)?,
                )?,
            ))
        },
        EGroup { expr } => eval(expr.as_ref(), env),
        EVar { name } => env.lookup( name ),
        EAssign { name, expr } => env.assign(
            name,
            eval(expr.as_ref(), env)?,
        ),
    }
}


pub fn exec(stmt: &Stmt, env: &Environment) -> Result<Rc<LoxValue>, String> {
    use Stmt::*;

    match stmt {
        SPrint(expr) => {
            println!("{}", eval(expr, env)?.value_string());
            Ok(Rc::new(LoxValue::VNil))
        },
        SExprStmt(expr) => eval(expr, env),
        SVar{ name, value } => Ok(env.var(
            name,
            match value {
                Some(v) => eval(v, env)?,
                None => Rc::new(LoxValue::VUninitialized),
            },
        )),
        SBlock(statments) => {
            let env = env.new_child();
            match statments.split_last() {
                Some(( last_statement, other_statements)) => {
                    for stmt in other_statements{
                        exec(stmt, &env)?;
                    }
                    exec(last_statement, &env)
                },
                None => Ok(Rc::new(LoxValue::VNil)),
            }
        },
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use LoxValue::*;

    fn run_expr(text : &str) -> LoxValue {
        let mut env = Environment::new(None);
        let src = crate::source::Source::from_string(
            text.to_string(),
        );
        let tokens = crate::tokenizer::tokenize(&src).unwrap();
        let expr = crate::parser::parse_expr(&tokens).unwrap();
        (*eval(&expr, &mut env).unwrap()).clone()
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
