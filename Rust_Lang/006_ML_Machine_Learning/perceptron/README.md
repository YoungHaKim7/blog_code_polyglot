# Result

```bash
$ cargo nextest run
────────────
    Starting 11 tests across 2 binaries
        PASS [   0.027s] perceptron::perceptron_test test_perceptron_different_learning_rates
        PASS [   0.027s] perceptron::perceptron_test test_perceptron_single_feature
        PASS [   0.028s] perceptron::perceptron_test test_classify_empty_weights
        PASS [   0.028s] perceptron::perceptron_test test_classify_empty_features
        PASS [   0.029s] perceptron::perceptron_test test_perceptron_empty_features
        PASS [   0.029s] perceptron::perceptron_test test_classify_mismatched_dimensions
        PASS [   0.029s] perceptron::perceptron_test test_perceptron_linearly_separable
        PASS [   0.029s] perceptron::perceptron_test test_perceptron_empty_data
        PASS [   0.015s] perceptron::perceptron_test test_perceptron_with_bias
        PASS [   0.014s] perceptron::perceptron_test test_perceptron_xor_like
        PASS [   0.014s] perceptron::perceptron_test test_perceptron_zero_iterations
────────────

```

