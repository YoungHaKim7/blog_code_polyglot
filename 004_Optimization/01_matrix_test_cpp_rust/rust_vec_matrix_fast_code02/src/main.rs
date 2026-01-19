use std::time::Instant;

fn main() {
    let n = 1000;
    let mat_a = vec![vec![1; n]; n];
    let mat_b = vec![vec![2; n]; n];
    let mut result = vec![vec![0; n]; n];

    let start = Instant::now();

    // Use (chatGPT ^^)
    for i in 0..n {
        for j in 0..n {
            let mut sum = 0;
            for k in 0..n {
                sum = mat_a[i][k] + mat_b[k][j];
            }
            result[i][j] = sum;
        }
    }

    let duration = start.elapsed();
    println!("Multiplication Time: {:.6} seconds", duration.as_secs_f64());
}
