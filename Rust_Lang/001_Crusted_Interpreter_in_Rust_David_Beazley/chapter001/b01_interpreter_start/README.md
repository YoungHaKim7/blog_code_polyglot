# Result

```bash
Hello, lox!
== test chunk ==
0000, OP_CONSTANT(0)
0001, OP_RETURN

```

# `cargo expand`

```bash
$ cargo expand vm::VM

pub struct VM<'a> {
    chunk: Option<&'a Chunk>,
    ip: usize,
    stack: Vec<Value>,
    trace_execution: bool,
    trace_stack: bool,
}
```
