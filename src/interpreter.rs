use std::rc::Rc;

use crate::evaluator::interpret;

use super::source::Source;
use super::environment::Environment;
use super::parser::parse;
use super::tokenizer::tokenize;

pub struct Interpreter {
    env: Rc<Environment>,
}

impl<'a> Interpreter {
    pub fn new() -> Interpreter {
        Interpreter{
            env: Environment::new(),
        }
    }

    pub fn interpret<'b>(&mut self, src: &'b mut Source) -> Result<Option<String>, String> {
        let tokens = match tokenize(src) {
            Ok(v) => v,
            Err(e) => {
                return Err(src.format_error(&e));
            },
        };

        let ast = match parse(&tokens) {
            Ok(v) => v,
            Err(e) => {
                return Err(src.format_error(&e));
            },
        };

        Ok(match interpret(&ast.top, &self.env)? {
            Some(v) => Some(v.value_string()),
            None => None,
        })

        // TODO: only do this in repl
        //if let Ok(result) = self.interpret_expression(src, &tokens) {
        //    match result {
        //        Ok(v) => return Ok(Some(v.value_string())),
        //        Err(e) => return Err(e),
        //    }
        //}

        //match self.interpret_statement(src, &tokens) {
        //    Ok(_) => Ok(None),
        //    Err(e) => Err(e),
        //}
    }

    //fn interpret_expression<'b>(
    //    &self,
    //    src: &Source,
    //    tokens: &'b Tokens,
    //) -> Result<Result<LoxValue, String>, String> {
    //    let expr = match parse_expr(&tokens) {
    //        Ok(v) => v,
    //        Err(e) => {
    //            return Err(src.format_error(&e));
    //        },
    //    };

    //    Ok(eval(&expr, &self.env))
    //}

    //fn interpret_statement<'b>(&self, src: &Source, tokens: &'b Tokens) -> Result<(), String> {
    //    let ast = match parse(&tokens) {
    //        Ok(v) => v,
    //        Err(e) => {
    //            return Err(src.format_error(&e));
    //        },
    //    };

    //    for statement in &*ast.top {
    //        exec(&statement, &self.env)?;
    //    }

    //    Ok(())
    //}

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
