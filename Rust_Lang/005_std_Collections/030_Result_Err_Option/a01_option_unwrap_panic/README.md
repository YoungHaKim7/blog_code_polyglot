# Result

```bash
$ cargo r
 --> src/main.rs:3:9
  |
3 |     let v = x.unwrap(); // 💥 panic
  |         ^ help: if this is intentional, prefix it with an underscore: `_v`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `a01_option_unwrap_panic` (bin "a01_option_unwrap_panic") generated 1 warning (run `cargo fix --bin "a01_option_unwrap_panic" -p a01_option_unwrap_panic` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.51s
     Running `target/debug/a01_option_unwrap_panic`

thread 'main' (50046) panicked at src/main.rs:3:15:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

