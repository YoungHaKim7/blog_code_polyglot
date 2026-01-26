# Result

```bash
Example 1: HELLO ASYNC
Example 1 memory address: 0x16d4dd848

Example 2: RUST IS AWESOME
Example 2 memory address: 0x16d4dd8d8

Example 3: PINNING IS POWERFUL
Example 3 memory address: 0x16d4dd968

Example 4: SELF-REFERENTIAL STRUCTS
Example 4 memory address: 0x16d4dd9f8

Example 5: ASYNC FUTURES IN RUST
Example 5 memory address: 0x16d4dda88

```


# Fixed.
- The issue was on line 18 - as_str() returns &str, and taking another reference with & gave &&str. The fix uses this.value.as_ref().unwrap() as
   *const String to directly get the correct pointer type.

  Changes made:
  1. Fixed the cast (src/main.rs:18): Changed from &this.value.as_ref().unwrap().as_str() as *const _ to this.value.as_ref().unwrap() as *const String
  2. Added parameter to make_future(): Now accepts &str input
  3. Added run_twice() executor: Simple executor that polls twice to demonstrate the self-reference pattern
  4. Added 5 examples in main():
    - "hello async" → HELLO ASYNC
    - "rust is awesome" → RUST IS AWESOME
    - "pinning is powerful" → PINNING IS POWERFUL
    - "self-referential structs" → SELF-REFERENTIAL STRUCTS
    - "async futures in rust" → ASYNC FUTURES IN RUST
