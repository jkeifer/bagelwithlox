use super::source::Source;
pub use crate::tokenizer::token_iter::{FilePosition, TokenIter};

mod token_iter;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    // Single-character tokens.
    LeftParen{ pos: FilePosition },
    RightParen{ pos: FilePosition },
    LeftBrace{ pos: FilePosition },
    RightBrace{ pos: FilePosition },
    Comma{ pos: FilePosition },
    Dot{ pos: FilePosition },
    Minus{ pos: FilePosition },
    Plus{ pos: FilePosition },
    SemiColon{ pos: FilePosition },
    Star{ pos: FilePosition },

    // One Or Two Character Tokens.
    Bang{ pos: FilePosition },
    BangEqual{ pos: FilePosition },
    Equal{ pos: FilePosition },
    EqualEqual{ pos: FilePosition },
    Greater{ pos: FilePosition },
    GreaterEqual{ pos: FilePosition },
    Less{ pos: FilePosition },
    LessEqual{ pos: FilePosition },
    Slash{ pos: FilePosition },
    Comment{ pos: FilePosition, lexeme: &'a str },

    // Literals.
    Identifier{ pos: FilePosition, lexeme: &'a str },
    Str{ pos: FilePosition, lexeme: &'a str, value: &'a str },
    Number{ pos: FilePosition, lexeme: &'a str, value: f64 },

    // Keywords.
    And{ pos: FilePosition },
    Class{ pos: FilePosition },
    Else{ pos: FilePosition },
    False{ pos: FilePosition },
    Fun{ pos: FilePosition },
    For{ pos: FilePosition },
    If{ pos: FilePosition },
    Nil{ pos: FilePosition },
    Or{ pos: FilePosition },
    Print{ pos: FilePosition },
    Return{ pos: FilePosition },
    Super{ pos: FilePosition },
    This{ pos: FilePosition },
    True{ pos: FilePosition },
    Var{ pos: FilePosition },
    While{ pos: FilePosition },

    Eof{ pos: FilePosition },
}

impl<'a> Token<'a> {
    fn match_identifier_token(pos: FilePosition, id: &'a str) -> Token<'a> {
        use Token::*;
        match id {
            "and" => And{ pos },
            "class" => Class{ pos },
            "else" => Else{ pos },
            "false" => False{ pos },
            "fun" => Fun{ pos },
            "for" => For{ pos },
            "if" => If{ pos },
            "nil" => Nil{ pos },
            "or" => Or{ pos },
            "print" => Print{ pos },
            "return" => Return{ pos },
            "super" => Super{ pos },
            "this" => This{ pos },
            "true" => True{ pos },
            "var" => Var{ pos },
            "while" => While{ pos },
            _ => Identifier{ pos, lexeme: id },
        }
    }
}


pub type Tokens<'a> = Vec<Token<'a>>;


fn find_number_end(token_iter: &mut TokenIter) -> usize {
    let mut end = 0;
    let mut has_dot = false;

    while let Some((_, ch)) = token_iter.peek() {
        match ch {
            '.' => {
                if !has_dot {
                    end += 1;
                    has_dot = true;
                    token_iter.next();
                } else {
                    break;
                }
            },
            _ if ch.is_digit(10) => {
                end += 1;
                token_iter.next();
            },
            _ => {break;},
        }
    }
    end
}

pub fn tokenize<'a>(src: &'a Source) -> Result<Tokens<'a>, String> {
    println!("Tokenizing...");
    let mut ch_idxs = TokenIter::new(src.content.char_indices().peekable());
    let mut tokens = Tokens::new();

    while let Some((start, ch)) = ch_idxs.next() {
        let pos = ch_idxs.filepos;
        tokens.push(match ch {
            '\n' => {
                continue;
            }
            _ if ch.is_whitespace() => continue,
            '(' => Token::LeftParen{ pos },
            ')' => Token::RightParen{ pos },
            '{' => Token::LeftBrace{ pos },
            '}' => Token::RightBrace{ pos },
            ',' => Token::Comma{ pos },
            '.' => Token::Dot{ pos },
            '-' => Token::Minus{ pos },
            '+' => Token::Plus{ pos },
            ';' => Token::SemiColon{ pos },
            '*' => Token::Star{ pos },
            '!' => match ch_idxs.next_if_eq('=') {
                Some(_) => Token::BangEqual{ pos },
                None => Token::Bang{ pos },
            },
            '=' => match ch_idxs.next_if_eq('=') {
                Some(_) => Token::EqualEqual{ pos },
                None => Token::Equal{ pos },
            },
            '>' => match ch_idxs.next_if_eq('=') {
                Some(_) => Token::GreaterEqual{ pos },
                None => Token::Greater{ pos },
            },
            '<' => match ch_idxs.next_if_eq('=') {
                Some(_) => Token::LessEqual{ pos },
                None => Token::Less{ pos },
            },
            '/' => match ch_idxs.next_if_eq('/') {
                Some(_) => {
                    // we have a comment, and we'll consume
                    // all content to the end of the line
                    let mut end = start;
                    while let Some((_end, _)) = ch_idxs.next_if_not_eq('\n') {
                        end = ch_idxs.next_index().unwrap_or(_end);
                    };
                    Token::Comment{ pos, lexeme: &src.content[start..=end] }
                },
                None =>Token::Slash{ pos },
            },

            // String
            '"' => {
                let end: usize;

                while let Some(_) = ch_idxs.next_if_not_eq('"') {}
                match ch_idxs.next() {
                    Some((_end, _)) => {
                        // we know next is a "
                        end = _end;
                    },
                    None => {
                        // we got to the end without a "
                        return Err(format!(
                            "Unterminated string literal: line {}, pos {}",
                            pos.lineno,
                            pos.linepos,
                        ));
                    }
                }

                Token::Str{
                    pos,
                    lexeme: &src.content[start..=end],
                    value: &src.content[start+1..=end-1],
                }
            },

            // Number
            _ if ch.is_digit(10) => {
                let end = start + find_number_end(&mut ch_idxs);
                let lexeme = &src.content[start..=end];
                let value = match lexeme.parse() {
                    Ok(val) => val,
                    Err(e) => return Err(format!(
                        "Invalid numeric literal line {}, pos {}.\nInput value {}. Error: {}",
                        pos.lineno,
                        pos.linepos,
                        lexeme,
                        e,
                    )),
                };

                Token::Number{
                    pos,
                    lexeme,
                    value,
                }
            },

            // Identifier or keyword
            _ if ch.is_alphabetic() || ch == '_' => {
                let mut end = ch_idxs.next_index().unwrap_or(src.content.len()+1);

                while let Some((_end, _)) = ch_idxs.next_if(
                    |&(_, ch)| ch.is_alphanumeric() || ch == '_',
                ) {
                    end = ch_idxs.next_index().unwrap_or(_end+1);
                }

                let lexeme = &src.content[start..end];
                Token::match_identifier_token(pos, lexeme)
            },

            // Invalid char
            other => {
                let pos = ch_idxs.filepos;
                return Err(format!(
                    "Bad character line {}, pos {}: {}",
                    pos.lineno,
                    pos.linepos,
                    other,
                ));
            },
        });

    }

    let mut pos = ch_idxs.filepos;
    pos.linepos += 1;
    tokens.push(Token::Eof{ pos });

    Ok(tokens)
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_symbols() {
        let tstr = "( ) { } , . + - ; * / ! = < >";
        let source = Source::from_string(tstr.to_string());
        let tokens = tokenize(&source).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LeftParen{ pos: FilePosition::new(1, 1) },
                Token::RightParen{ pos: FilePosition::new(1, 3) },
                Token::LeftBrace{ pos: FilePosition::new(1, 5) },
                Token::RightBrace{ pos: FilePosition::new(1, 7) },
                Token::Comma{ pos: FilePosition::new(1, 9) },
                Token::Dot{ pos: FilePosition::new(1, 11) },
                Token::Plus{ pos: FilePosition::new(1, 13) },
                Token::Minus{ pos: FilePosition::new(1, 15) },
                Token::SemiColon{ pos: FilePosition::new(1, 17) },
                Token::Star{ pos: FilePosition::new(1, 19) },
                Token::Slash{ pos: FilePosition::new(1, 21) },
                Token::Bang{ pos: FilePosition::new(1, 23) },
                Token::Equal{ pos: FilePosition::new(1, 25) },
                Token::Less{ pos: FilePosition::new(1, 27) },
                Token::Greater{ pos: FilePosition::new(1, 29) },
                Token::Eof{ pos: FilePosition::new(1, 30) },
           ],
        );
    }

    #[test]
    fn test_tricky_symbols() {
        let tstr = "!= == <= >=";
        let source = Source::from_string(tstr.to_string());
        let tokens = tokenize(&source).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::BangEqual{ pos: FilePosition::new(1, 1) },
                Token::EqualEqual{ pos: FilePosition::new(1, 4) },
                Token::LessEqual{ pos: FilePosition::new(1, 7) },
                Token::GreaterEqual{ pos: FilePosition::new(1, 10) },
                Token::Eof{ pos: FilePosition::new(1, 12) },
            ],
        );
    }

    #[test]
    fn test_identifiers() {
        let tstr = "abc abc123 _x_3_4_\n";
        let source = Source::from_string(tstr.to_string());
        let tokens = tokenize(&source).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Identifier{ pos: FilePosition::new(1, 1), lexeme: "abc"},
                Token::Identifier{ pos: FilePosition::new(1, 5), lexeme: "abc123" },
                Token::Identifier{ pos: FilePosition::new(1, 12), lexeme: "_x_3_4_" },
                Token::Eof{ pos: FilePosition::new(2, 1) },
            ],
        );
    }

    #[test]
    fn test_keywords() {
        let tstr = "and class else false for fun if nil or print return super this true var while";
        let source = Source::from_string(tstr.to_string());
        let tokens = tokenize(&source).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::And{ pos: FilePosition::new(1, 1) },
                Token::Class{ pos: FilePosition::new(1, 5) },
                Token::Else{ pos: FilePosition::new(1, 11) },
                Token::False{ pos: FilePosition::new(1, 16) },
                Token::For{ pos: FilePosition::new(1, 22) },
                Token::Fun{ pos: FilePosition::new(1, 26) },
                Token::If{ pos: FilePosition::new(1, 30) },
                Token::Nil{ pos: FilePosition::new(1, 33) },
                Token::Or{ pos: FilePosition::new(1, 37) },
                Token::Print{ pos: FilePosition::new(1, 40) },
                Token::Return{ pos: FilePosition::new(1, 46) },
                Token::Super{ pos: FilePosition::new(1, 53) },
                Token::This{ pos: FilePosition::new(1, 59) },
                Token::True{ pos: FilePosition::new(1, 64) },
                Token::Var{ pos: FilePosition::new(1, 69) },
                Token::While{ pos: FilePosition::new(1, 73) },
                Token::Eof{ pos: FilePosition::new(1, 78) },
            ],
        );
    }

    #[test]
    fn test_numbers() {
        let tstr = "1 1234 12.34";
        let source = Source::from_string(tstr.to_string());
        let tokens = tokenize(&source).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number{ value: 1.0, lexeme: "1", pos: FilePosition::new(1, 1) },
                Token::Number{ value: 1234.0, lexeme: "1234", pos: FilePosition::new(1, 3) },
                Token::Number{ value: 12.34, lexeme: "12.34", pos: FilePosition::new(1, 8) },
                Token::Eof{ pos: FilePosition::new(1, 13) },
            ],
        );
    }

    #[test]
    fn test_strings() {
        let tstr = "\"hello\" \"world\"";
        let source = Source::from_string(tstr.to_string());
        let tokens = tokenize(&source).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Str{ value: "hello", lexeme:  "\"hello\"", pos: FilePosition::new(1, 1) },
                Token::Str{ value: "world", lexeme: "\"world\"", pos: FilePosition::new(1, 9) },
                Token::Eof{ pos: FilePosition::new(1, 16) },
            ],
        );
    }
}
