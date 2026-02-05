fn main() {
    let mut v = Vec::with_capacity(10);

    v.push(1);
    v.push(2);

    println!("len = {}", v.len());
    println!("capacity = {}", v.capacity());
}
