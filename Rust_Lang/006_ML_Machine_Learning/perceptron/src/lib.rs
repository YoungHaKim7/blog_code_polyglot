/// Returns the weights and bias after performing Perceptron algorithm on the input data points.
/// The Perceptron is a binary classification algorithm that learns a linear separator.
/// Labels should be either -1.0 or 1.0 for the two classes.
pub fn perceptron(
    data_points: Vec<(Vec<f64>, f64)>,
    max_iterations: usize,
    learning_rate: f64,
) -> Option<(Vec<f64>, f64)> {
    if data_points.is_empty() {
        return None;
    }

    let num_features = data_points[0].0.len();
    if num_features == 0 {
        return None;
    }

    let mut weights = vec![0.0; num_features];
    let mut bias = 0.0;

    for _ in 0..max_iterations {
        let mut misclassified = 0;

        for (features, label) in &data_points {
            let prediction = predict(&weights, bias, features);

            if prediction != *label {
                misclassified += 1;

                for (weight, feature) in weights.iter_mut().zip(features.iter()) {
                    *weight += learning_rate * label * feature;
                }
                bias += learning_rate * label;
            }
        }

        if misclassified == 0 {
            break;
        }
    }

    Some((weights, bias))
}

/// Make a prediction using the given weights and bias.
fn predict(weights: &[f64], bias: f64, features: &[f64]) -> f64 {
    let sum = weights
        .iter()
        .zip(features.iter())
        .map(|(w, x)| w * x)
        .sum::<f64>()
        + bias;

    if sum >= 0.0 { 1.0 } else { -1.0 }
}

/// Classify a new data point using the learned weights and bias.
pub fn classify(weights: &[f64], bias: f64, features: &[f64]) -> Option<f64> {
    if weights.is_empty() || features.is_empty() {
        return None;
    }

    if weights.len() != features.len() {
        return None;
    }

    Some(predict(weights, bias, features))
}
