use std::{fmt::Debug, ops::Add};

use crate::InterpretError;

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
    // Sub,
    Return,
}

impl Debug for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Constant(_) => write!(f, "CONSTANT"),
            OpCode::Negate => write!(f, "NEGATE"),
            OpCode::Add => write!(f, "ADD"),
            // OpCode::Sub => write!(f, "SUB"),
            OpCode::Return => write!(f, "RETURN"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Value {
    Int(isize),
    Double(f32),
}

impl Value {
    pub fn negate(&self) -> Result<Self, InterpretError> {
        match self {
            Value::Double(v) => Ok(Value::Double(-*v)),
            Value::Int(v) => Ok(Value::Int(-*v)),
        }
    }
}

impl Add for Value {
    type Output = Result<Self, InterpretError>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Double(lhs), Value::Double(rhs)) => Ok(Value::Double(lhs + rhs)),
            (Value::Int(lhs), Value::Int(rhs)) => Ok(Value::Int(lhs + rhs)),
            (Value::Int(lhs), Value::Double(rhs)) => Ok(Value::Double(lhs as f32 + rhs)),
            (Value::Double(lhs), Value::Int(rhs)) => Ok(Value::Double(lhs + rhs as f32)),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Double(v) => write!(f, "{v}"),
            Value::Int(v) => write!(f, "{v}"),
        }
    }
}
