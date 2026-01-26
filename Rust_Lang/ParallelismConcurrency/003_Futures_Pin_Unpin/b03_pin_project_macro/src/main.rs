use std::pin::Pin;

use pin_project::pin_project;

#[pin_project]
struct ImportantThing {
    #[pin]
    buffer: String,
    ptr: *const u8, // will point into buffer
}

impl ImportantThing {
    fn new() -> Self {
        ImportantThing {
            buffer: String::from("important data"),
            ptr: std::ptr::null(),
        }
    }

    fn pin_init(self: Pin<&mut Self>) {
        let this = self.project();
        *this.ptr = this.buffer.as_ptr();
    }

    fn get_first_byte(self: Pin<&Self>) -> u8 {
        unsafe { *self.ptr }
    }
}

fn main() {
    // Example 1: Basic pinning and projection usage
    {
        let mut thing = ImportantThing::new();
        let thing_test = ImportantThing::new();
        let mut pinned = Pin::new(&mut thing);
        pinned.as_mut().pin_init();
        println!(
            "Example 1 - First byte: {}",
            pinned.as_ref().get_first_byte()
        );
        println!("thing_test mem address: {:p}", &thing_test);
        println!("pinned mem address: {:p}", &pinned);
    }

    // Example 2: Using pin_init with Box::pin (heap-pinned)
    {
        let mut pinned_thing = Box::pin(ImportantThing::new());
        pinned_thing.as_mut().pin_init();
        println!(
            "Example 2 - First byte from Box::pin: {}",
            pinned_thing.as_ref().get_first_byte()
        );
        println!("pinned_thing mem address: {:p}", &pinned_thing);
    }
}
