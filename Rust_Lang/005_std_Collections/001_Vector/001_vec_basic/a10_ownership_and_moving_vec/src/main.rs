fn consume(v: Vec<i32>) {
    println!("{:?}", v);
}

fn main() {
    let v = vec![1, 2, 3];

    consume(v);
    println!("{:?}", v); // ❌ compile error
}
