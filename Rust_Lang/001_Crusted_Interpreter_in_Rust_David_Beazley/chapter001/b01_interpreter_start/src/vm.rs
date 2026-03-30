use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM<'a> {
    chunk: Option<&'a Chunk>,
    ip: usize,
    stack: Vec<Value>,
    trace_execution: bool,
    trace_stack: bool,
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        Self {
            chunk: None,
            ip: 0,
            stack: Vec::new(),
            trace_execution: false,
            trace_stack: false,
        }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
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
            if self.trace_stack {
                println!("      {:?}", self.stack);
            }
            match instruction {
                OpCode::OP_RETURN => {
                    println!("{:?}", self.pop());
                    return InterpretResult::Ok;
                }
                OpCode::OP_CONSTANT(n) => {
                    let constant = chunk.constants[*n];
                    self.push(constant);
                }
                OpCode::OP_NEGATE => {
                    let val = self.pop();
                    self.push(-val);
                }
                _ => todo!("{instruction:?}"),
            }
        }
    }
}
