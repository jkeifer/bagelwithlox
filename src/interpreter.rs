use crate::ast::Stmt;
use crate::parser::parse_expr;
use crate::tokenizer::Tokens;

use super::source::Source;
use super::environment::Environment;
use super::parser::parse;
use super::evaluator::eval;
use super::tokenizer::tokenize;

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter{
            env: Environment::new(),
        }
    }

    pub fn interpret(&mut self, src: &mut Source) -> Result<Option<String>, String> {
        let tokens = match tokenize(src) {
            Ok(v) => v,
            Err(e) => {
                return Err(src.format_error(&e));
            },
        };

        // TODO: only do this in repl
        if let Ok(v) = self.interpret_expression(src, &tokens) {
            return Ok(Some(v));
        }

        match self.interpret_statement(src, &tokens) {
            Ok(_) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn interpret_expression(&mut self, src: &Source, tokens: &Tokens) -> Result<String, String> {
        let expr = match parse_expr(&tokens) {
            Ok(v) => v,
            Err(e) => {
                return Err(src.format_error(&e));
            },
        };

        Ok(eval(&expr, &mut self.env)?.value_string())
    }

    fn interpret_statement(&mut self, src: &Source, tokens: &Tokens) -> Result<(), String> {
        let ast = match parse(&tokens) {
            Ok(v) => v,
            Err(e) => {
                return Err(src.format_error(&e));
            },
        };

        for statement in ast.top {
            self.execute(&statement)?;
        }

        Ok(())
    }

    fn execute<'a>(&mut self, stmt: &Stmt<'a>) -> Result<(), String> {
        use Stmt::*;

        match stmt {
            SPrint(expr) => {
                println!("{}", eval(expr, &mut self.env)?.value_string());
                Ok(())
            },
            SVar{ name, value } => Ok(()),
            SExprStmt(expr) => {
                eval(expr, &mut self.env)?;
                Ok(())
            },
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpret() {
        Interpreter::new().interpret(
            &mut Source::from_string("string".to_string()),
        );
    }
}
