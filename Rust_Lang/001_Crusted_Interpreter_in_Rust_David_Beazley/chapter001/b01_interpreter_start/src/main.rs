use crate::{
    chunk::{Chunk, OpCode::*},
    vm::VM,
};

mod chunk;
mod value;
mod vm;

fn main() {
    println!("Hello, lox!");

    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write(OP_CONSTANT(constant), 1);
    chunk.write(OP_RETURN, 1);
    chunk.disassemble("test chunk");
    let mut vm = VM::new();
    vm.interpret(&chunk);
}
