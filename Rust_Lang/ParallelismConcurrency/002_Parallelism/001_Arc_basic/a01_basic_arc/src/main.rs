use std::sync::Arc;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);

    let a = Arc::clone(&data);
    let b = Arc::clone(&data);

    println!("data: {:?}", data);
    println!("a: {:?}", a);
    println!("b: {:?}", b);

    println!("strong count = {}", Arc::strong_count(&data));
}
