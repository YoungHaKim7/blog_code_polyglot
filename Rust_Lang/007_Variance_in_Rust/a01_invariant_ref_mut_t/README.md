# Result

```bash

```

# fix

Done! I've created comprehensive examples demonstrating why `&mut T` is **invariant** in Rust. Here's a summary of the examples:

## Examples Created

| Example | Description |
|---------|-------------|
| **Example 1** | Basic invariant behavior - `&mut &'short i32` cannot coerce to `&mut &'long i32` |
| **Example 2** | Why invariance prevents use-after-free bugs |
| **Example 3** | How covariance would allow type confusion (writing `Cat` into `Dog` variable) |
| **Example 4** | Exact lifetime match required for `&mut T` |
| **Example 5** | Invariance with `RefCell` |
| **Example 6** | Contrast: `&T` IS covariant (works!) |

The key insight: **`&mut T` is invariant** because it allows **both reading AND writing**. If it were covariant:
- Reading could return shorter-lived data (use-after-free)
- Writing could insert wrong types (type confusion)

Run `cargo run` to see the working examples, or uncomment `demonstrate_invariant_basic()` to see the compiler error in action.
