use std::io::Error;
use std::fs;

pub struct Source {
    filename: Option<String>,
    content: String,
}

impl Source {
    pub fn from_string(content: &str) -> Source {
        Source{
            filename: None,
            content: content.to_string(),
        }
    }

    pub fn from_file(path: &str) -> Result<Source, Error> {
        Ok(Source{
            filename: Some(path.to_string()),
            content: fs::read_to_string(path)?,
        })
    }

    pub fn get_filename(&self) -> Option<&str> {
        match &self.filename {
            Some(string) => Some(&string),
            None => None,
        }
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_source() {
        Source::from_string("content");
    }
}
