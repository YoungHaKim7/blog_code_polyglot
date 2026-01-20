// https://doc.rust-lang.org/stable/std/ops/struct.RangeInclusive.html

fn main() {
    let range_no: Vec<_> = (1..=10).collect();

    let range_no_9: Vec<_> = (1..10).collect();
    println!("{range_no:?}");
    println!("{range_no_9:?}");
}
