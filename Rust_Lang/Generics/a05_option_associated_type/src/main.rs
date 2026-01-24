use std::ops::Add;

// generic

fn sum_generic<I, T>(iter: I) -> Option<T>
where
    I: Iterator<Item = T>,
    T: Add<Output = T>,
{
    iter.reduce(|a, b| a + b)
}

// associated type
fn sum_asso<I>(iter: I) -> Option<I::Item>
where
    I: Iterator,
    I::Item: Add<Output = I::Item>,
{
    iter.reduce(|a, b| a + b)
}

fn main() {
    let range_int = 1..=10;
    println!("{:?}", sum_asso(range_int));

    let range_int2 = 1..=10;
    println!("{:?}", sum_generic(range_int2));
}
