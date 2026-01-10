use dsa_dynamic_programming::lcs;

fn print_matrix(dp: &[Vec<usize>]) {
    for row in dp {
        for elem in row {
            print!("{} ", elem);
        }
        println!();
    }
}

fn main() {
    let a = vec![1, 4, 5, 6, 9, 10, 11];
    let b = vec![6, 4, 5, 9, 11];
    let dp = lcs::lcs_table(&b, &a);
    print_matrix(&dp);
}
