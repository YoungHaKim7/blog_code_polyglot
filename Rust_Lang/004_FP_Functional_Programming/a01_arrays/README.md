# Result

```bash
❯ cargo r
   Compiling a01_arrays v0.1.0 (/Users/gy-gyoung/my_project/Rust_Lang/9999/2222/blog_code_polyglot/Rust_Lang/004_FP_Functional_Programming/a01_arrays)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/a01_arrays`
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
arr product :240000
arr product :240000
arr fold (i32->u32) :100
arr reduce (i32->u32) :Some(100)
```

# `fold` & `reduce` 활용

⏺ Fixed. The key difference between fold and reduce for type conversion:

|Method|Initial Value|Return Type |Use Case|
|-|-|-|-|
|fold|Required (specifies output type)|Direct value|When you want to control the accumulator type<br />누산기 유형을 제어하고 싶을 때|
|reduce|Not needed|`Option<T>`|When the iterator may be empty, type inferred from elements <br />반복자가 비어 있을 수 있는 경우 요소에서 추론된 유형을 입력합니다|

  Summary of changes:
  - Line 82: fold - Uses *x as u32 to cast each &i32 to u32 before adding
  - Line 86: reduce - First maps i32 to u32, then reduces (returns Option<u32>)


# ⏺ Fixed. The changes made:
- c % 0 → *c % 2 == 0 (mod 2 to check even, and *c to dereference the reference)


- Done! The my_sum() method now works as a method on iterators. Here's what I changed:

  main.rs:5-17 - Created a MySumExt trait and implemented it for all iterators that yield i32 values, allowing you to call .my_sum() on any iterator.

  main.rs:52 - Added .copied() because iter() yields &i32 references, not i32 values. The .copied() converts &i32 → i32 so my_sum() can work.

  Output shows the sum working correctly: arr sum :100 (10 + 20 + 30 + 40 = 100)

- All errors are fixed. The my_product() method now works correctly:

  Summary of fixes:
  1. src/main.rs:7 - Changed trait signature from fn my_product(x: i32) -> i32 to fn my_product(self) -> i32 to match the impl
  2. src/main.rs:24 - Changed initial value from 0 to 1 (product must start at 1, otherwise result is always 0)
  3. src/main.rs:42 - Renamed my_squares variable to squares_arr to avoid shadowing the my_squares function
  4. src/main.rs:75 - Added println! for product output

  Result:
  arr product :240000
  10 × 20 × 30 × 40 = 240,000
