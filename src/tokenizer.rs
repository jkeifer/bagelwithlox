use super::source::Source;
pub use crate::tokenizer::token_iter::{FilePosition, TokenIter};

mod token_iter;

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub lexeme: &'a str,
    pub filepos: FilePosition,
}

impl<'a> Token<'a> {
    pub fn new(
        token_type: TokenType<'a>,
        lexeme: &'a str,
        filepos: FilePosition,
    ) -> Token<'a> {
        Token{
            token_type,
            lexeme,
            filepos,
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum TokenType<'a> {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Star,

    // One Or Two Character Tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Slash,
    Comment,

    // Literals.
    Identifier,
    Str{ value: &'a str },
    Number{ value: f64 },

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}


pub type Tokens<'a> = Vec<Token<'a>>;


fn match_identifier_token_type(id: &str) -> TokenType {
    match id {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "fun" => TokenType::Fun,
        "for" => TokenType::For,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,
        _ => TokenType::Identifier,
    }
}

pub fn find_number_end(token_iter: &mut TokenIter) -> usize {
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
        tokens.push(match ch {
            '\n' => {
                continue;
            }
            _ if ch.is_whitespace() => continue,
            '(' => Token::new(TokenType::LeftParen, &src.content[start..=start], ch_idxs.filepos),
            ')' => Token::new(TokenType::RightParen, &src.content[start..=start], ch_idxs.filepos),
            '{' => Token::new(TokenType::LeftBrace, &src.content[start..=start], ch_idxs.filepos),
            '}' => Token::new(TokenType::RightBrace, &src.content[start..=start], ch_idxs.filepos),
            ',' => Token::new(TokenType::Comma, &src.content[start..=start], ch_idxs.filepos),
            '.' => Token::new(TokenType::Dot, &src.content[start..=start], ch_idxs.filepos),
            '-' => Token::new(TokenType::Minus, &src.content[start..=start], ch_idxs.filepos),
            '+' => Token::new(TokenType::Plus, &src.content[start..=start], ch_idxs.filepos),
            ';' => Token::new(TokenType::SemiColon, &src.content[start..=start], ch_idxs.filepos),
            '*' => Token::new(TokenType::Star, &src.content[start..=start], ch_idxs.filepos),
            '!' => {
                let start_pos = ch_idxs.filepos;
                if let Some(_) = ch_idxs.next_if_eq('=') {
                    Token::new(
                        TokenType::BangEqual,
                        &src.content[start..=start+1],
                        start_pos,
                    )
                } else {
                    Token::new(TokenType::Bang, &src.content[start..=start], start_pos)
                }
            },
            '=' => {
                let start_pos = ch_idxs.filepos;
                if let Some(_) = ch_idxs.next_if_eq('=') {
                    Token::new(
                        TokenType::EqualEqual,
                        &src.content[start..=start+1],
                        start_pos,
                    )
                } else {
                    Token::new(TokenType::Equal, &src.content[start..=start], start_pos)
                }
            },
            '>' => {
                let start_pos = ch_idxs.filepos;
                if let Some(_) = ch_idxs.next_if_eq('=') {
                    Token::new(
                        TokenType::GreaterEqual,
                        &src.content[start..=start+1],
                        start_pos,
                    )
                } else {
                    Token::new(TokenType::Greater, &src.content[start..=start], start_pos)
                }
            },
            '<' => {
                let start_pos = ch_idxs.filepos;
                if let Some(_) = ch_idxs.next_if_eq('=') {
                    Token::new(
                        TokenType::LessEqual,
                        &src.content[start..=start+1],
                        start_pos,
                    )
                } else {
                    Token::new(TokenType::Less, &src.content[start..=start], start_pos)
                }
            },
            '/' => {
                let start_pos = ch_idxs.filepos;
                if let Some(_) = ch_idxs.next_if_eq('/') {
                    // we have a comment, and we'll consume
                    // all content to the end of the line
                    let mut end = start;
                    while let Some((_end, _)) = ch_idxs.next_if_not_eq('\n') {
                        end = ch_idxs.next_index().unwrap_or(_end);
                    };
                    Token::new(TokenType::Comment, &src.content[start..=end], start_pos)
                } else {
                    Token::new(TokenType::Slash, &src.content[start..=start], start_pos)
                }
            },

            // String
            '"' => {
                let end: usize;
                let start_pos = ch_idxs.filepos;

                while let Some(_) = ch_idxs.next_if_not_eq('"') {}
                if let Some((_end, _)) = ch_idxs.next() {
                    // we know next is a "
                    end = _end;
                } else {
                    // we got to the end without a "
                    return Err(format!(
                        "Unterminated string literal: line {}, pos {}",
                        start_pos.lineno,
                        start_pos.linepos,
                    ));
                }

                Token::new(
                    TokenType::Str { value: &src.content[start+1..=end-1]},
                    &src.content[start..=end],
                    start_pos,
                )
            },

            // Number
            _ if ch.is_digit(10) => {
                let start_pos = ch_idxs.filepos;
                let end = start + find_number_end(&mut ch_idxs);
                let lexeme = &src.content[start..=end];
                let value = match lexeme.parse() {
                    Ok(val) => val,
                    Err(e) => return Err(format!(
                        "Invalid numeric literal line {}, pos {}.\nInput value {}. Error: {}",
                        start_pos.lineno,
                        start_pos.linepos,
                        lexeme,
                        e,
                    )),
                };
                dbg!(&value);

                Token::new(
                    TokenType::Number { value },
                    lexeme,
                    start_pos,
                )
            },

            // Identifier or keyword
            _ if ch.is_alphabetic() || ch == '_' => {
                let mut end = ch_idxs.next_index().unwrap_or(src.content.len()+1);
                let start_pos = ch_idxs.filepos;

                while let Some((_end, _)) = ch_idxs.next_if(
                    |&(_, ch)| ch.is_alphanumeric() || ch == '_',
                ) {
                    end = ch_idxs.next_index().unwrap_or(_end+1);
                }

                let lexeme = &src.content[start..end];
                Token::new(
                    match_identifier_token_type(lexeme),
                    lexeme,
                    start_pos,
                )
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

    let mut eof_pos = ch_idxs.filepos;
    eof_pos.linepos += 1;
    tokens.push(Token::new(TokenType::Eof, "", eof_pos));

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
                Token::new(TokenType::LeftParen, "(", FilePosition::new(1, 1)),
                Token::new(TokenType::RightParen, ")", FilePosition::new(1, 3)),
                Token::new(TokenType::LeftBrace, "{", FilePosition::new(1, 5)),
                Token::new(TokenType::RightBrace, "}", FilePosition::new(1, 7)),
                Token::new(TokenType::Comma, ",", FilePosition::new(1, 9)),
                Token::new(TokenType::Dot, ".", FilePosition::new(1, 11)),
                Token::new(TokenType::Plus, "+", FilePosition::new(1, 13)),
                Token::new(TokenType::Minus, "-", FilePosition::new(1, 15)),
                Token::new(TokenType::SemiColon, ";", FilePosition::new(1, 17)),
                Token::new(TokenType::Star, "*", FilePosition::new(1, 19)),
                Token::new(TokenType::Slash, "/", FilePosition::new(1, 21)),
                Token::new(TokenType::Bang, "!", FilePosition::new(1, 23)),
                Token::new(TokenType::Equal, "=", FilePosition::new(1, 25)),
                Token::new(TokenType::Less, "<", FilePosition::new(1, 27)),
                Token::new(TokenType::Greater, ">", FilePosition::new(1, 29)),
                Token::new(TokenType::Eof, "", FilePosition::new(1, 30)),
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
                Token::new(TokenType::BangEqual, "!=", FilePosition::new(1, 1)),
                Token::new(TokenType::EqualEqual, "==", FilePosition::new(1, 4)),
                Token::new(TokenType::LessEqual, "<=", FilePosition::new(1, 7)),
                Token::new(TokenType::GreaterEqual, ">=", FilePosition::new(1, 10)),
                Token::new(TokenType::Eof, "", FilePosition::new(1, 12)),
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
                Token::new(TokenType::Identifier, "abc", FilePosition::new(1, 1)),
                Token::new(TokenType::Identifier, "abc123", FilePosition::new(1, 5)),
                Token::new(TokenType::Identifier, "_x_3_4_", FilePosition::new(1, 12)),
                Token::new(TokenType::Eof, "", FilePosition::new(2, 1)),
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
                Token::new(TokenType::And, "and", FilePosition::new(1, 1)),
                Token::new(TokenType::Class, "class", FilePosition::new(1, 5)),
                Token::new(TokenType::Else, "else", FilePosition::new(1, 11)),
                Token::new(TokenType::False, "false", FilePosition::new(1, 16)),
                Token::new(TokenType::For, "for", FilePosition::new(1, 22)),
                Token::new(TokenType::Fun, "fun", FilePosition::new(1, 26)),
                Token::new(TokenType::If, "if", FilePosition::new(1, 30)),
                Token::new(TokenType::Nil, "nil", FilePosition::new(1, 33)),
                Token::new(TokenType::Or, "or", FilePosition::new(1, 37)),
                Token::new(TokenType::Print, "print", FilePosition::new(1, 40)),
                Token::new(TokenType::Return, "return", FilePosition::new(1, 46)),
                Token::new(TokenType::Super, "super", FilePosition::new(1, 53)),
                Token::new(TokenType::This, "this", FilePosition::new(1, 59)),
                Token::new(TokenType::True, "true", FilePosition::new(1, 64)),
                Token::new(TokenType::Var, "var", FilePosition::new(1, 69)),
                Token::new(TokenType::While, "while", FilePosition::new(1, 73)),
                Token::new(TokenType::Eof, "", FilePosition::new(1, 78)),
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
                Token::new(TokenType::Number{ value: 1.0}, "1", FilePosition::new(1, 1)),
                Token::new(TokenType::Number{ value: 1234.0}, "1234", FilePosition::new(1, 3)),
                Token::new(TokenType::Number{ value: 12.34}, "12.34", FilePosition::new(1, 8)),
                Token::new(TokenType::Eof, "", FilePosition::new(1, 13)),
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
                Token::new(TokenType::Str{ value: "hello"}, "\"hello\"", FilePosition::new(1, 1)),
                Token::new(TokenType::Str{ value: "world"}, "\"world\"", FilePosition::new(1, 9)),
                Token::new(TokenType::Eof, "", FilePosition::new(1, 16)),
            ],
        );
    }
}
