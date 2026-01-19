use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let v: Vec<i64> = (1..=1_000_000).collect();

    let sum: i64 = v.par_iter().map(|x| *x * 2).sum();

    println!("{sum}");
}
