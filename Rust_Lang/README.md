# rust

# Rust를 깊게 들어가는 외국 블로그
- https://aturon.github.io/

# `const fn` 에 대하여(Rust)
- https://medium.com/intro-to-rust/const-fn-5d1a15fd4e29

# 연습 `use std::marker::PhantomData`

```rs
use std::marker::PhantomData;

struct Inv<'a>(PhantomData<*mut &'a ()>);

// This type is only sometimes `PartialEq`.
impl PartialEq for Inv<'static> {
    fn eq(&self, _: &Inv<'static>) -> bool {
        true
    }
}

impl<'a> Inv<'a> {
    // The value `None` makes this have structural equality for any type `Self`.
    const NOT_STATIC: Option<Self> = None;
}

fn foo<'a>(x: Option<Inv<'a>>) {
    match x {
        Inv::<'a>::NOT_STATIC => (),
        Some(_) => panic!()
    }
    
    // Enabling the next line confirms that the type does
    // indeed not implement `PartialEq`.
    //x == Inv::<'a>::NOT_STATIC;
}

fn main() {
    foo(None)
}
```

- https://github.com/rust-lang/rust/issues/121007
