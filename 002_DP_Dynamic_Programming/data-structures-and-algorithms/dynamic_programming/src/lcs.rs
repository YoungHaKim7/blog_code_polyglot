/// Computes the LCS dynamic programming table for two sequences.
pub fn lcs_table<T: PartialEq>(a: &[T], b: &[T]) -> Vec<Vec<usize>> {
    let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp
}

/// Returns the length of the longest common subsequence.
pub fn longest_common_subsequence<T: PartialEq>(a: &[T], b: &[T]) -> usize {
    let dp = lcs_table(a, b);
    dp[a.len()][b.len()]
}

/// Reconstructs the LCS elements from the DP table.
pub fn reconstruct_elements<T: PartialEq + Clone>(
    dp: &[Vec<usize>],
    a: &[T],
    b: &[T],
) -> Vec<T> {
    let mut elements = Vec::new();
    let mut n = a.len();
    let mut m = b.len();
    while n > 0 && m > 0 {
        if a[n - 1] == b[m - 1] {
            elements.push(a[n - 1].clone());
            n -= 1;
            m -= 1;
        } else if dp[n - 1][m] > dp[n][m - 1] {
            n -= 1;
        } else {
            m -= 1;
        }
    }
    elements.reverse();
    elements
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcs_length() {
        let a = vec![1, 4, 5, 6, 9, 10, 11];
        let b = vec![6, 4, 5, 9, 11];
        assert_eq!(longest_common_subsequence(&a, &b), 4);
    }

    #[test]
    fn test_lcs_reconstruct() {
        let a = vec![1, 4, 5, 6, 9, 10, 11];
        let b = vec![6, 4, 5, 9, 11];
        let dp = lcs_table(&a, &b);
        let lcs = reconstruct_elements(&dp, &a, &b);
        assert_eq!(lcs, vec![4, 5, 9, 11]);
    }
}
