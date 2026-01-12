# Result

```bash
basic list : [10, 20, 30, 40]
arr idx 2 : 30
arr map :[30, 60, 90, 120]
arr squares :[100, 400, 900, 1600]
arr squares :[100, 400, 900, 1600]
arr filter(isEven) :Filter { iter: Iter([10, 20, 30, 40]) }
arr filter(isEven) : [10, 20, 30, 40]
arr filter > 25 :[30, 40]
arr sum :100
arr sum :100

```


# ⏺ Fixed. The changes made:
- c % 0 → *c % 2 == 0 (mod 2 to check even, and *c to dereference the reference)


- Done! The my_sum() method now works as a method on iterators. Here's what I changed:

  main.rs:5-17 - Created a MySumExt trait and implemented it for all iterators that yield i32 values, allowing you to call .my_sum() on any iterator.

  main.rs:52 - Added .copied() because iter() yields &i32 references, not i32 values. The .copied() converts &i32 → i32 so my_sum() can work.

  Output shows the sum working correctly: arr sum :100 (10 + 20 + 30 + 40 = 100)
