fn push_front_str(src: &str, mut dst: String) -> String {
    for c in src.chars() {
        dst.push(c)
    }
    dst
}

fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");

    let upp = String::from("test\t");
    let res = push_front_str(&arg, upp);

    println!("upp = {}", res);
    println!("arg = {}", arg);
}
