use std::collections::VecDeque;

use crate::{
    code::{Code, OpCode, Value},
    scanner::{Scanner, Token, TokenKind},
};

#[derive(Clone)]
pub struct Parser<'a> {
    scanner: Scanner<'a>,
    codes: VecDeque<Code>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            scanner: Scanner::new(source),
            codes: VecDeque::with_capacity(2),
        }
    }

    fn handle_token(&mut self, token: Token) -> Option<Code> {
        match token.kind {
            TokenKind::Minus => {
                self.codes.push_back(Code {
                    code: OpCode::Negate,
                    line: token.line,
                });
                self.scanner.next().and_then(|t| self.handle_token(t))
            }
            TokenKind::Plus => {
                self.codes.push_back(Code {
                    code: OpCode::Add,
                    line: token.line,
                });
                self.scanner.next().and_then(|t| self.handle_token(t))
            }
            TokenKind::Number => {
                self.codes.push_back(Code {
                    code: OpCode::Constant(Value::Double(token.lexeme.parse().unwrap())),
                    line: token.line,
                });
                self.next()
            }
            TokenKind::Eof => Some(Code {
                code: OpCode::Return,
                line: token.line,
            }),
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Code;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.codes.is_empty() {
            return self.codes.pop_back();
        }

        let Some(token) = self.scanner.next() else { return None };

        self.handle_token(token)
    }
}

// pub struct Compiler<'a> {
//     parser: Parser<'a>,
// }
//
// impl<'a> Compiler<'a> {
//     pub fn new(source: &'a str) -> Self {
//         Self {
//             parser: Parser::new(source),
//         }
//     }
// }
