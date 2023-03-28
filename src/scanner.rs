use std::str::CharIndices;

#[derive(Clone)]
pub struct Scanner<'a> {
    source: &'a str,
    chars: CharIndices<'a>,
    line: usize,
    finished: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.char_indices(),
            line: 1,
            finished: false,
        }
    }

    fn make_token(&self, kind: TokenKind, start: usize, end: usize) -> Token<'a> {
        Token::new(kind, self.line, &self.source[start..=end])
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub line: usize,
    pub lexeme: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, line: usize, lexeme: &'a str) -> Self {
        Self { kind, line, lexeme }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    Minus,
    Plus,

    Number,

    // Error,
    Eof,
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        match self.chars.next() {
            Some((start, c)) => match c {
                '-' => Some(self.make_token(TokenKind::Minus, start, start)),
                '+' => Some(self.make_token(TokenKind::Plus, start, start)),
                c if c.is_ascii_digit() => {
                    let end = self
                        .chars
                        .by_ref()
                        .take_while(|(_, c)| c.is_ascii_digit() || *c == '.')
                        .fold(0, |_, (i, _)| i);

                    Some(self.make_token(TokenKind::Number, start, end))
                }
                '\n' => {
                    self.line += 1;
                    self.next()
                }
                c if c.is_whitespace() => self.next(),
                _ => Some(self.make_token(TokenKind::Eof, start, start)),
            },
            None => {
                self.line += 1;
                self.finished = true;
                Some(Token::new(TokenKind::Eof, self.line, ""))
            }
        }
    }
}
