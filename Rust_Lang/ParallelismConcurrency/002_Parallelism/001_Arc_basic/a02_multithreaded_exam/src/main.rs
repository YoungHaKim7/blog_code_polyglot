use std::{sync::Arc, thread};

fn main() {
    let shared = Arc::new(vec![10, 20, 30]);

    let mut handles = vec![];

    for i in 0..3 {
        let shared_clone = Arc::clone(&shared);

        let handle = thread::spawn(move || {
            println!("Thread {i}: {:?}", shared_clone);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final strong count = {}", Arc::strong_count(&shared));
}
