// return Option<T>
fn main() {
    let mut v = vec![1, 2, 3];

    let last = v.pop();
    println!("{:?}", last);
    println!("{:?}", v);
}
