use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Reads a file into a vector of lines.
pub fn read_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect()
}

/// Prints the differences between two files using LCS.
pub fn print_differences(file_a: &[String], file_b: &[String]) {
    let dp = crate::lcs::lcs_table(file_a, file_b);
    let lcs = crate::lcs::reconstruct_elements(&dp, file_a, file_b);

    let mut line_a = 0;
    let mut line_b = 0;

    for line in &lcs {
        while file_a[line_a] != *line {
            println!("- {}", file_a[line_a]);
            line_a += 1;
        }
        while file_b[line_b] != *line {
            println!("+ {}", file_b[line_b]);
            line_b += 1;
        }

        assert_eq!(file_a[line_a], file_b[line_b]);
        println!("  {}", file_a[line_a]);
        line_a += 1;
        line_b += 1;
    }

    while line_a < file_a.len() {
        println!("- {}", file_a[line_a]);
        line_a += 1;
    }

    while line_b < file_b.len() {
        println!("+ {}", file_b[line_b]);
        line_b += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_differences() {
        let a = vec!["line1".to_string(), "line2".to_string(), "line3".to_string()];
        let b = vec!["line1".to_string(), "modified".to_string(), "line3".to_string()];
        print_differences(&a, &b);
    }
}
