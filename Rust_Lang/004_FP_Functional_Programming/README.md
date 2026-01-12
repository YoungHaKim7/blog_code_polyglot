# array.map

- https://doc.rust-lang.org/stable/std/primitive.array.html#method.map

# `fold` & `reduce` 활용

⏺ Fixed. The key difference between fold and reduce for type conversion:

|Method|Initial Value|Return Type |Use Case|
|-|-|-|-|
|fold|Required (specifies output type)|Direct value|When you want to control the accumulator type<br />누산기 유형을 제어하고 싶을 때|
|reduce|Not needed|`Option<T>`|When the iterator may be empty, type inferred from elements <br />반복자가 비어 있을 수 있는 경우 요소에서 추론된 유형을 입력합니다|

  Summary of changes:
  - Line 82: fold - Uses *x as u32 to cast each &i32 to u32 before adding
  - Line 86: reduce - First maps i32 to u32, then reduces (returns Option<u32>)

```rs
// Fold : Reduce to single value (i32 -> u32 conversion)
let my_fold_arr: u32 = my_arr
    .iter()
    .fold(0u32, |acc, x| acc + *x as u32);

println!("arr fold (i32->u32) :{my_fold_arr}");

// Reduce : Alternative without initial value (returns Option)
let my_reduce_arr: Option<u32> = my_arr
    .iter()
    .map(|x| *x as u32)
    .reduce(|acc, x| acc + x);
println!("arr reduce (i32->u32) :{my_reduce_arr:?}");
```

- result

```bash
arr fold (i32->u32) :100
arr reduce (i32->u32) :Some(100)
```
