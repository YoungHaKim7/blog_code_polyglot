use std::thread;

struct Counters {
    a: u64,
    b: u64,
}

fn main() {
    let mut c = Counters { a: 0, b: 0 };
    let p = &mut c as *mut Counters as usize;

    let t1 = thread::spawn(move || {
        let p = p as *mut Counters;
        for _ in 0..50_000_000 {
            unsafe {
                (*p).a += 1;
            }
        }
    });

    let t2 = thread::spawn(move || {
        let p = p as *mut Counters;
        for _ in 0..50_000_000 {
            unsafe {
                (*p).b += 1;
            }
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
