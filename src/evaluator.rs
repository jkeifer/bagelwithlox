use super::parser::AST;

pub fn evaluate(ast: AST) {
    println!("Evaluating...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        evaluate(());
    }
}
