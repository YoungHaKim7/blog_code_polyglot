use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MyFuture {
    value: Option<String>,
    ptr: *const String, // will point inside self
}

impl Future for MyFuture {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.as_mut().get_unchecked_mut() };

        if this.ptr.is_null() {
            // First poll: set self-reference
            this.ptr = this.value.as_ref().unwrap() as *const String;
            Poll::Pending
        } else {
            // Later poll: use the self-reference
            let s = unsafe { &*this.ptr };
            Poll::Ready(s.to_uppercase())
        }
    }
}

fn make_future(s: &str) -> impl Future<Output = String> + 'static {
    MyFuture {
        value: Some(String::from(s)),
        ptr: std::ptr::null(),
    }
}

// Simple executor that polls twice to demonstrate the self-reference
fn run_twice<F: Future>(mut fut: F) -> F::Output {
    let waker = std::task::Waker::noop();
    let mut cx = Context::from_waker(&waker);

    // First poll - sets self-reference
    let mut fut = unsafe { Pin::new_unchecked(&mut fut) };
    match fut.as_mut().poll(&mut cx) {
        Poll::Pending => {}
        Poll::Ready(v) => return v,
    }

    // Second poll - uses self-reference
    match fut.poll(&mut cx) {
        Poll::Pending => panic!("Should be ready on second poll"),
        Poll::Ready(v) => v,
    }
}

fn main() {
    // Example 1: Basic greeting
    let result1 = run_twice(make_future("hello async"));
    println!("Example 1: {}", result1);
    println!("Example 1 memory address: {:p}", &result1);

    // Example 2: Rust enthusiast
    let result2 = run_twice(make_future("rust is awesome"));
    println!("Example 2: {}", result2);
    println!("Example 2 memory address: {:p}", &result2);

    // Example 3: Pinning concepts
    let result3 = run_twice(make_future("pinning is powerful"));
    println!("Example 3: {}", result3);
    println!("Example 3 memory address: {:p}", &result3);

    // Example 4: Self-reference
    let result4 = run_twice(make_future("self-referential structs"));
    println!("Example 4: {}", result4);
    println!("Example 4 memory address: {:p}", &result4);

    // Example 5: Async programming
    let result5 = run_twice(make_future("async futures in rust"));
    println!("Example 5: {}", result5);
    println!("Example 5 memory address: {:p}", &result5);
}
