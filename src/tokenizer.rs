use std::fmt;
use crate::source::{
    FilePosition,
    Source,
    SourceError,
};

mod token_iter;
pub use self::token_iter::TokenIter;


const TOKENIZE_ERROR: &'static str = "TokenizeError";


#[derive(Debug)]
pub struct TokenizeError {
    pos: Option<FilePosition>,
    msg: String,
}

impl SourceError for TokenizeError {
    fn get_message(&self) -> &str {
        &self.msg
    }

    fn get_position(&self) -> Option<FilePosition> {
        self.pos
    }

    fn get_type(&self) -> &str {
        &TOKENIZE_ERROR
    }
}

impl TokenizeError {
    fn new(pos: FilePosition, msg: String) -> TokenizeError {
        TokenizeError {
            pos: Some(pos),
            msg,
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    typ: TokenType<'a>,
    pos: FilePosition,
}

impl<'a> Token<'a> {
    pub fn new(typ: TokenType<'a>, pos: FilePosition) -> Token<'a> {
        Token {
            typ,
            pos,
        }
    }

    pub fn get_type(&self) -> &TokenType<'a> {
        &self.typ
    }

    pub fn get_position(&self) -> FilePosition {
        self.pos
    }

    fn match_identifier_token(pos: FilePosition, id: &'a str) -> Token<'a> {
        use TokenType::*;
        match id {
            "and" => Token::new(And, pos),
            "class" => Token::new(Class, pos),
            "else" => Token::new(Else, pos),
            "false" => Token::new(False, pos),
            "fun" => Token::new(Fun, pos),
            "for" => Token::new(For, pos),
            "if" => Token::new(If, pos),
            "nil" => Token::new(Nil, pos),
            "or" => Token::new(Or, pos),
            "print" => Token::new(Print, pos),
            "return" => Token::new(Return, pos),
            "super" => Token::new(Super, pos),
            "this" => Token::new(This, pos),
            "true" => Token::new(True, pos),
            "var" => Token::new(Var, pos),
            "while" => Token::new(While, pos),
            _ => Token::new(Identifier{ lexeme: id }, pos),
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
    Comment{ lexeme: &'a str },

    // Literals.
    Identifier{ lexeme: &'a str },
    Str{ pos_end: FilePosition, lexeme: &'a str, value: &'a str },
    Number{ lexeme: &'a str, value: f64 },

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

impl<'a> fmt::Display for TokenType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            TokenType::Comment{..} => "Comment".to_string(),
            TokenType::Identifier{..} => "Identifier".to_string(),
            TokenType::Str{..} => "Str".to_string(),
            TokenType::Number{..} => "Number".to_string(),
            _ => format!("{:?}", self),
        })
    }
}

impl<'a> TokenType<'a> {
    pub fn lexeme(&self) -> &str {
        use TokenType::*;
        match self {
            LeftParen => "(",
            RightParen => ")",
            LeftBrace => "{",
            RightBrace => "}",
            Comma => ",",
            Dot => ".",
            Minus => "-",
            Plus => "+",
            SemiColon => ";",
            Star => "*",

            // One Or Two Character Tokens.
            Bang => "!",
            BangEqual => "!=",
            Equal => "=",
            EqualEqual => "==",
            Greater => ">",
            GreaterEqual => ">=",
            Less => "<",
            LessEqual => "<=",
            Slash => "/",
            Comment{ lexeme: _ } => "//",

            // Literals.
            Identifier{ lexeme: v } => v,
            Str{ pos_end: _, lexeme: v, value: _ } => v,
            Number{ lexeme: v, value: _ } => v,

            // Keywords.
            And => "and",
            Class => "class",
            Else => "else",
            False => "false",
            Fun => "fun",
            For => "for",
            If => "if",
            Nil => "nil",
            Or => "or",
            Print => "print",
            Return => "return",
            Super => "super",
            This => "this",
            True => "true",
            Var => "var",
            While => "while",

            // This token should probably be removed...
            Eof => "EOF",
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

pub fn tokenize<'a>(src: &'a Source) -> Result<Tokens<'a>, TokenizeError> {
    use TokenType::*;

    let mut ch_idxs = TokenIter::new(src.content.char_indices().peekable());
    let mut tokens = Tokens::new();

    while let Some((start, ch)) = ch_idxs.next() {
        let mut pos = ch_idxs.filepos;
        pos.length = 1;
        tokens.push(match ch {
            '\n' => {
                continue;
            }
            _ if ch.is_whitespace() => continue,
            '(' => Token::new(LeftParen, pos),
            ')' => Token::new(RightParen, pos),
            '{' => Token::new(LeftBrace, pos),
            '}' => Token::new(RightBrace, pos),
            ',' => Token::new(Comma, pos),
            '.' => Token::new(Dot, pos),
            '-' => Token::new(Minus, pos),
            '+' => Token::new(Plus, pos),
            ';' => Token::new(SemiColon, pos),
            '*' => Token::new(Star, pos),
            '!' => match ch_idxs.next_if_eq('=') {
                Some(_) => {
                    pos.length = 2;
                    Token::new(BangEqual, pos)
                },
                None => Token::new(Bang, pos),
            },
            '=' => match ch_idxs.next_if_eq('=') {
                Some(_) => {
                    pos.length = 2;
                    Token::new(EqualEqual, pos)
                },
                None => Token::new(Equal, pos),
            },
            '>' => match ch_idxs.next_if_eq('=') {
                Some(_) => {
                    pos.length = 2;
                    Token::new(GreaterEqual, pos)
                },
                None => Token::new(Greater, pos),
            },
            '<' => match ch_idxs.next_if_eq('=') {
                Some(_) => {
                    pos.length = 2;
                    Token::new(LessEqual, pos)
                },
                None => Token::new(Less, pos),
            },
            '/' => match ch_idxs.next_if_eq('/') {
                Some(_) => {
                    // we have a comment, and we'll consume
                    // all content to the end of the line
                    let mut end = start;
                    while let Some((_end, _)) = ch_idxs.next_if_not_eq('\n') {
                        end = ch_idxs.next_index().unwrap_or(_end);
                    };
                    pos.length += end - start;
                    Token::new(Comment{ lexeme: &src.content[start..=end] }, pos)
                },
                None =>Token::new(Slash, pos),
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
                        return Err(TokenizeError::new(
                            ch_idxs.filepos,
                            "unterminated string literal".to_string(),
                        ));
                    },
                }

                Token::new(
                    Str {
                        pos_end: ch_idxs.filepos,
                        lexeme: &src.content[start..=end],
                        value: &src.content[start+1..=end-1],
                    },
                    pos,
                )
            },

            // Number
            _ if ch.is_digit(10) => {
                let end = start + find_number_end(&mut ch_idxs);
                let lexeme = &src.content[start..=end];
                pos.length += end - start;

                let value = match lexeme.parse() {
                    Ok(val) => val,
                    Err(e) => {
                        return Err(TokenizeError::new(
                            pos,
                            format!("invalid numeric literal: {}", e),
                        ));
                    },
                };

                Token::new(
                    Number{
                        lexeme,
                        value,
                    },
                    pos,
                )
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
                pos.length = end - start;
                Token::match_identifier_token(pos, lexeme)
            },

            // Invalid char
            other => {
                let pos = ch_idxs.filepos;
                return Err(TokenizeError::new(
                    pos,
                    format!("bad character: {}", other),
                ));
            },
        });

    }

    //let mut pos = ch_idxs.filepos;
    //pos.linepos += 1;
    //tokens.push(Token::new(Eof, pos));

    Ok(tokens)
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use TokenType::*;

    #[test]
    fn test_symbols() {
        let tstr = "( ) { } , . + - ; * / ! = < >";
        let source = Source::from_string(tstr.to_string());
        let tokens = tokenize(&source).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::new(LeftParen, FilePosition::nwl(1, 1, 1)),
                Token::new(RightParen, FilePosition::nwl(1, 3, 1)),
                Token::new(LeftBrace, FilePosition::nwl(1, 5, 1)),
                Token::new(RightBrace, FilePosition::nwl(1, 7, 1)),
                Token::new(Comma, FilePosition::nwl(1, 9, 1)),
                Token::new(Dot, FilePosition::nwl(1, 11, 1)),
                Token::new(Plus, FilePosition::nwl(1, 13, 1)),
                Token::new(Minus, FilePosition::nwl(1, 15, 1)),
                Token::new(SemiColon, FilePosition::nwl(1, 17, 1)),
                Token::new(Star, FilePosition::nwl(1, 19, 1)),
                Token::new(Slash, FilePosition::nwl(1, 21, 1)),
                Token::new(Bang, FilePosition::nwl(1, 23, 1)),
                Token::new(Equal, FilePosition::nwl(1, 25, 1)),
                Token::new(Less, FilePosition::nwl(1, 27, 1)),
                Token::new(Greater, FilePosition::nwl(1, 29, 1)),
                //Token::new(Eof, FilePosition::nwl(1, 30, 0)),
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
                Token::new(BangEqual, FilePosition::nwl(1, 1, 2)),
                Token::new(EqualEqual, FilePosition::nwl(1, 4, 2)),
                Token::new(LessEqual, FilePosition::nwl(1, 7, 2)),
                Token::new(GreaterEqual, FilePosition::nwl(1, 10, 2)),
                //Token::new(Eof, FilePosition::nwl(1, 12, 0)),
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
                Token::new(Identifier { lexeme: "abc" }, FilePosition::nwl(1, 1, 3)),
                Token::new(Identifier { lexeme: "abc123" }, FilePosition::nwl(1, 5, 6)),
                Token::new(Identifier { lexeme: "_x_3_4_"}, FilePosition::nwl(1, 12, 7)),
                //Token::new(Eof, FilePosition::nwl(2, 1, 0)),
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
                Token::new(And, FilePosition::nwl(1, 1, 3)),
                Token::new(Class, FilePosition::nwl(1, 5, 5)),
                Token::new(Else, FilePosition::nwl(1, 11, 4)),
                Token::new(False, FilePosition::nwl(1, 16, 5)),
                Token::new(For, FilePosition::nwl(1, 22, 3)),
                Token::new(Fun, FilePosition::nwl(1, 26, 3)),
                Token::new(If, FilePosition::nwl(1, 30, 2)),
                Token::new(Nil, FilePosition::nwl(1, 33, 3)),
                Token::new(Or, FilePosition::nwl(1, 37, 2)),
                Token::new(Print, FilePosition::nwl(1, 40, 5)),
                Token::new(Return, FilePosition::nwl(1, 46, 6)),
                Token::new(Super, FilePosition::nwl(1, 53, 5)),
                Token::new(This, FilePosition::nwl(1, 59, 4)),
                Token::new(True, FilePosition::nwl(1, 64, 4)),
                Token::new(Var, FilePosition::nwl(1, 69, 3)),
                Token::new(While, FilePosition::nwl(1, 73, 5)),
                //Token::new(Eof, FilePosition::nwl(1, 78, 0)),
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
                Token::new(Number{ value: 1.0, lexeme: "1" }, FilePosition::nwl(1, 1, 1)),
                Token::new(Number{ value: 1234.0, lexeme: "1234" }, FilePosition::nwl(1, 3, 4)),
                Token::new(Number{ value: 12.34, lexeme: "12.34" }, FilePosition::nwl(1, 8, 5)),
                //Token::new(Eof, FilePosition::nwl(1, 13, 0)),
            ],
        );
    }

    #[test]
    fn test_strings() {
        let tstr = "\"hello\" \"wor\nld\"";
        let source = Source::from_string(tstr.to_string());
        let tokens = tokenize(&source).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::new(Str{
                    value: "hello",
                    lexeme:  "\"hello\"",
                    pos_end: FilePosition::new(1, 7),
                }, FilePosition::nwl(1, 1, 1)),
                Token::new(Str{
                    value: "wor\nld",
                    lexeme: "\"wor\nld\"",
                    pos_end: FilePosition::new(2, 3),
                }, FilePosition::nwl(1, 9, 1)),
                //Token::new(Eof, FilePosition::nwl(1, 16, 0)),
            ],
        );
    }

    #[test]
    fn test_mix() {
        let tstr = "{}( ),.-+; \n*/!!=>>=<<====else death 11.12 ";
        let source = Source::from_string(tstr.to_string());
        let tokens = tokenize(&source).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::new(LeftBrace, FilePosition::nwl(1, 1, 1)),
                Token::new(RightBrace, FilePosition::nwl(1, 2, 1)),
                Token::new(LeftParen, FilePosition::nwl(1, 3, 1)),
                Token::new(RightParen, FilePosition::nwl(1, 5, 1)),
                Token::new(Comma, FilePosition::nwl(1, 6, 1)),
                Token::new(Dot, FilePosition::nwl(1, 7, 1)),
                Token::new(Minus, FilePosition::nwl(1, 8, 1)),
                Token::new(Plus, FilePosition::nwl(1, 9, 1)),
                Token::new(SemiColon, FilePosition::nwl(1, 10, 1)),
                Token::new(Star, FilePosition::nwl(2, 1, 1)),
                Token::new(Slash, FilePosition::nwl(2, 2, 1)),
                Token::new(Bang, FilePosition::nwl(2, 3, 1)),
                Token::new(BangEqual, FilePosition::nwl(2, 4, 2)),
                Token::new(Greater, FilePosition::nwl(2, 6, 1)),
                Token::new(GreaterEqual, FilePosition::nwl(2, 7, 2)),
                Token::new(Less, FilePosition::nwl(2, 9, 1)),
                Token::new(LessEqual, FilePosition::nwl(2, 10, 2)),
                Token::new(EqualEqual, FilePosition::nwl(2, 12, 2)),
                Token::new(Equal, FilePosition::nwl(2, 14, 1)),
                Token::new(Else, FilePosition::nwl(2, 15, 4)),
                Token::new(Identifier{ lexeme:  "death" }, FilePosition::nwl(2, 20, 5)),
                Token::new(Number{ value: 11.12, lexeme: "11.12" }, FilePosition::nwl(2, 26, 5)),
                //Token::new(Eof, FilePosition::nwl(2, 32, 0)),
            ],
        );
    }

    #[test]
    #[should_panic(expected = "bad character: &")]
    fn test_illegal() {
        let tstr = " &";
        let source = Source::from_string(tstr.to_string());
        let _ = tokenize(&source).unwrap();
    }
}
