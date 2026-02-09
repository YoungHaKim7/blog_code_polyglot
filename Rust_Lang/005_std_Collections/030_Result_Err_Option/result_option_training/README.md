# Result

```bash
$ cargo t --lib

running 6 tests
test ex1_basic_ok_or ... ok
test ex4_chaining_question_mark ... ok
test ex2_parse_with_question_mark ... ok
test ex6_result_map ... ok
test ex7_and_then ... ok
test ex8_collect_results ... ok
```
# AI

All tests pass cleanly now. Here's a summary of the **10 examples**:

| # | Pattern | Key Function |
|---|---------|--------------|
| 1 | `Option` → `Result` | `ok_or()`, `ok_or_else()` |
| 2 | Parse with `?` | `s.parse()?` propagates errors |
| 3 | Full error enum | `Display` + `std::error::Error` traits |
| 4 | Chaining `?` | Multiple fallible operations |
| 5 | File I/O | `map_err()` converts error types |
| 6 | `map()` | Transform `Ok` values |
| 7 | `and_then()` | Chain `Result`-producing functions |
| 8 | `collect()` | `Vec<Result<T>>` → `Result<Vec<T>>` |
| 9 | Context enrichment | Add info to errors with `map_err` |
| 10 | `From` trait | Auto error conversion with `?` |

**Key takeaways:**

```rust
// ✅ OK - unwrap_or() returns a default value
result.unwrap_or(default_value)

// ❌ BAD - unwrap() panics on Err
result.unwrap()

// ✅ OK - ? propagates errors
fn foo() -> Result<T, E> {
    let x = may_fail()?;
    Ok(x)
}
```

Run tests with: `cargo test --lib`
