use crate::chunk::{Chunk, OpCode};

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM<'a> {
    chunk: Option<&'a Chunk>,
    ip: usize,
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        Self { chunk: None, ip: 0 }
    }

    pub fn interpret(&mut self, chunk: &'a Chunk) -> InterpretResult {
        self.chunk = Some(chunk);
        InterpretResult::Ok
    }

    pub fn run(&mut self) -> InterpretResult {
        let chunk = self.chunk.unwrap();
        loop {
            let instruction = &chunk.code[self.ip];
            match instruction {
                OpCode::OP_RETURN => return InterpretResult::Ok,
                _ => todo!("{instruction:?}"),
            }
        }
    }
}
