trait StreamingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

// Example 1: Tokenizer - splits text into tokens borrowing from internal buffer
struct Tokenizer<'input> {
    input: &'input str,
    position: usize,
}

impl<'input> Tokenizer<'input> {
    fn new(input: &'input str) -> Self {
        Self { input, position: 0 }
    }
}

impl<'input> StreamingIterator for Tokenizer<'input> {
    type Item<'a> = &'a str where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        while self.position < self.input.len() {
            let start = self.position;
            let byte = self.input.as_bytes()[self.position];
            self.position += 1;

            if !byte.is_ascii_whitespace() {
                return Some(&self.input[start..self.position]);
            }
        }
        None
    }
}

// Example 2: Parser - yields parsed elements borrowing from internal buffer
struct Parser<'input> {
    input: &'input str,
    position: usize,
}

impl<'input> Parser<'input> {
    fn new(input: &'input str) -> Self {
        Self { input, position: 0 }
    }
}

impl<'input> StreamingIterator for Parser<'input> {
    type Item<'a> = (&'a str, &'a str) where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.position >= self.input.len() {
            return None;
        }

        // Parse key=value pairs
        let remaining = &self.input[self.position..];
        if let Some(eq_pos) = remaining.find('=') {
            if let Some(end_pos) = remaining[eq_pos + 1..].find(|c| c == ',') {
                let key = &remaining[..eq_pos];
                let value = &remaining[eq_pos + 1..eq_pos + 1 + end_pos];
                self.position += eq_pos + 1 + end_pos + 1;
                return Some((key, value));
            }
        }
        None
    }
}

// Example 3: IO Buffer - yields chunks borrowing from internal buffer
struct IoBuffer {
    buffer: Vec<u8>,
    position: usize,
    chunk_size: usize,
}

impl IoBuffer {
    fn new(data: Vec<u8>, chunk_size: usize) -> Self {
        Self {
            buffer: data,
            position: 0,
            chunk_size,
        }
    }
}

impl StreamingIterator for IoBuffer {
    type Item<'a> = &'a [u8] where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.position >= self.buffer.len() {
            return None;
        }

        let end = (self.position + self.chunk_size).min(self.buffer.len());
        let chunk = &self.buffer[self.position..end];
        self.position = end;
        Some(chunk)
    }
}

fn main() {
    // Example 1: Tokenizer - borrowing from internal text buffer
    println!("=== Tokenizer Example ===");
    let text = "hello world foo bar";
    let mut tokenizer = Tokenizer::new(text);

    while let Some(token) = tokenizer.next() {
        println!("Token: {}", token);
    }

    // Example 2: Parser - borrowing key=value pairs from internal buffer
    println!("\n=== Parser Example ===");
    let config = "name=alice,age=30,city=nyc";
    let mut parser = Parser::new(config);

    while let Some((key, value)) = parser.next() {
        println!("Parsed: {} = {}", key, value);
    }

    // Example 3: IO Buffer - yielding chunks borrowing from internal buffer
    println!("\n=== IO Buffer Example ===");
    let data = b"Hello, this is a streaming buffer example".to_vec();
    let mut buffer = IoBuffer::new(data, 10);

    let mut chunk_num = 1;
    while let Some(chunk) = buffer.next() {
        println!("Chunk {}: {:?}", chunk_num, std::str::from_utf8(chunk).unwrap());
        chunk_num += 1;
    }
}
