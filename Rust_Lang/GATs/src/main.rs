use std::fmt::Debug;

fn first_generic<I, T>(iter: I) -> Option<T>
where
    I: Iterator<Item = T>,
{
    iter.into_iter().next()
}

// After (associated type only)

fn first_asso<I>(iter: I) -> Option<I::Item>
where
    I: Iterator,
{
    iter.into_iter().next()
}

fn first_asso_ver1<I>(iter: I) -> Option<I::Item>
where
    I: Iterator<Item = i32>, // Constrain Item to be i32
{
    iter.into_iter().next()
}

// Option 2: Bound the associated type by a trait
fn first_asso_ver2<I>(iter: I) -> Option<I::Item>
where
    I: Iterator,
    I::Item: Debug, // I::Item must implement Debug
{
    iter.into_iter().next()
}

// Option 3: Use a separate type parameter with the associated type
fn first_asso_ver3<I, T>(iter: I) -> Option<T>
where
    I: Iterator<Item = T>, // Connect I::Item to T
    T: Debug,              // Then bound T
{
    iter.into_iter().next()
}

fn main() {
    let generic_test = 1..=10;
    println!("{:?}", first_generic(generic_test));

    let mut asso_test = 1..=10;
    while let Some(item) = first_asso(asso_test.by_ref()) {
        println!("{:?}", item);
    }
}
