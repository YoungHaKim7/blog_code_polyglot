fn main() {
    let mut v: Vec<i32> = Vec::new();
    // 동일하다 Vec::new() = vec![];

    v.push(10);
    v.push(20);
    v.push(30);

    println!("{:?}", v);
}
