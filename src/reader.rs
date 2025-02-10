use std::fs;

pub struct Source {
    filename: String,
    content: String,
}

impl Source {
    pub fn from_string(content: &str) -> Source {
        Source{
            filename: "__str__".to_string(),
            content: content.to_string(),
        }
    }

    pub fn from_file(path: &str) -> Result<Source, String> {
        match fs::read_to_string(path) {
            Ok(content) =>  Ok(Source{
                filename: path.to_string(),
                content,
            }),
            Err(e) => Err(format!("Failed to read file '{}': {}", path, e)),
        }
    }

    pub fn get_filename(&self) -> &str {
        &self.filename
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
