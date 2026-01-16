# Result

```bash
Item: 1
Item: 2
Item: 3
Item: 4
Item: 5

Original data still valid: [1, 2, 3, 4, 5]

```


# ⏺ Done! Here's what I added to demonstrate the GAT pattern:

- Key points:

- 1. `SliceIter<'slice, T>` - Holds a reference to a slice with lifetime `'slice`
- 2. `type Item<'a> = &'a T` - The GAT allows Item to borrow from `self`:
  - The lifetime `'a` comes from `&'a mut self` in `next()`
  - Each returned `&'a T` is tied to how long the borrow of `self` lasts
- 3. Zero-copy - No cloning, no allocation. The iterator returns references directly into the original data.

- Without GATs, you couldn't express this in a trait. The standard Iterator trait's Item is:
```rs
type Item;  // No lifetime parameter possible!
```

- This means you can only return owned values or have awkward lifetime constraints. GATs unlock borrowing patterns that were previously impossible in trait APIs.
