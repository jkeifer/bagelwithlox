use super::reader::Source;
use super::environment::Environment;
use super::parser::parse;
use super::tokenizer::tokenize;
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

    pub fn interpret(&self, src: Source) -> Result<(), String> {
        let tokens = tokenize(src);
        let ast = parse(tokens);
        evaluate(ast);
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpret() {
        Interpreter::new().interpret(());
    }
}
