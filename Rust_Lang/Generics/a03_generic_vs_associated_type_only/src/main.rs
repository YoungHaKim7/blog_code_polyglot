use std::ops::Add;

fn generics_sum<I, T>(iter: I) -> Option<T>
where
    I: Iterator<Item = T>,
    T: Add<Output = T>,
{
    iter.reduce(|a, b| a + b)
}

fn associated_sum<I>(iter: I) -> Option<I::Item>
where
    I: Iterator,
    I::Item: Add<Output = I::Item>,
{
    iter.reduce(|a, b| a + b)
}

fn main() {
    let generics_val = 1..=10;
    println!("{:?}", generics_sum(generics_val));

    let associated_val = 1..=10;
    println!("{:?}", associated_sum(associated_val));
}
