use std::pin::Pin;

struct SelfRef {
    data: String,
    ptr: *const String, // points to our own `data`
}

impl SelfRef {
    fn new() -> Self {
        SelfRef {
            data: String::from("hello"),
            ptr: std::ptr::null(),
        }
    }

    // Call this **after** pinning
    fn init(self: Pin<&mut Self>) {
        let this = unsafe { self.get_unchecked_mut() };
        this.ptr = &this.data;
    }

    fn get_data_via_ptr(self: Pin<&Self>) -> &str {
        unsafe { &*self.ptr }
    }
}

fn main() {
    let mut sr = Box::pin(SelfRef::new());
    SelfRef::init(sr.as_mut());

    println!("{}", sr.as_ref().get_data_via_ptr()); // safe
    // sr cannot be moved anymore → pointer stays valid
}
