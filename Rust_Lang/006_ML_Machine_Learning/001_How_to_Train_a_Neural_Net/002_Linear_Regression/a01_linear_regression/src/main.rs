fn main() {
    // Training data (y = 2x + 1)
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![3.0, 5.0, 7.0, 9.0, 11.0];

    let n = x.len() as f64;

    let mut w: f64 = 0.0; // weight
    let mut b: f64 = 0.0; // bias

    let learning_rate = 0.01;
    let iterations = 1000;

    for iter in 0..iterations {
        let mut dw = 0.0;
        let mut db = 0.0;
        let mut loss = 0.0;

        for i in 0..x.len() {
            let y_pred = w * x[i] + b;
            let error = y_pred - y[i];

            loss += error * error;

            dw += error * x[i];
            db += error;
        }

        // compute gradients
        dw = (2.0 / n) * dw;
        db = (2.0 / n) * db;
        loss /= n;

        // update parameters
        w -= learning_rate * dw;
        b -= learning_rate * db;

        if iter % 100 == 0 {
            println!("iter {:4} | w={:.4} b={:.4} loss={:.6}", iter, w, b, loss);
        }
    }

    println!("\nFinal Model: y = {:.4}x + {:.4}", w, b);
}
