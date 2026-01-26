use std::pin::Pin;
use std::ptr;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
}

impl Test {
    // Initialize the self-reference only on a pinned instance
    fn init(self: Pin<&mut Self>) {
        unsafe {
            let this = self.get_unchecked_mut();
            this.b = &this.a as *const String;
        }
    }

    fn get_a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn get_b(self: Pin<&Self>) -> &str {
        unsafe { &*self.b }
    }
}

fn main() {
    // Pin the struct on the heap to prevent moves
    let mut test = Box::pin(Test {
        a: String::from("hello"),
        b: ptr::null(),
    });

    // Safely initialize the self-reference
    Test::init(test.as_mut());

    println!("A: {}", test.as_ref().get_a()); // Outputs: A: hello
    println!("B: {}", test.as_ref().get_b()); // Outputs: B: hello

    // Attempting to move out of a pinned box isn't straightforward and is prevented by the type system
    // let test2 = *test;  // This won't compile: cannot move out of a Pin<Box<Test>>
}
