use std::time::Duration;

#[inline(never)]
fn hot_loop(data: &mut [u64]) {
    for _ in 0..50 {
        for x in data.iter_mut() {
            *x = x.wrapping_mul(1664525).wrapping_add(1013904223);
        }
    }
}

fn main() {
    let mut data = vec![1u64; 20_000_000];

    // warm-up (important)
    hot_loop(&mut data);

    // give profiler time
    std::thread::sleep(Duration::from_secs(1));

    hot_loop(&mut data);
}
