use std::ops::Add;

fn debug_items_generic<I, T>(iter: I)
where
    I: Iterator<Item = T>,
    T: std::fmt::Debug,
{
    for item in iter {
        println!("{:?}", item);
    }
}

fn debug_items_asso<I>(iter: I)
where
    I: Iterator,
    I::Item: std::fmt::Debug,
{
    for item in iter {
        println!("{:?}", item);
    }
}

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

    let range_int3 = 1..=10;
    println!("{:?}", debug_items_generic(range_int3));
    let range_int33 = 1..=10;
    dbg!(debug_items_generic(range_int33));

    let range_int4 = 1..=10;
    println!("{:?}", debug_items_asso(range_int4));
}
