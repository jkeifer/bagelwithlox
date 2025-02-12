use super::source::Source;
use super::environment::Environment;
use super::parser::parse;
use super::evaluator::evaluate;
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

    pub fn interpret(&mut self, src: &mut Source) -> Result<String, String> {
        let tokens = tokenize(src)?;
        dbg!(&tokens);
        let ast = parse(&tokens)?;
        evaluate(ast, &mut self.env)
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
