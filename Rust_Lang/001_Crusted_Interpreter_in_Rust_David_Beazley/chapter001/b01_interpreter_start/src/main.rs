use crate::chunk::{Chunk, OpCode::*};

mod chunk;
mod value;

fn main() {
    println!("Hello, lox!");

    let mut chunk = Chunk::new();
    chunk.write(OP_RETURN);
    let constant = chunk.add_constant(1.2);
    chunk.write(OP_CONSTANT(constant as f64));
    chunk.disassemble("test chunk");
}
