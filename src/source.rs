use std::fs;
use std::rc::Rc;

pub struct Source {
    pub filename: String,
    pub content: Rc<String>,
}

impl Source {
    fn new(filename: String, content: String) -> Source {
        Source{
            filename,
            content: Rc::new(content),
        }
    }

    pub fn from_string(content: String) -> Source {
        Source::new(
            "__str__".to_string(),
            content,
        )
    }

    pub fn from_file(path: &str) -> Result<Source, String> {
        match fs::read_to_string(path) {
            Ok(content) =>  Ok(Source::new(
                path.to_string(),
                content,
            )),
            Err(e) => Err(format!("Failed to read file '{}': {}", path, e)),
        }
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
