mod code;
mod compiler;
mod scanner;
mod vm;

use std::io::{self, BufRead, Write};

use vm::Vm;

#[derive(Debug)]
pub enum InterpretError {
    CompileError,
    RuntimeError,
}

fn main() {
    let args = std::env::args();

    let mut vm = Vm::new();

    if args.len() == 1 {
        let mut lines = io::stdin().lock().lines();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let Some(Ok(line)) = lines.next() else { break };
            match vm.interpret(&line) {
                Ok(_) => continue,
                Err(e) => println!("{e:?}"),
            }
        }
    }
}
