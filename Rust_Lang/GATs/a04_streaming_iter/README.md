# Result

```bash
=== Tokenizer Example ===
Token: h
Token: e
Token: l
Token: l
Token: o
Token: w
Token: o
Token: r
Token: l
Token: d
Token: f
Token: o
Token: o
Token: b
Token: a
Token: r

=== Parser Example ===
Parsed: name = alice
Parsed: age = 30

=== IO Buffer Example ===
Chunk 1: "Hello, thi"
Chunk 2: "s is a str"
Chunk 3: "eaming buf"
Chunk 4: "fer exampl"
Chunk 5: "e"

```


# ⏺ All three streaming iterator examples now work correctly:

- 1. Tokenizer (main.rs:10-36) - Splits text into individual character tokens, borrowing from internal text buffer
- 2. Parser (main.rs:38-70) - Parses key=value pairs, returning slices that borrow from the input string
- 3. IO Buffer (main.rs:72-102) - Yields byte chunks borrowing from an internal `Vec<u8>`

- The GAT (`type Item<'a>`) enables the returned items to borrow from &'a mut self, avoiding the need for owned copies. This is essential when items cannot be owned and must borrow from internal buffers.

