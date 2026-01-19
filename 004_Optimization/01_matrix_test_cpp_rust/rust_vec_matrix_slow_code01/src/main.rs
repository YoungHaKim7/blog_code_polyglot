use std::time::Instant;

fn main() {
    let n = 1_000;
    let mat_a = vec![vec![1; n]; n];
    let mat_b = vec![vec![1; n]; n];
    let mut result = vec![vec![0; n]; n];

    let start = Instant::now();

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                result[i][j] += mat_a[i][k] * mat_b[k][j];
            }
        }
    }

    let duration = start.elapsed();
    println!(
        "Multiplication time : {:.6} seconds",
        duration.as_secs_f64()
    );
}
