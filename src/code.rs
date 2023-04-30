use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

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
    Sub,
    Mul,
    Div,
    Return,
}

impl Debug for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Constant(_) => write!(f, "CONSTANT"),
            OpCode::Negate => write!(f, "NEGATE"),
            OpCode::Add => write!(f, "ADD"),
            OpCode::Sub => write!(f, "SUB"),
            OpCode::Mul => write!(f, "MUL"),
            OpCode::Div => write!(f, "Div"),
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
    pub fn negate(&self) -> Self {
        match self {
            Value::Double(v) => Value::Double(-*v),
            Value::Int(v) => Value::Int(-*v),
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Double(lhs), Value::Double(rhs)) => Value::Double(lhs + rhs),
            (Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs + rhs),
            (Value::Int(lhs), Value::Double(rhs)) => Value::Double(lhs as f32 + rhs),
            (Value::Double(lhs), Value::Int(rhs)) => Value::Double(lhs + rhs as f32),
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Double(lhs), Value::Double(rhs)) => Value::Double(lhs - rhs),
            (Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs - rhs),
            (Value::Int(lhs), Value::Double(rhs)) => Value::Double(lhs as f32 - rhs),
            (Value::Double(lhs), Value::Int(rhs)) => Value::Double(lhs - rhs as f32),
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Double(lhs), Value::Double(rhs)) => Value::Double(lhs * rhs),
            (Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs * rhs),
            (Value::Int(lhs), Value::Double(rhs)) => Value::Double(lhs as f32 * rhs),
            (Value::Double(lhs), Value::Int(rhs)) => Value::Double(lhs * rhs as f32),
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Double(lhs), Value::Double(rhs)) => Value::Double(lhs / rhs),
            (Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs / rhs),
            (Value::Int(lhs), Value::Double(rhs)) => Value::Double(lhs as f32 / rhs),
            (Value::Double(lhs), Value::Int(rhs)) => Value::Double(lhs / rhs as f32),
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
