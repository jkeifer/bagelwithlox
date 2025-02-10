use std::fs;
use super::tokenizer;
use super::tokenizer::{Token, Tokens};

pub struct Source<'a> {
    filename: String,
    content: String,
    tokens: Tokens<'a>,
}

impl<'a> Source<'a> {
    fn new(filename: String, content: String) -> Source<'a> {
        Source{
            filename,
            content,
            tokens: Tokens::new(),
        }
    }

    pub fn from_string(content: String) -> Source<'a> {
        Source::new(
            "__str__".to_string(),
            content,
        )
    }

    pub fn from_file(path: &str) -> Result<Source<'a>, String> {
        match fs::read_to_string(path) {
            Ok(content) =>  Ok(Source::new(
                path.to_string(),
                content,
            )),
            Err(e) => Err(format!("Failed to read file '{}': {}", path, e)),
        }
    }

    pub fn get_filename(&self) -> &str {
        &self.filename
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_tokens(&self) -> &Tokens {
        &self.tokens
    }

    pub fn add_token(&mut self, token: Token<'a>) {
        self.tokens.push(token)
    }

    pub fn tokenize(&mut self) {
        tokenizer::tokenize(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_source() {
        Source::from_string("content".to_string());
    }
}
