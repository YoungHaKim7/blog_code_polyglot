use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

struct MyArcInner<T> {
    count: AtomicUsize,
    data: T,
}

#[derive(Debug)]
struct MyArc<T> {
    ptr: NonNull<MyArcInner<T>>,
}

impl<T> MyArc<T> {
    fn new(data: T) -> Self {
        let boxed = Box::new(MyArcInner {
            count: AtomicUsize::new(1),
            data,
        });

        MyArc {
            ptr: NonNull::new(Box::into_raw(boxed)).unwrap(),
        }
    }

    fn clone(&self) -> Self {
        let inner = unsafe { self.ptr.as_ref() };
        inner.count.fetch_add(1, Ordering::Relaxed);

        MyArc { ptr: self.ptr }
    }

    // Helper method to access the inner data safely
    fn get_data(&self) -> &T {
        let inner = unsafe { self.ptr.as_ref() };
        &inner.data
    }

    // Helper method to get reference count
    fn ref_count(&self) -> usize {
        let inner = unsafe { self.ptr.as_ref() };
        inner.count.load(Ordering::Relaxed)
    }
}

impl<T> Drop for MyArc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };

        if inner.count.fetch_sub(1, Ordering::Release) == 1 {
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

// Safety: MyArc can be sent between threads if T is Send
// because the data is accessed atomically through reference counting
unsafe impl<T: Send> Send for MyArc<T> {}

// Safety: MyArc can be shared between threads if T is Sync
// because all accesses to the inner data are synchronized through atomic operations
unsafe impl<T: Sync> Sync for MyArc<T> {}

fn main() {
    // Example 1: Basic usage - creating and cloning MyArc
    {
        println!("=== Example 1: Basic Usage ===");
        let original = MyArc::new(String::from("Hello, MyArc!"));
        let clone1 = MyArc::clone(&original);
        let clone2 = MyArc::clone(&original);

        // All three point to the same data
        println!("Reference count: {}", original.ref_count());
        println!("Data: {}", original.get_data());
        println!();
    }
    println!("Example 1 completed - all clones dropped\n");

    // Example 2: Using with threads (demonstrates Arc-like behavior)
    {
        println!("=== Example 2: Multi-threaded Usage ===");
        use std::thread;
        use std::time::Duration;

        let shared_data = MyArc::new(vec![1, 2, 3, 4, 5]);
        let mut handles = vec![];

        for i in 0..3 {
            let arc_clone = MyArc::clone(&shared_data);
            handles.push(thread::spawn(move || {
                println!(
                    "Thread {}: {:?}, count: {}",
                    i,
                    arc_clone.get_data(),
                    arc_clone.ref_count()
                );
                thread::sleep(Duration::from_millis(100));
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Main: Final count: {}\n", shared_data.ref_count());
    }

    // Example 3: Custom type with MyArc
    {
        println!("=== Example 3: Custom Type ===");

        #[derive(Debug)]
        struct Counter {
            value: i32,
        }

        let counter = MyArc::new(Counter { value: 0 });
        println!("Initial counter: {:?}", counter.get_data());

        let counter_clone1 = MyArc::clone(&counter);
        let counter_clone2 = MyArc::clone(&counter);
        let counter_clone3 = MyArc::clone(&counter);

        println!("With 4 total references, count: {}", counter.ref_count());

        println!("{counter_clone1:?} {counter_clone2:?} {counter_clone3:?}");
    }
}
