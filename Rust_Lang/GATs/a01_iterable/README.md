# Result

```bash
1
2
3

```


# ⏺ Fixed. The example demonstrates the key GATs concept:
                                                                                                 Pre-GATs limitation (commented out in src/main.rs:3-6):
- type `Item<'a>` — associated types couldn't have lifetime parameters
- This prevented expressing "the iterator's item type borrows from &self"


- With GATs (working code in src/main.rs:9-12):                                                  trait IterableGAT {
```rs
    type Item<'a> where Self: 'a;  // ✅ Now legal!
    fn iter<'a>(&'a self) -> Self::Item<'a>;
}
```

- This enables zero-copy borrowing APIs where the returned iterator's lifetime is tied to self, not some arbitrary lifetime. The pattern is essential for:
  - Zero-copy iterators (`std::slice::Iter<'a, T>`)
  - View types into collections
  - Streaming abstractions with borrowing
