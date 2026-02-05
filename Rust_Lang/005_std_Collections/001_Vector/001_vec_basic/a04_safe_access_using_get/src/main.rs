fn main() {
    let v = vec![10, 20, 30];

    match v.get(1) {
        Some(value) => println!("Value: {}", value),
        None => println!("Out of bounds"),
    }
}
