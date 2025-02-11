use super::tokenizer::Tokens;


pub type AST = ();



pub fn parse(tokens: &Tokens) -> Result<AST, String> {
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
    }
}
