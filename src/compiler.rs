use std::collections::{HashMap, VecDeque};

use crate::{
    code::{Code, OpCode, Value},
    scanner::{Scanner, Token, TokenKind},
};

// Given a token, what is its precendence and how should it be interpreted?

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    None,
    Assignment, // =
    // Or,         // or
    // And,        // and
    // Equality,   // == !=
    // Comparison, // < > <= >=
    Term, // + -
    // Factor,     // * /
    Unary, // ! -
           // Call,       // . ()
           // Primary,
}

impl From<u8> for Precedence {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Assignment,
            1 => Self::Term,
            _ => Self::Unary,
        }
    }
}

#[derive(Debug, Clone)]
enum Rule {
    Unary,
    Binary,
    Grouping,
    Number,
}

#[derive(Debug, Clone)]
struct ParseRule {
    prefix: Option<Rule>,
    infix: Option<Rule>,
    precedence: Precedence,
}

impl ParseRule {
    const fn new(prefix: Option<Rule>, infix: Option<Rule>, precedence: Precedence) -> Self {
        Self {
            prefix,
            infix,
            precedence,
        }
    }
}

const RULES: [(TokenKind, ParseRule); 4] = [
    (
        TokenKind::Minus,
        ParseRule::new(Some(Rule::Unary), Some(Rule::Binary), Precedence::Term),
    ),
    (
        TokenKind::Plus,
        ParseRule::new(None, Some(Rule::Binary), Precedence::Term),
    ),
    (
        TokenKind::Number,
        ParseRule::new(Some(Rule::Number), None, Precedence::None),
    ),
    (TokenKind::Eof, ParseRule::new(None, None, Precedence::None)),
];

#[derive(Clone)]
pub struct Parser<'a> {
    scanner: Scanner<'a>,
    previous: Option<Token<'a>>,
    current: Option<Token<'a>>,
    codes: VecDeque<Code>,
    parse_rule: HashMap<TokenKind, ParseRule>,
    finished: bool,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            scanner: Scanner::new(source),
            previous: None,
            current: None,
            codes: VecDeque::with_capacity(2),
            parse_rule: HashMap::from(RULES),
            finished: false,
        }
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();
        self.current = self.scanner.next();
    }

    fn get_prev_rule(&mut self) -> Option<&ParseRule> {
        self.parse_rule.get(&self.previous.clone()?.kind)
    }

    fn get_curr_rule(&self) -> Option<&ParseRule> {
        self.parse_rule.get(&self.current.clone()?.kind)
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment)
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        // dbg!(&self.codes, &self.previous, &self.current, &precedence);

        let Some(rule) = self.get_prev_rule() else { return };
        let Some(prefix) = &rule.prefix else { unreachable!() };
        match prefix {
            Rule::Unary => self.unary(),
            Rule::Number => self.number(),
            _ => todo!(),
        };

        while self
            .get_curr_rule()
            .map_or(false, |rule| precedence <= rule.precedence)
        {
            self.advance();
            // dbg!(&self.codes, &self.previous, &self.current, &precedence);
            let Some(infix) = &self.get_prev_rule().expect("No parse rule").infix else { break };

            match infix {
                Rule::Unary => self.unary(),
                Rule::Binary => self.binary(),
                Rule::Grouping => todo!(),
                Rule::Number => self.unary(),
            };
        }
    }

    fn unary(&mut self) {
        let token = self.previous.clone().expect("Missing token");

        self.parse_precedence(Precedence::Unary);

        let operator = match token.kind {
            TokenKind::Minus => Code {
                code: OpCode::Negate,
                line: token.line,
            },
            _ => unreachable!(),
        };

        self.codes.push_back(operator);
    }

    fn binary(&mut self) {
        let token = self.previous.clone().expect("Missing token");
        let rule = self.parse_rule.get(&token.kind).expect("Missing rule");

        self.parse_precedence((rule.precedence.clone() as u8 + 1).into());

        let operator = match token.kind {
            TokenKind::Plus => Code {
                code: OpCode::Add,
                line: token.line,
            },
            _ => unreachable!(),
        };

        self.codes.push_back(operator);
    }

    fn number(&mut self) {
        let token = self.previous.clone().expect("Missing token");

        let code = match token.kind {
            TokenKind::Number => {
                let value = if token.lexeme.contains('.') {
                    Value::Double(token.lexeme.parse().unwrap())
                } else {
                    Value::Int(token.lexeme.parse().unwrap())
                };
                Code {
                    code: OpCode::Constant(value),
                    line: token.line,
                }
            }
            _ => unreachable!(),
        };

        self.codes.push_back(code)
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Code;

    fn next(&mut self) -> Option<Self::Item> {
        // dbg!(&self.codes);
        if self.finished {
            return None;
        }

        if !self.codes.is_empty() {
            return self.codes.pop_front();
        }

        self.advance();

        if let Some(Token {
            kind: TokenKind::Eof,
            line,
            ..
        }) = self.previous
        {
            self.finished = true;
            return Some(Code {
                code: OpCode::Return,
                line,
            });
        }

        self.expression();
        self.next()
    }
}
