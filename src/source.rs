use std::fs;
use std::rc::Rc;


pub trait SourceError {
    fn get_position(&self) -> Option<FilePosition>;
    fn get_message(&self) -> &str;
    fn get_type(&self) -> &str;
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FilePosition {
    pub lineno: usize,
    pub linepos: usize,
    pub length: usize,
}

impl FilePosition {
    pub fn new(lineno: usize, linepos: usize) -> FilePosition {
        FilePosition {
            lineno,
            linepos,
            length: 0,
        }
    }

    pub fn nwl(lineno: usize, linepos: usize, length: usize) -> FilePosition {
        FilePosition {
            lineno,
            linepos,
            length,
        }
    }

    pub fn char_inc(&mut self, ch: char) {
        if ch == '\n' {
            self.lineno += 1;
            self.linepos = 0;
        } else {
            self.linepos += 1;
        }
    }
}


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

    pub fn format_error<E: SourceError>(&self, err: &E) -> String {
        let pos = match err.get_position() {
            Some(pos) => pos,
            None => {
                return format!(
                    "{}: {}",
                    err.get_type(),
                    err.get_message(),
                );
            },
        };
        let length = match pos.length {
            0 => 1,
            v => v,
        };
        let line = match self.content.split('\n').nth(pos.lineno - 1) {
            Some(v) => v,
            None => return format!(
                "SourceError: could not find line in source when formatting error message: {}",
                pos.lineno,
            ),
        };
        let line_err = " ".repeat(pos.linepos - 1) + &"^".repeat(length);

        format!(
            "Encountered and error on line {}:\n\n{}\n{}\n\n{}: {}",
            pos.lineno,
            line,
            line_err,
            err.get_type(),
            err.get_message(),
        )
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
