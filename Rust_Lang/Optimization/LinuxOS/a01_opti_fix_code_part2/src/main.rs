use std::thread;

fn main() {
    // Thread-local reduction: each thread works on its own local variable
    // No shared memory = no false sharing = no cache line bouncing

    let t1 = thread::spawn(|| {
        let mut local_a = 0u64;
        for _ in 0..50_000_000 {
            local_a += 1;
        }
        local_a
    });

    let t2 = thread::spawn(|| {
        let mut local_b = 0u64;
        for _ in 0..50_000_000 {
            local_b += 1;
        }
        local_b
    });

    let a = t1.join().unwrap();
    let b = t2.join().unwrap();

    // Optional: verify results
    // println!("a: {}, b: {}", a, b);
}
