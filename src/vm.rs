use crate::{
    code::{OpCode, Value},
    compiler::Parser,
    InterpretResult,
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

    pub fn interpret(&mut self, source: &str) -> Result<(), InterpretResult> {
        let parser = Parser::new(source);
        #[cfg(feature = "dbg_code")]
        {
            for (i, ins) in parser.clone().enumerate() {
                println!("{i:<04} {:>4} {ins:?}", ins.line);
            }
        }

        for (i, instruction) in parser.enumerate() {
            #[cfg(feature = "dbg_trace")]
            {
                print!("          ");
                for slot in self.stack.iter() {
                    print!("[ {slot:?} ]");
                }
                print!("\n");
                println!("{i:<04} {:>4} {instruction:?}", instruction.line);
            }
            match instruction.code {
                OpCode::Constant(v) => self.stack.push(v.clone()),
                OpCode::Negate => {
                    let Some(v) = self.stack.pop() else {return Err(InterpretResult::RuntimeError)};
                    let v = v.negate()?;
                    self.stack.push(v);
                }
                OpCode::Add => {
                    let Some((lhs, rhs)) = self.stack.pop().zip(self.stack.pop()) else {return Err(InterpretResult::RuntimeError)};
                    self.stack.push((lhs + rhs)?);
                }
                OpCode::Return => {
                    println!("{:?}", self.stack.pop());
                    break;
                }
            }
        }

        Ok(())
    }
}
