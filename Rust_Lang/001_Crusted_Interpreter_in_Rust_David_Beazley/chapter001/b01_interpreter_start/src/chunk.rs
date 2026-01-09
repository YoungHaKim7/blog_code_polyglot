#![allow(non_camel_case_types)]

use crate::value::Value;

#[derive(Debug)]
pub enum OpCode {
    OP_RETURN,
    OP_CONSTANT(usize),
}

#[derive(Debug)]
pub struct Chunk {
    code: Vec<OpCode>,
    constants: Vec<Value>,
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

    pub fn write(&mut self, op: OpCode, line: usize) {
        self.code.push(op);
        self.lines.push(line);
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {name} ==");
        for (n, inst) in self.code.iter().enumerate() {
            println!("{n:04}, {inst:?}");
        }
    }
}
