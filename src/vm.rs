use crate::{
    code::{OpCode, Value},
    compiler::Parser,
    InterpretError,
};

const STACK_MAX: usize = 256;

#[derive(Default)]
pub struct Vm {
    stack: Vec<Value>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(STACK_MAX),
        }
    }

    pub fn interpret(&mut self, source: &str) -> Result<(), InterpretError> {
        let parser = Parser::new(source);

        #[cfg(feature = "dbg_code")]
        {
            use std::io::{stdout, Write};
            let mut lock = stdout().lock();
            for (i, ins) in parser.clone().enumerate() {
                writeln!(lock, "{i:<04} {:>4} {ins:?}", ins.line).unwrap();
            }
        }

        for (i, instruction) in parser.enumerate() {
            #[cfg(feature = "dbg_trace")]
            {
                use std::io::{stdout, Write};
                let mut lock = stdout().lock();
                write!(lock, "          ").unwrap();
                for slot in self.stack.iter() {
                    write!(lock, "[ {slot:?} ]").unwrap();
                }
                write!(lock, "\n").unwrap();
                writeln!(lock, "{i:<04} {:>4} {instruction:?}", instruction.line).unwrap();
            }
            match instruction.code {
                OpCode::Constant(v) => self.stack.push(v.clone()),
                OpCode::Negate => {
                    let Some(v) = self.stack.pop() else {return Err(InterpretError::RuntimeError)};
                    self.stack.push(v.negate());
                }
                OpCode::Add => {
                    let Some((rhs, lhs)) = self.stack.pop().zip(self.stack.pop()) else {return Err(InterpretError::RuntimeError)};
                    self.stack.push(lhs + rhs);
                }
                OpCode::Sub => {
                    let Some((rhs, lhs)) = self.stack.pop().zip(self.stack.pop()) else {return Err(InterpretError::RuntimeError)};
                    self.stack.push(lhs - rhs);
                }
                OpCode::Mul => {
                    let Some((rhs, lhs)) = self.stack.pop().zip(self.stack.pop()) else {return Err(InterpretError::RuntimeError)};
                    self.stack.push(lhs * rhs);
                }
                OpCode::Div => {
                    let Some((rhs, lhs)) = self.stack.pop().zip(self.stack.pop()) else {return Err(InterpretError::RuntimeError)};
                    self.stack.push(lhs / rhs);
                }
                OpCode::Return => {
                    println!("{:?}", self.stack.pop().unwrap());
                    break;
                }
            }
        }

        Ok(())
    }
}
