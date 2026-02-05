fn main() {
    let mut v = vec![1, 2, 3];

    for x in &mut v {
        *x *= 10;
    }

    println!("{:?}", v);
}
