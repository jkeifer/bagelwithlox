use std::rc::Rc;

use crate::parser::parse_expr;
use crate::tokenizer::Tokens;
use crate::value::LoxValue;

use super::source::Source;
use super::environment::Environment;
use super::parser::parse;
use super::evaluator::{eval, exec};
use super::tokenizer::tokenize;

pub struct Interpreter<'a> {
    env: Environment<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Interpreter<'a> {
        Interpreter{
            env: Environment::new(None),
        }
    }

    pub fn interpret<'b>(&mut self, src: &'b mut Source) -> Result<Option<String>, String> {
        let tokens = match tokenize(src) {
            Ok(v) => v,
            Err(e) => {
                return Err(src.format_error(&e));
            },
        };

        // TODO: only do this in repl
        if let Ok(result) = self.interpret_expression(src, &tokens) {
            match result {
                Ok(v) => return Ok(Some(v.value_string())),
                Err(e) => return Err(e),
            }
        }

        match self.interpret_statement(src, &tokens) {
            Ok(_) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn interpret_expression<'b>(
        &self,
        src: &Source,
        tokens: &'b Tokens,
    ) -> Result<Result<Rc<LoxValue>, String>, String> {
        let expr = match parse_expr(&tokens) {
            Ok(v) => v,
            Err(e) => {
                return Err(src.format_error(&e));
            },
        };

        Ok(eval(&expr, &self.env))
    }

    fn interpret_statement<'b>(&self, src: &Source, tokens: &'b Tokens) -> Result<(), String> {
        let ast = match parse(&tokens) {
            Ok(v) => v,
            Err(e) => {
                return Err(src.format_error(&e));
            },
        };

        for statement in ast.top {
            exec(&statement, &self.env)?;
        }

        Ok(())
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
