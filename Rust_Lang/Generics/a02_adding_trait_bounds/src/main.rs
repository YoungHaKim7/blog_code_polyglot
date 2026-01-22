use std::ops::Add;

fn sum<I, T>(iter: I) -> Option<T>
where
    I: Iterator<Item = T>,
    T: Add<Output = T>,
{
    iter.reduce(|a, b| a + b)
}

fn debug_items<I, T>(iter: I)
where
    I: Iterator<Item = T>,
    T: std::fmt::Debug,
{
    for item in iter {
        println!("{:?}", item);
    }
}

fn main() {
    debug_items(vec![1, 2, 3].iter());
    debug_items(vec!["a", "b", "c"].iter());

    // Sum examples
    let nums = vec![1, 2, 3, 4, 5];
    if let Some(total) = sum(nums.into_iter()) {
        println!("Sum of nums: {}", total);
    }

    let floats = vec![1.5, 2.5, 3.0];
    if let Some(total) = sum(floats.into_iter()) {
        println!("Sum of floats: {}", total);
    }
}
