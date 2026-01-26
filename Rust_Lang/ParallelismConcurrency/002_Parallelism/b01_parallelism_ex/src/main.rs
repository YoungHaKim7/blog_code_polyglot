use std::thread;

fn main() {
    let t1 = thread::spawn(|| {
        for i in 0..5 {
            println!("Thread A {i}");
        }
    });

    let t2 = thread::spawn(|| {
        for i in 0..5 {
            println!("Thread B {i}");
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
