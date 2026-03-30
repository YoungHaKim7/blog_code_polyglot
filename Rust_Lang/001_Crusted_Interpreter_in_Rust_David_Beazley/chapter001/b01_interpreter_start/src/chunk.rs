#![allow(non_camel_case_types)]

use crate::value::Value;

#[derive(Debug)]
pub enum OpCode {
    OP_RETURN,
    OP_CONSTANT(usize),
}

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {name} ==");
        for (n, inst) in self.code.iter().enumerate() {
            let op = match inst {
                OpCode::OP_CONSTANT(n) => format!("OP_CONSTANT({:?})", self.constants[*n]),
                i => format!("{i:?}"),
            };
            println!("{n:04}, {:5} {op:?}", self.lines[n]);
        }
    }

    pub fn write(&mut self, op: OpCode, line: usize) {
        self.code.push(op);
        self.lines.push(line);
    }
}
