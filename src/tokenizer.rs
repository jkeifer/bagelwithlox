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


#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue<'a> {
    LNumber(f64),
    LString(&'a str),
}


#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub typ: TokenType,
    pub pos: FilePosition,
    pub lexeme: &'a str,
    pub literal: Option<LiteralValue<'a>>,
}

impl<'a> Token<'a> {
    pub fn new(typ: TokenType, pos: FilePosition, lexeme: &'a str) -> Token<'a> {
        Token {
            typ,
            pos,
            lexeme,
            literal: None,
        }
    }

    pub fn nol(typ: TokenType, pos: FilePosition) -> Token<'a> {
        Token {
            typ,
            pos,
            lexeme: typ.lexeme().expect("Cannot call nol without a known-lexeme type!"),
            literal: None,
        }
    }

    pub fn new_literal(
        typ: TokenType,
        pos: FilePosition,
        lexeme: &'a str,
        literal: LiteralValue<'a>,
    ) -> Token<'a> {
        Token {
            typ,
            pos,
            lexeme,
            literal: Some(literal),
        }
    }

    pub fn get_type(&self) -> &TokenType {
        &self.typ
    }

    pub fn get_position(&self) -> FilePosition {
        self.pos
    }

    fn match_identifier_token(pos: FilePosition, lexeme: &'a str) -> Token<'a> {
        use TokenType::*;
        match lexeme {
            "and" => Token::new(And, pos, lexeme),
            "class" => Token::new(Class, pos, lexeme),
            "else" => Token::new(Else, pos, lexeme),
            "false" => Token::new(False, pos, lexeme),
            "fun" => Token::new(Fun, pos, lexeme),
            "for" => Token::new(For, pos, lexeme),
            "if" => Token::new(If, pos, lexeme),
            "nil" => Token::new(Nil, pos, lexeme),
            "or" => Token::new(Or, pos, lexeme),
            "print" => Token::new(Print, pos, lexeme),
            "return" => Token::new(Return, pos, lexeme),
            "super" => Token::new(Super, pos, lexeme),
            "this" => Token::new(This, pos, lexeme),
            "true" => Token::new(True, pos, lexeme),
            "var" => Token::new(Var, pos, lexeme),
            "while" => Token::new(While, pos, lexeme),
            _ => Token::new(Identifier, pos, lexeme),
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
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
    Str,
    Number,

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

impl fmt::Display for TokenType {
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

impl TokenType {
    pub fn lexeme(&self) -> Option<&'static str> {
        use TokenType::*;
        match self {
            LeftParen => Some("("),
            RightParen => Some(")"),
            LeftBrace => Some("{"),
            RightBrace => Some("}"),
            Comma => Some(","),
            Dot => Some("."),
            Minus => Some("-"),
            Plus => Some("+"),
            SemiColon => Some(";"),
            Star => Some("*"),
            Bang => Some("!"),
            BangEqual => Some("!="),
            Equal => Some("="),
            EqualEqual => Some("=="),
            Greater => Some(">"),
            GreaterEqual => Some(">="),
            Less => Some("<"),
            LessEqual => Some("<="),
            Slash => Some("/"),
            Comment => Some("//"),
            And => Some("and"),
            Class => Some("class"),
            Else => Some("else"),
            False => Some("false"),
            Fun => Some("fun"),
            For => Some("for"),
            If => Some("if"),
            Nil => Some("nil"),
            Or => Some("or"),
            Print => Some("print"),
            Return => Some("return"),
            Super => Some("super"),
            This => Some("this"),
            True => Some("true"),
            Var => Some("var"),
            While => Some("while"),
            _ => None
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
            '(' => Token::new(LeftParen, pos, "("),
            ')' => Token::new(RightParen, pos, ")"),
            '{' => Token::new(LeftBrace, pos, "{"),
            '}' => Token::new(RightBrace, pos, "}"),
            ',' => Token::new(Comma, pos, ","),
            '.' => Token::new(Dot, pos, "."),
            '-' => Token::new(Minus, pos, "-"),
            '+' => Token::new(Plus, pos, "+"),
            ';' => Token::new(SemiColon, pos, ";"),
            '*' => Token::new(Star, pos, "*"),
            '!' => match ch_idxs.next_if_eq('=') {
                Some(_) => {
                    pos.length = 2;
                    Token::new(BangEqual, pos, "!=")
                },
                None => Token::new(Bang, pos, "!"),
            },
            '=' => match ch_idxs.next_if_eq('=') {
                Some(_) => {
                    pos.length = 2;
                    Token::new(EqualEqual, pos, "==")
                },
                None => Token::new(Equal, pos, "="),
            },
            '>' => match ch_idxs.next_if_eq('=') {
                Some(_) => {
                    pos.length = 2;
                    Token::new(GreaterEqual, pos, ">=")
                },
                None => Token::new(Greater, pos, ">"),
            },
            '<' => match ch_idxs.next_if_eq('=') {
                Some(_) => {
                    pos.length = 2;
                    Token::new(LessEqual, pos, "<=")
                },
                None => Token::new(Less, pos, "<"),
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
                    //Token::new(Comment, pos, &src.content[start..=end])
                    continue;
                },
                None =>Token::new(Slash, pos, "/"),
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

                Token::new_literal(
                    Str,
                    pos,
                    &src.content[start..=end],
                    LiteralValue::LString(&src.content[start+1..=end-1]),

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

                Token::new_literal(
                    Number,
                    pos,
                    lexeme,
                    LiteralValue::LNumber(value),
                )
            },

            // Identifier or keyword
            _ if ch.is_alphabetic() || ch == '_' => {
                let mut end = ch_idxs.next_index().unwrap_or(src.content.len());

                while let Some((_end, _)) = ch_idxs.next_if(
                    |&(_, ch)| ch.is_alphanumeric() || ch == '_',
                ) {
                    end = ch_idxs.next_index().unwrap_or(_end + 1);
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
                Token::new(LeftParen, FilePosition::nwl(1, 1, 1), "("),
                Token::new(RightParen, FilePosition::nwl(1, 3, 1), ")"),
                Token::new(LeftBrace, FilePosition::nwl(1, 5, 1), "{"),
                Token::new(RightBrace, FilePosition::nwl(1, 7, 1), "}"),
                Token::new(Comma, FilePosition::nwl(1, 9, 1), ","),
                Token::new(Dot, FilePosition::nwl(1, 11, 1), "."),
                Token::new(Plus, FilePosition::nwl(1, 13, 1), "+"),
                Token::new(Minus, FilePosition::nwl(1, 15, 1), "-"),
                Token::new(SemiColon, FilePosition::nwl(1, 17, 1), ";"),
                Token::new(Star, FilePosition::nwl(1, 19, 1), "*"),
                Token::new(Slash, FilePosition::nwl(1, 21, 1), "/"),
                Token::new(Bang, FilePosition::nwl(1, 23, 1), "!"),
                Token::new(Equal, FilePosition::nwl(1, 25, 1), "="),
                Token::new(Less, FilePosition::nwl(1, 27, 1), "<"),
                Token::new(Greater, FilePosition::nwl(1, 29, 1), ">"),
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
                Token::new(BangEqual, FilePosition::nwl(1, 1, 2), "!="),
                Token::new(EqualEqual, FilePosition::nwl(1, 4, 2), "=="),
                Token::new(LessEqual, FilePosition::nwl(1, 7, 2), "<="),
                Token::new(GreaterEqual, FilePosition::nwl(1, 10, 2), ">="),
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
                Token::new(Identifier, FilePosition::nwl(1, 1, 3), "abc"),
                Token::new(Identifier, FilePosition::nwl(1, 5, 6), "abc123"),
                Token::new(Identifier, FilePosition::nwl(1, 12, 7), "_x_3_4_"),
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
                Token::new(And, FilePosition::nwl(1, 1, 3), "and"),
                Token::new(Class, FilePosition::nwl(1, 5, 5), "class"),
                Token::new(Else, FilePosition::nwl(1, 11, 4), "else"),
                Token::new(False, FilePosition::nwl(1, 16, 5), "false"),
                Token::new(For, FilePosition::nwl(1, 22, 3), "for"),
                Token::new(Fun, FilePosition::nwl(1, 26, 3), "fun"),
                Token::new(If, FilePosition::nwl(1, 30, 2), "if"),
                Token::new(Nil, FilePosition::nwl(1, 33, 3), "nil"),
                Token::new(Or, FilePosition::nwl(1, 37, 2), "or"),
                Token::new(Print, FilePosition::nwl(1, 40, 5), "print"),
                Token::new(Return, FilePosition::nwl(1, 46, 6), "return"),
                Token::new(Super, FilePosition::nwl(1, 53, 5), "super"),
                Token::new(This, FilePosition::nwl(1, 59, 4), "this"),
                Token::new(True, FilePosition::nwl(1, 64, 4), "true"),
                Token::new(Var, FilePosition::nwl(1, 69, 3), "var"),
                Token::new(While, FilePosition::nwl(1, 73, 5), "while"),
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
                Token::new_literal(
                    Number,
                    FilePosition::nwl(1, 1, 1),
                    "1",
                    LiteralValue::LNumber(1.0),
                ),
                Token::new_literal(
                    Number,
                    FilePosition::nwl(1, 3, 4),
                    "1234",
                    LiteralValue::LNumber(1234.0),
                ),
                Token::new_literal(
                    Number,
                    FilePosition::nwl(1, 8, 5),
                    "12.34",
                    LiteralValue::LNumber(12.34),
                ),
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
                Token::new_literal(
                    Str,
                    FilePosition::nwl(1, 1, 1),
                    "\"hello\"",
                    LiteralValue::LString("hello"),
                ),
                Token::new_literal(
                    Str,
                    FilePosition::nwl(1, 9, 1),
                    "\"wor\nld\"",
                    LiteralValue::LString("wor\nld"),
                ),
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
                Token::nol(LeftBrace, FilePosition::nwl(1, 1, 1)),
                Token::nol(RightBrace, FilePosition::nwl(1, 2, 1)),
                Token::nol(LeftParen, FilePosition::nwl(1, 3, 1)),
                Token::nol(RightParen, FilePosition::nwl(1, 5, 1)),
                Token::nol(Comma, FilePosition::nwl(1, 6, 1)),
                Token::nol(Dot, FilePosition::nwl(1, 7, 1)),
                Token::nol(Minus, FilePosition::nwl(1, 8, 1)),
                Token::nol(Plus, FilePosition::nwl(1, 9, 1)),
                Token::nol(SemiColon, FilePosition::nwl(1, 10, 1)),
                Token::nol(Star, FilePosition::nwl(2, 1, 1)),
                Token::nol(Slash, FilePosition::nwl(2, 2, 1)),
                Token::nol(Bang, FilePosition::nwl(2, 3, 1)),
                Token::nol(BangEqual, FilePosition::nwl(2, 4, 2)),
                Token::nol(Greater, FilePosition::nwl(2, 6, 1)),
                Token::nol(GreaterEqual, FilePosition::nwl(2, 7, 2)),
                Token::nol(Less, FilePosition::nwl(2, 9, 1)),
                Token::nol(LessEqual, FilePosition::nwl(2, 10, 2)),
                Token::nol(EqualEqual, FilePosition::nwl(2, 12, 2)),
                Token::nol(Equal, FilePosition::nwl(2, 14, 1)),
                Token::nol(Else, FilePosition::nwl(2, 15, 4)),
                Token::new(Identifier, FilePosition::nwl(2, 20, 5), "death"),
                Token::new_literal(
                    Number,
                    FilePosition::nwl(2, 26, 5),
                    "11.12",
                    LiteralValue::LNumber(11.12),
                ),
                //Token::new(Eof, FilePosition::nwl(2, 32, 0)),
            ],
        );
    }

    #[test]
    fn test_x() {
        let tstr = "x";
        let source = Source::from_string(tstr.to_string());
        let tokens = tokenize(&source).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::new(Identifier, FilePosition::nwl(1, 1, 1), "x"),
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
