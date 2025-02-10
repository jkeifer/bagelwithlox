use super::source::Source;
use super::environment::Environment;
use super::parser::parse;
use super::evaluator::evaluate;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter{
            environment: Environment::new(),
        }
    }

    pub fn interpret(&self, src: &mut Source) -> Result<(), String> {
        src.tokenize();
        let ast = parse(src.get_tokens());
        evaluate(ast);
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
