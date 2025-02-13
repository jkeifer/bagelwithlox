use std::iter::Peekable;
use std::str::CharIndices;
use crate::source::FilePosition;


pub struct TokenIter<'a> {
    peekable: Peekable<CharIndices<'a>>,
    pub filepos: FilePosition,
}

impl<'a> TokenIter<'a> {
    pub fn new(peekable: Peekable<CharIndices<'a>>) -> TokenIter<'a> {
        TokenIter {
            peekable,
            filepos: FilePosition::new(1, 0),
        }
    }

    fn maybe_count_char(&mut self, got: Option<(usize, char)>) -> Option<(usize, char)> {
        if let Some((_, ch)) = got {
            self.filepos.char_inc(ch);
        }
        got
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<(usize, char)> {
        let got = self.peekable.next();
        self.maybe_count_char(got)
    }
}

impl<'a> TokenIter<'a> {
    pub fn peek(&mut self) -> Option<&(usize, char)> {
        self.peekable.peek()
    }

    pub fn next_if(&mut self, func: impl FnOnce(&(usize, char)) -> bool) -> Option<(usize, char)> {
        let got = self.peekable.next_if(func);
        self.maybe_count_char(got)
    }

    pub fn next_if_eq(&mut self, expected: char) -> Option<(usize, char)>
    {
        let got = self.peekable.next_if(|(_, ch)| *ch == expected);
        self.maybe_count_char(got)
    }

    pub fn next_if_not_eq(&mut self, unexpected: char) -> Option<(usize, char)>
    {
        let got = self.peekable.next_if(|(_, ch)| *ch != unexpected);
        self.maybe_count_char(got)
    }
    pub fn next_index(&mut self) -> Option<usize> {
        match self.peek() {
            Some((idx, _)) => Some(*idx),
            None => None,
        }
    }
}
