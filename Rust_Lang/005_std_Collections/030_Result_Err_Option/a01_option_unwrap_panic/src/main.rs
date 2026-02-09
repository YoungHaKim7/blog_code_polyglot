fn main() {
    let x: Option<i32> = None;
    let v = x.unwrap(); // 💥 panic
}
