use std::rc::Rc;

use crate::ast::Interpretables;

use super::ast::{Expr, Stmt, Interpretable, Operator};
use super::environment::Environment;
use super::value::{LoxValue, LoxType};


fn eval_bin_op(
    op: &Operator,
    left: &LoxValue,
    right: &LoxValue,
) -> Result<LoxValue, String> {
    use Operator::*;
    match op {
        Sub => left.sub(&right),
        Add => left.add(&right),
        Mul => left.mul(&right),
        Div => left.div(&right),
        NotEqual => left.neq(&right),
        Equal => left.eq(&right),
        Greater => left.gt(&right),
        GreaterEqual => left.ge(&right),
        Less => left.lt(&right),
        LessEqual => left.le(&right),
        _ => Err(format!("Unsupported binary operation: {}", op)),
    }
}


fn eval_logical_op(
    op: &Operator,
    left: &LoxValue,
    right: &LoxValue,
) -> Result<LoxValue, String> {
    use Operator::*;
    match op {
        And => left.and(&right),
        Or => left.or(&right),
        _ => Err(format!("Unsupported logical operation: {}", op)),
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


pub fn eval(expr: &Expr, env: &Rc<Environment>) -> Result<LoxValue, String> {
    use Expr::*;
    use LoxType::*;
    match expr {
        ENumb { value } => Ok(LoxValue::new(VNumb(*value))),
        EStr { value } => Ok(LoxValue::new(VStr(value.to_string()))),
        EBool { value } => Ok(LoxValue::new(VBool(*value))),
        ENil => Ok(LoxValue::new(VNil)),
        EBinOp { op, left, right } => {
            Ok(eval_bin_op(
                &op,
                &eval(left.as_ref(), env)?,
                &eval(right.as_ref(), env)?,
            )?,)
        },
        EUnaryOp { op, operand } => {
            Ok(eval_unary_op(
                &op,
                &eval(operand.as_ref(), env)?,
            )?,)
        },
        EGroup { expr } => eval(expr.as_ref(), env),
        EVar { name } => env.lookup( name ),
        EAssign { name, expr } => env.assign(
            name,
            eval(expr.as_ref(), env)?,
        ),
        ELogicalOp { op, left, right } => {
            Ok(eval_logical_op(
                &op,
                &eval(left.as_ref(), env)?,
                &eval(right.as_ref(), env)?,
            )?,)
        },
        ECall{ func, args } => {
            let func = eval(func.as_ref(), env)?;

            let VCallable(_, params, body, _env) = &*func else {
                return Err(format!("{:?} not a function", func));
            };

            if args.len() != params.len() {
                return Err(format!("Function {:?} requires {} argument(s)", func, params.len()));
            }

            let mut arg_vals = Vec::new();
            for arg in args.iter() {
                arg_vals.push(eval(arg, env)?);
            }

            let func_env = Environment::new_child(&_env);
            for (parm, arg) in params.iter().zip(arg_vals) {
                func_env.var(&parm, Some(arg));
            }

            match exec(&body, &func_env)? {
                Some(v) => Ok(v),
                None => Ok(LoxValue::new(VNil)),
            }
        },
    }
}


fn _add_option<T, E>(x: Result<T, E>) -> Result<Option<T>, E> {
    match x {
        Ok(v) => Ok(Some(v)),
        Err(e) => Err(e),
    }
}


pub fn exec(stmt: &Stmt, env: &Rc<Environment>) -> Result<Option<LoxValue>, String> {
    use Stmt::*;
    match stmt {
        SPrint(expr) => {
            println!("{}", eval(expr, env)?.value_string());
        },
        SExpr(expr) => {
            eval(expr, env)?;
        },
        SVar(name, value) => {
            let value = match value {
                Some(v) => Some(eval(v, env)?),
                None => None,

            };
            env.var(name, value);
        },
        SBlock(stmts) => {
            let env = Environment::new_child(env);
            for stmt in stmts{
                match exec(stmt, &env)? {
                    Some(v) => return Ok(Some(v)),
                    None => (),
                }
            }
        },
        SIf(cond, then, else_) => {
            if eval(cond, &env)?._is_truthy() {
                return exec(then, &env);
            }

            if let Some(else_) = else_ {
                return exec(else_, &env);
            }
        },
        SWhile(cond, body) => {
            while eval(cond, &env)?._is_truthy() {
                match exec(body, &env)? {
                    Some(v) => return Ok(Some(v)),
                    None => (),
                }
            }
        },
        SFun(name, params, body) => {
            let func = LoxValue::new(LoxType::VCallable(
                name.clone(),
                params.clone(),
                *body.clone(),
                Environment::new_child(&env),
            ));
            env.var(name, Some(func));
        },
        SReturn(expr) => return _add_option(eval(expr, env)),
        SEmpty => (),
    }
    Ok(None)
}


pub fn interpret(
    interpretables: &Interpretables,
    env: &Rc<Environment>,
) -> Result<Option<LoxValue>, String> {
    for interpretable in &**interpretables {
        match interpretable {
            Interpretable::IStmt(stmt) => match exec(&stmt, &env)? {
                Some(v) => return Ok(Some(v)),
                None => (),
            },
            Interpretable::IExpr(expr) => return _add_option(eval(expr, &env)),
        }
    };
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use LoxType::*;

    fn run_expr(text : &str) -> LoxValue {
        let env = Environment::new();
        let src = crate::source::Source::from_string(
            text.to_string(),
        );
        let tokens = crate::tokenizer::tokenize(&src).unwrap();
        let expr = crate::parser::parse_expr(&tokens).unwrap();
        (eval(&expr, &env).unwrap()).clone()
    }

    #[test]
    fn literals() {
        assert_eq!(*run_expr("2"), VNumb(2.0));
        assert_eq!(*run_expr("true"), VBool(true));
        assert_eq!(*run_expr("false"), VBool(false));
        assert_eq!(*run_expr("nil"), VNil);
        assert_eq!(*run_expr("\"hello\""), VStr(String::from("hello")));
    }

    #[test]
    fn binops() {
        assert_eq!(*run_expr("2+3"), VNumb(5.0));
        assert_eq!(*run_expr("2*3"), VNumb(6.0));
        assert_eq!(*run_expr("2-3"), VNumb(-1.0));
        assert_eq!(*run_expr("3/2"), VNumb(1.5));
        assert_eq!(*run_expr("\"hello\"+\"world\""), VStr(String::from("helloworld")));
    }

    #[test]
    fn compare() {
        assert_eq!(*run_expr("2<3"), VBool(true));
        assert_eq!(*run_expr("3<=3"), VBool(true));
        assert_eq!(*run_expr("2>3"), VBool(false));
        assert_eq!(*run_expr("3>=3"), VBool(true));
        assert_eq!(*run_expr("3==3"), VBool(true));
        assert_eq!(*run_expr("3!=3"), VBool(false));
        assert_eq!(*run_expr("\"x\" == \"x\""), VBool(true));
    }

    #[test]
    fn group() {
        assert_eq!(*run_expr("2 + (3*4)"), VNumb(14.0));
    }

    #[test]
    fn unary() {
        assert_eq!(*run_expr("-3 + 4"), VNumb(1.0));
        assert_eq!(*run_expr("!true"), VBool(false));
        assert_eq!(*run_expr("!123"), VBool(false));
    }
}
