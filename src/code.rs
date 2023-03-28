use std::{fmt::Debug, ops::Add};

use crate::InterpretResult;

#[derive(Clone)]
pub struct Code {
    pub(crate) code: OpCode,
    pub(crate) line: usize,
}

impl Debug for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.code {
            OpCode::Constant(v) => write!(f, "{:?}{:>12}'{v:?}'", self.code, " "),
            _ => write!(f, "{:?}", self.code),
        }
    }
}

#[derive(Clone)]
pub enum OpCode {
    Constant(Value),
    Negate,
    Add,
    Return,
}

impl Debug for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Constant(_) => write!(f, "CONSTANT"),
            OpCode::Negate => write!(f, "NEGATE"),
            OpCode::Add => write!(f, "ADD"),
            OpCode::Return => write!(f, "RETURN"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Value {
    Double(f32),
}

impl Value {
    pub fn negate(&self) -> Result<Self, InterpretResult> {
        match self {
            Value::Double(v) => Ok(Value::Double(-*v)),
        }
    }
}

impl Add for Value {
    type Output = Result<Self, InterpretResult>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Double(lhs), Value::Double(rhs)) => Ok(Value::Double(lhs + rhs)),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Double(v) => write!(f, "{v}"),
        }
    }
}
