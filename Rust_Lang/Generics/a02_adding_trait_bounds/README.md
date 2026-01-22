# Result

```bash
1
2
3
"a"
"b"
"c"
Sum of nums: 15
Sum of floats: 7

```


# ⏺ The code is now fixed and running. Here's what I changed:

  Fixed the original errors:
  - Added .iter() to the debug_items calls (lines 22-23) to convert Vec to an iterator

  Added 2 sum examples:
  - Sum of integers: vec![1, 2, 3, 4, 5] → 15
  - Sum of floats: vec![1.5, 2.5, 3.0] → 7

  The key issue was that Vec<T> doesn't implement Iterator directly. To use it with functions that require Iterator, you need to call:
  - .iter() - for borrowing elements (Iterator<Item = &T>)
  - .into_iter() - for consuming the collection (Iterator<Item = T>)

