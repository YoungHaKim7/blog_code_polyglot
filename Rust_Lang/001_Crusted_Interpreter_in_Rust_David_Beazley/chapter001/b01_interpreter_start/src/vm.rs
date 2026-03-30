use crate::chunk::Chunk;

pub struct VM<'a> {
    chunk: Option<&'a Chunk>,
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        Self { chunk: None }
    }
}
