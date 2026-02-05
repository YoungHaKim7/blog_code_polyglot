# Result

```bash
error[E0382]: borrow of moved value: `v`
 --> src/main.rs:9:22
  |
6 |     let v = vec![1, 2, 3];
  |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
7 |
8 |     consume(v);
  |             - value moved here
9 |     println!("{:?}", v); // ❌ compile error
  |                      ^ value borrowed here after move
  |
note: consider changing this parameter type in function `consume` to borrow instead if owning the value isn't necessary
 --> src/main.rs:1:15
  |
1 | fn consume(v: Vec<i32>) {
  |    -------    ^^^^^^^^ this parameter takes ownership of the value
  |    |
  |    in this function
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run wi
th -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
8 |     consume(v.clone());
  |              ++++++++

```

