use super::source::Source;


#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub lexeme: &'a str,
    pub lineno: usize,
    pub linepos: usize,
}

impl<'a> Token<'a> {
    pub fn new(
        token_type: TokenType<'a>,
        lexeme: &'a str,
        lineno: usize,
        linepos: usize,
    ) -> Token<'a> {
        Token{
            token_type,
            lexeme,
            lineno,
            linepos,
        }
    }
}


#[derive(Debug)]
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
    Semicolon,
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


pub fn tokenize<'a>(source: &'a Source) -> Result<Tokens<'a>, String> {
    println!("Tokenizing...");
    let mut char_indices = source.content.char_indices().peekable();
    let mut lineno: usize = 1;
    let mut linepos: usize = 0;
    let mut tokens = Tokens::new();

    while let Some((start, ch)) = char_indices.next() {
        linepos += 1;
        tokens.push(match ch {
            '\n' => {
                lineno += 1;
                linepos = 0;
                continue;
            }
            _ if ch.is_whitespace() => continue,
            '(' => Token::new(TokenType::LeftParen, &source.content[start..=start], lineno, linepos),
            ')' => Token::new(TokenType::RightParen, &source.content[start..=start], lineno, linepos),
            '[' => Token::new(TokenType::LeftBrace, &source.content[start..=start], lineno, linepos),
            ']' => Token::new(TokenType::RightBrace, &source.content[start..=start], lineno, linepos),
            ',' => Token::new(TokenType::Comma, &source.content[start..=start], lineno, linepos),
            '.' => Token::new(TokenType::Dot, &source.content[start..=start], lineno, linepos),
            '-' => Token::new(TokenType::Minus, &source.content[start..=start], lineno, linepos),
            '+' => Token::new(TokenType::Plus, &source.content[start..=start], lineno, linepos),
            ';' => Token::new(TokenType::Semicolon, &source.content[start..=start], lineno, linepos),
            '*' => Token::new(TokenType::Star, &source.content[start..=start], lineno, linepos),
            '!' => {
                if let Some(_) = char_indices.next_if(
                    |&(_, ch)| ch == '=',
                ) {
                    linepos += 1;
                    Token::new(
                        TokenType::BangEqual,
                        &source.content[start..=start+1],
                        lineno,
                        linepos-1,
                    )
                } else {
                    Token::new(TokenType::Bang, &source.content[start..=start], lineno, linepos)
                }
            },
            '=' => {
                if let Some(_) = char_indices.next_if(
                    |&(_, ch)| ch == '=',
                ) {
                    linepos += 1;
                    Token::new(
                        TokenType::EqualEqual,
                        &source.content[start..=start+1],
                        lineno,
                        linepos-1,
                    )
                } else {
                    Token::new(TokenType::Equal, &source.content[start..=start], lineno, linepos)
                }
            },
            '>' => {
                if let Some(_) = char_indices.next_if(
                    |&(_, ch)| ch == '=',
                ) {
                    linepos += 1;
                    Token::new(
                        TokenType::GreaterEqual,
                        &source.content[start..=start+1],
                        lineno,
                        linepos-1,
                    )
                } else {
                    Token::new(TokenType::Greater, &source.content[start..=start], lineno, linepos)
                }
            },
            '<' => {
                if let Some(_) = char_indices.next_if(
                    |&(_, ch)| ch == '=',
                ) {
                    linepos += 1;
                    Token::new(
                        TokenType::LessEqual,
                        &source.content[start..=start+1],
                        lineno,
                        linepos-1,
                    )
                } else {
                    Token::new(TokenType::Less, &source.content[start..=start], lineno, linepos)
                }
            },
            '/' => {
                if let Some(_) = char_indices.next_if(
                    |&(_, ch)| ch == '/',
                ) {
                    // we have a comment, and we'll consume
                    // all content to the end of the line
                    while let Some(_) = char_indices.next_if(
                        |&(_, ch)| ch != '\n',
                    ) { };
                    continue;
                } else {
                    Token::new(TokenType::Slash, &source.content[start..=start], lineno, linepos)
                }
            },

            // String
            '"' => {
                let end: usize;
                let start_lineno = lineno;
                let start_linepos = linepos;

                while let Some((_end, ch)) = char_indices.next_if(
                    |&(_, ch)| ch != '"',
                ) {
                    linepos += 1;
                    if ch == '\n' {
                        lineno += 1;
                        linepos = 0;
                    }

                }

                if let Some((_end, _)) = char_indices.next() {
                    // we know next is a "
                    end = _end;
                    linepos += 1;
                } else {
                    // we got to the end without a "
                    return Err(format!(
                        "Unterminated string literal: line {}, pos {}",
                        start_lineno,
                        start_linepos,
                    ));
                }

                Token::new(
                    TokenType::Str { value: &source.content[start+1..=end-1]},
                    &source.content[start..=end],
                    start_lineno,
                    start_linepos,
                )
            },

            // Number
            _ if ch.is_digit(10) => {
                let mut end = start;
                let start_linepos = linepos;
                while let Some((_end, _)) = char_indices.next_if(
                    |&(_, ch)| ch.is_digit(10),
                ) {
                    end = _end;
                    linepos += 1;
                }

                let lexeme = &source.content[start..=end];
                let value = match lexeme.parse() {
                    Ok(val) => val,
                    Err(e) => return Err(format!(
                        "Invalid numeric literal line {}, pos {}.\nInput value {}. Error: {}",
                        lineno,
                        linepos,
                        lexeme,
                        e,
                    )),
                };

                Token::new(
                    TokenType::Number { value },
                    lexeme,
                    lineno,
                    start_linepos,
                )

            },

            // Identifier or keyword
            _ if ch.is_alphabetic() || ch == '_' => {
                let mut end: usize = start;
                let start_linepos = linepos;
                while let Some((_end, _)) = char_indices.next_if(
                    |&(_, ch)| ch.is_alphanumeric() || ch == '_',
                ) {
                    end = _end;
                    linepos += 1;
                }

                let lexeme = &source.content[start..=end];
                Token::new(
                    match_identifier_token_type(lexeme),
                    lexeme,
                    lineno,
                    start_linepos,
                )
            },

            // Invalid char
            other => {
                return Err(format!(
                    "Bad character line {}, pos {}: {}",
                    lineno,
                    linepos,
                    other,
                ));
            },
        });

    }

    dbg!(&tokens);
    //source.tokens = tokens;
    Ok(tokens)
}


#[cfg(test)]
mod tests {
    use super::*;
}
