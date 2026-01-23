use std::iter::Sum;

fn total<I, T>(iter: I) -> T
where
    I: Iterator,
    T: Sum<I::Item>,
{
    iter.sum()
}

fn main() {
    let v = vec![1u8, 2, 3];
    let x: u64 = total(v.into_iter().map(|x| x as u64));
    // println!("{:?}", v);
    println!("{}", x);
}
