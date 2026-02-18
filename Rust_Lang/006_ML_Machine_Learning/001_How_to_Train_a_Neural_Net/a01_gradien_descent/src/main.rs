fn main() {
    let mut theta: f64 = 0.0; // initial guess
    let learning_rate: f64 = 0.1; // η
    let iterations = 50;

    for i in 0..iterations {
        // derivative of (theta - 3)^2
        let gradient = 2.0 * (theta - 3.0);

        // update rule
        theta = theta - learning_rate * gradient;

        println!(
            "iter {:02} | theta = {:.6} | cost = {:.6}",
            i,
            theta,
            (theta - 3.0).powi(2)
        );
    }

    println!("\nFinal theta ≈ {}", theta);
}
