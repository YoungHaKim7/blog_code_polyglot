use std::ops::Add;

fn sum<I>(iter: I) -> Option<I::Item>
where
    I: Iterator,
    I::Item: Add<Output = I::Item>,
{
    iter.reduce(|a, b| a + b)
}

fn main() {
    let v = vec![1, 2, 3];
    let result = sum(v.into_iter());

    println!("{:?}", result); // Some(6)

    println!("\n#####\nexample 2\n");
    let v2: Vec<i32> = vec![];
    let result2 = sum(v2.into_iter());

    println!("{:?}", result2); // None

    println!("\n#####\nexample 3\n");
    let v3 = vec![10, 20, 30];

    match sum(v3.into_iter()) {
        Some(total) => println!("sum = {}", total),
        None => println!("empty iterator"),
    }

    println!("\n#####\nexample 4\n");
    let words = vec!["Rust", " ", "Lang"];

    let result = sum(words.into_iter());
    println!("{:?}", result); // Some("Rust Lang")
}
