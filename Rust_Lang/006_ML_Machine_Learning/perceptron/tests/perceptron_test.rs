use perceptron::*;

#[test]
fn test_perceptron_linearly_separable() {
    let data = vec![
        (vec![1.0, 1.0], 1.0),
        (vec![2.0, 2.0], 1.0),
        (vec![3.0, 3.0], 1.0),
        (vec![-1.0, -1.0], -1.0),
        (vec![-2.0, -2.0], -1.0),
        (vec![-3.0, -3.0], -1.0),
    ];

    let result = perceptron(data, 100, 0.1);
    assert!(result.is_some());

    let (weights, bias) = result.unwrap();

    let prediction1 = classify(&weights, bias, &[2.5, 2.5]);
    assert_eq!(prediction1, Some(1.0));

    let prediction2 = classify(&weights, bias, &[-2.5, -2.5]);
    assert_eq!(prediction2, Some(-1.0));
}

#[test]
fn test_perceptron_xor_like() {
    let data = vec![
        (vec![0.0, 0.0], -1.0),
        (vec![1.0, 1.0], 1.0),
        (vec![0.0, 1.0], -1.0),
        (vec![1.0, 0.0], -1.0),
    ];

    let result = perceptron(data, 100, 0.1);
    assert!(result.is_some());

    let (weights, _bias) = result.unwrap();
    assert_eq!(weights.len(), 2);
}

#[test]
fn test_perceptron_single_feature() {
    let data = vec![
        (vec![1.0], 1.0),
        (vec![2.0], 1.0),
        (vec![3.0], 1.0),
        (vec![-1.0], -1.0),
        (vec![-2.0], -1.0),
        (vec![-3.0], -1.0),
    ];

    let result = perceptron(data, 100, 0.1);
    assert!(result.is_some());

    let (weights, bias) = result.unwrap();
    assert_eq!(weights.len(), 1);

    let prediction1 = classify(&weights, bias, &[5.0]);
    assert_eq!(prediction1, Some(1.0));

    let prediction2 = classify(&weights, bias, &[-5.0]);
    assert_eq!(prediction2, Some(-1.0));
}

#[test]
fn test_perceptron_empty_data() {
    let result = perceptron(vec![], 100, 0.1);
    assert_eq!(result, None);
}

#[test]
fn test_perceptron_empty_features() {
    let data = vec![(vec![], 1.0), (vec![], -1.0)];
    let result = perceptron(data, 100, 0.1);
    assert_eq!(result, None);
}

#[test]
fn test_perceptron_zero_iterations() {
    let data = vec![(vec![1.0, 1.0], 1.0), (vec![-1.0, -1.0], -1.0)];

    let result = perceptron(data, 0, 0.1);
    assert!(result.is_some());

    let (weights, bias) = result.unwrap();
    assert_eq!(weights, vec![0.0, 0.0]);
    assert_eq!(bias, 0.0);
}

#[test]
fn test_classify_empty_weights() {
    let result = classify(&[], 0.0, &[1.0, 2.0]);
    assert_eq!(result, None);
}

#[test]
fn test_classify_empty_features() {
    let result = classify(&[1.0, 2.0], 0.0, &[]);
    assert_eq!(result, None);
}

#[test]
fn test_classify_mismatched_dimensions() {
    let result = classify(&[1.0, 2.0], 0.0, &[1.0]);
    assert_eq!(result, None);
}

#[test]
fn test_perceptron_different_learning_rates() {
    let data = vec![
        (vec![1.0, 1.0], 1.0),
        (vec![2.0, 2.0], 1.0),
        (vec![-1.0, -1.0], -1.0),
        (vec![-2.0, -2.0], -1.0),
    ];

    let result1 = perceptron(data.clone(), 100, 0.01);
    let result2 = perceptron(data, 100, 1.0);

    assert!(result1.is_some());
    assert!(result2.is_some());

    let (weights1, bias1) = result1.unwrap();
    let (weights2, bias2) = result2.unwrap();

    let prediction1 = classify(&weights1, bias1, &[3.0, 3.0]);
    let prediction2 = classify(&weights2, bias2, &[3.0, 3.0]);

    assert_eq!(prediction1, Some(1.0));
    assert_eq!(prediction2, Some(1.0));
}

#[test]
fn test_perceptron_with_bias() {
    let data = vec![
        (vec![1.0], 1.0),
        (vec![2.0], 1.0),
        (vec![10.0], 1.0),
        (vec![0.0], -1.0),
        (vec![-1.0], -1.0),
        (vec![-10.0], -1.0),
    ];

    let result = perceptron(data, 100, 0.1);
    assert!(result.is_some());

    let (weights, bias) = result.unwrap();

    let prediction_positive = classify(&weights, bias, &[5.0]);
    let prediction_negative = classify(&weights, bias, &[-5.0]);

    assert_eq!(prediction_positive, Some(1.0));
    assert_eq!(prediction_negative, Some(-1.0));
}
