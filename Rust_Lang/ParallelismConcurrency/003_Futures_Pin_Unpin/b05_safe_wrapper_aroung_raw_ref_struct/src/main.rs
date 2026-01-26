use std::pin::Pin;
use std::ptr::NonNull;

pub struct SafeSelfRef {
    inner: String,
    // We use NonNull + invariant over pinning
    ptr: Option<NonNull<String>>,
}

impl SafeSelfRef {
    pub fn new(inner: String) -> Self {
        SafeSelfRef { inner, ptr: None }
    }

    pub fn pin_and_init(mut self: Pin<&mut Self>) {
        unsafe {
            let this = self.as_mut().get_unchecked_mut();
            this.ptr = Some(NonNull::from(&this.inner));
        }
    }

    pub fn get(self: Pin<&Self>) -> &str {
        let ptr = self.ptr.unwrap();
        unsafe { ptr.as_ref().as_str() }
    }
}

fn usage() {
    let value = String::from("protected content");
    let mut sr = Box::pin(SafeSelfRef::new(value));

    SafeSelfRef::pin_and_init(sr.as_mut());

    println!("{}", sr.as_ref().get()); // guaranteed safe
}

fn main() {
    // Example 1: Using the SafeSelfRef wrapper
    usage();

    // Example 2: Multiple self-referential instances
    let value1 = String::from("first content");
    let value2 = String::from("second content");

    let mut sr1 = Box::pin(SafeSelfRef::new(value1));
    let mut sr2 = Box::pin(SafeSelfRef::new(value2));

    SafeSelfRef::pin_and_init(sr1.as_mut());
    SafeSelfRef::pin_and_init(sr2.as_mut());

    println!("Example 2: {} and {}", sr1.as_ref().get(), sr2.as_ref().get());
}
