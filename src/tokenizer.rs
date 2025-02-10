use super::source::Source;

pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub lexeme: &'a str,
    pub lineno: usize,
    pub linepos: usize,
}

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
    Slash,
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

    // Literals.
    Identifier,
    Str(&'a str),
    Number(f32),

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

    Eof
}

pub type Tokens<'a> = Vec<Token<'a>>;


pub fn tokenize(source: &mut Source) {
    println!("Tokenizing...");
    let mut char_indices = source.get_content().char_indices().peekable();
    let mut lineno: usize = 1;

    while let Some((start, ch)) = char_indices.next() {
        println!("{} {}" , start, ch);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        tokenize(());
    }
}
