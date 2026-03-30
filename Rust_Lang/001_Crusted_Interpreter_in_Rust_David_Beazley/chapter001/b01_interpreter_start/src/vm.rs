use crate::chunk::{Chunk, OpCode};

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM<'a> {
    chunk: Option<&'a Chunk>,
    ip: usize,
    trace_execution: bool,
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        Self {
            chunk: None,
            ip: 0,
            trace_execution: false,
        }
    }

    pub fn interpret(&mut self, chunk: &'a Chunk) -> InterpretResult {
        self.chunk = Some(chunk);
        self.run()
    }

    pub fn run(&mut self) -> InterpretResult {
        let chunk = self.chunk.unwrap();
        loop {
            let instruction = &chunk.code[self.ip];
            self.ip += 1;
            if self.trace_execution {
                println!("{instruction:?}")
            }
            match instruction {
                OpCode::OP_RETURN => return InterpretResult::Ok,
                OpCode::OP_CONSTANT(n) => {
                    let constant = chunk.constants[*n];
                    println!("{:?}", constant);
                }
                _ => todo!("{instruction:?}"),
            }
        }
    }
}
